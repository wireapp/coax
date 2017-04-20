use std;
use std::ffi::CString;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Sender};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use chashmap::CHashMap;
use channel::{Channel, Message, TextMessage, Image};
use chrono::{DateTime, Local, UTC};
use coax_actor::{self, Actor, Pkg, Delivery};
use coax_actor::actor::{Offline, Online};
use coax_actor::config;
use coax_api::conv::ConvType;
use coax_api::message::send;
use coax_api::types::{Label, Name, Email, Password, UserId, ConvId, random_uuid};
use coax_api::user::{self, AssetKey, ConnectStatus};
use coax_api_proto::{Builder as MsgBuilder, GenericMessage};
use coax_client::error::{Error as ClientError};
use coax_data::{self, User, Conversation, Connection, MessageData, MessageStatus, ConvStatus};
use coax_data::{AssetStatus, AssetType};
use coax_data::db::{PagingState, C};
use coax_data::profiles::ProfileDb;
use coax_net::http::tls::{self, Tls};
use contact::{Contact, Contacts};
use error::Error;
use ffi;
use futures::Future;
use futures::future;
use futures_cpupool::{self as pool, CpuPool};
use gdk::prelude::ContextExt;
use gdk_pixbuf::{InterpType, Pixbuf};
use gio::{self, MenuModel, SimpleAction};
use glib_sys;
use gtk::prelude::*;
use gtk::{self, Builder, Button, MenuButton, HeaderBar, Window};
use gtk::{MessageDialog, MessageType, ButtonsType, Orientation};
use gtk_sys::GTK_STYLE_PROVIDER_PRIORITY_USER;
use notify_rust::{Notification, NotificationHint};
use poll::Loop;
use profile::{self, ProfileView};
use res;
use signals::Signal;
use slog::Logger;

#[derive(Clone)]
pub struct Coax {
    log:        Logger,
    config:     config::Main,
    tls:        Arc<Tls>,
    pool_on:    CpuPool, // sending & receiving threads (online)
    pool_off:   CpuPool, // threads acting on local state (offline)
    futures:    Sender<Box<Future<Item=(), Error=()>>>,
    profiles:   Arc<Mutex<ProfileDb>>,
    builder:    gtk::Builder,
    header:     gtk::Builder,
    info:       gtk::Label,
    revealer:   gtk::Revealer,
    mainview:   gtk::Grid,
    convlist:   gtk::ListBox,
    send_btn:   gtk::Button,
    me_box:     gtk::Popover,
    timezone:   Local,
    channels:   Rc<CHashMap<ConvId, Channel>>,
    contacts:   Rc<Contacts>,
    resources:  Rc<res::Resources>,
    sig_online: Rc<Signal<'static, bool, ()>>

}

struct State {
    me:        User<'static>,
    actor_off: Mutex<Actor<Offline>>,
    actor_on:  Mutex<Option<Actor<Online>>>,
    inbox:     Mutex<Option<JoinHandle<()>>>,
    is_sync:   AtomicBool,
    is_online: AtomicBool,
    ch_state:  Mutex<Option<PagingState<C>>>
}

macro_rules! from_some {
    ($e:expr) => {{
        match $e {
            Some(x) => x,
            None    => return ()
        }
    }}
}

impl Coax {
    pub fn new(g: &Logger, cfg: config::Main) -> Result<gtk::Application, Box<std::error::Error>> {
        let app =
            if let Ok(a) = gtk::Application::new(Some("com.wire.coax"), gio::APPLICATION_NON_UNIQUE) {
                a
            } else {
                return Err("failed to create gtk application".into())
            };
        let app_name = CString::new("coax").unwrap();
        unsafe {
            glib_sys::g_set_prgname(app_name.as_ptr())
        }
        let log      = g.new(o!("context" => "Ui"));
        let tls      = Arc::new(tls::context()?);
        let pdb      = profile::open_profile_db(&log, &cfg)?;
        let builder  = Builder::new_from_string(include_str!("gtk/main.ui"));
        let info     = builder.get_object("info-label").unwrap();
        let sendbtn  = builder.get_object("send-button").unwrap();
        let mainview = builder.get_object("mainview").unwrap();
        let header   = Builder::new_from_string(include_str!("gtk/header.ui"));
        let popover  = header.get_object("profile-popover").unwrap();

        let css = gtk::CssProvider::new();
        css.load_from_data(include_str!("gtk/style.css"))?;

        let infobar: gtk::InfoBar = builder.get_object("info-bar").unwrap();
        let revealer: gtk::Revealer = builder.get_object("info-revealer").unwrap();

        infobar.connect_response(with!(revealer => move |_, sc| {
            if sc == gtk::ResponseType::Close.into() {
                revealer.set_reveal_child(false)
            }
        }));

        let convlist = builder.get_object("conversation-list").unwrap();
        ffi::set_sort_by_time(&convlist);

        let (tx, rx) = channel();
        Loop::new(rx).start();

        let contacts = Rc::new(Contacts::new());

        let sig_online = Signal::new();
        sig_online.connect(with!(contacts => move |status| {
            contacts.set_refresh_enabled(*status)
        }));

        let coax = Coax {
            log:        log,
            config:     cfg.clone(),
            tls:        tls.clone(),
            pool_on:    pool::Builder::new().pool_size(1).name_prefix("rem-").create(),
            pool_off:   pool::Builder::new().pool_size(1).name_prefix("loc-").create(),
            futures:    tx,
            profiles:   Arc::new(Mutex::new(pdb)),
            builder:    builder,
            header:     header,
            info:       info,
            revealer:   revealer,
            mainview:   mainview,
            convlist:   convlist,
            send_btn:   sendbtn,
            me_box:     popover,
            timezone:   Local::now().timezone(),
            channels:   Rc::new(CHashMap::new()),
            contacts:   contacts,
            resources:  Rc::new(res::Resources::new()),
            sig_online: Rc::new(sig_online)
        };

        app.connect_startup(Coax::startup);
        app.connect_activate(move |app| coax.activate(app, css.clone()));
        Ok(app)
    }

    fn startup(app: &gtk::Application) {
        let builder = Builder::new_from_string(include_str!("gtk/app-menu.ui"));
        let menu: MenuModel = builder.get_object("app-menu").unwrap();

        let quit = SimpleAction::new("quit", None);
        quit.connect_activate(with!(app => move |_, _| app.quit()));

        app.add_action(&quit);
        app.set_app_menu(Some(&menu))
    }

    fn activate(&self, app: &gtk::Application, css: gtk::CssProvider) {
        trace!(self.log, "activate");
        let this = self.clone();

        let window = gtk::ApplicationWindow::new(app);
        window.set_size_request(800, 600);

        window.get_screen().map(|s| {
            gtk::StyleContext::add_provider_for_screen(&s, &css, GTK_STYLE_PROVIDER_PRIORITY_USER as u32)
        });

        let menu: MenuButton = self.header.get_object("menu-button").unwrap();
        let profile_menu: gtk::MenuButton = self.header.get_object("profile-menu").unwrap();
        profile_menu.set_sensitive(false);

        let menu_builder = Builder::new_from_string(include_str!("gtk/button-menu.ui"));
        let model: MenuModel = menu_builder.get_object("button-menu").unwrap();
        menu.set_menu_model(Some(&model));

        let new_conv: gtk::ToolButton = self.builder.get_object("add-conv-button").unwrap();
        new_conv.set_sensitive(false);

        let show_contacts: gtk::ToolButton = self.builder.get_object("show-cons-button").unwrap();
        show_contacts.set_sensitive(false);

        // Open menu action

        let open = SimpleAction::new("open", None);
        open.connect_activate(with!(this, app, window, profile_menu => move |open, _| {
            let builder = Builder::new_from_string(include_str!("gtk/open-account.ui"));
            let notebook: gtk::Notebook = builder.get_object("open-notebook").unwrap();
            let flags  = gtk::DIALOG_USE_HEADER_BAR | gtk::DIALOG_MODAL | gtk::DIALOG_DESTROY_WITH_PARENT;
            let dialog = gtk::Dialog::new_with_buttons(Some("Open"), Some(&window), flags, &[]);
            dialog.get_content_area().add(&notebook);
            dialog.add_button("Cancel", gtk::ResponseType::Cancel.into());

            let submit = dialog.add_button("Submit", gtk::ResponseType::Ok.into());
            submit.set_sensitive(false);
            submit.set_can_default(true);
            submit.grab_default();
            submit.get_style_context().map(|ctx| ctx.add_class("suggested-action"));

            let reg_name: gtk::Entry = builder.get_object("register-name-entry").unwrap();
            let reg_email: gtk::Entry = builder.get_object("register-email-entry").unwrap();
            let reg_pass: gtk::Entry = builder.get_object("register-pass-entry").unwrap();
            Coax::setup_register_entry_handlers(&submit, &reg_email, &reg_name, &reg_pass);

            let login_email: gtk::Entry = builder.get_object("login-email-entry").unwrap();
            let login_pass: gtk::Entry = builder.get_object("login-pass-entry").unwrap();
            Coax::setup_login_entry_handlers(&submit, &login_email, &login_pass);

            let profiles_list: gtk::ListBox = builder.get_object("profiles-list").unwrap();
            this.setup_profiles(&app, &submit, &profiles_list);

            with! { login_email, login_pass, reg_name, reg_email, reg_pass, submit =>
                notebook.connect_switch_page(move |_, _, num| {
                    let value = match num {
                        1 => Coax::proceed_login(&login_email, &login_pass),
                        2 => Coax::proceed_registration(&reg_email, &reg_name, &reg_pass),
                        _ => false
                    };
                    submit.set_sensitive(value)
                })
            };

            let response = dialog.run();
            dialog.hide();

            if response != gtk::ResponseType::Ok.into() {
                return ()
            }

            let enable = vec![profile_menu.clone().upcast::<gtk::Widget>()];
            match notebook.get_current_page() {
                Some(0) => {
                    let row = from_some!(profiles_list.get_selected_row());
                    let id = ffi::get_data(&row, &ffi::KEY_ID);
                    with! { open =>
                        this.on_profile(&app, &window, open, enable, id.cloned())
                    }
                }
                Some(1) => {
                    let email = login_email.get_text().unwrap_or(String::new());
                    let pass = login_pass.get_text().unwrap_or(String::new());
                    with! { open =>
                        this.on_login(&app, &window, open, enable, Email::new(email), Password::new(pass))
                    }
                }
                Some(2) => {
                    let name = reg_name.get_text().unwrap_or(String::new());
                    let email = reg_email.get_text().unwrap_or(String::new());
                    let pass = reg_pass.get_text().unwrap_or(String::new());
                    this.on_register(&app, Name::new(name), Email::new(email), Password::new(pass))
                }
                _ => {}
            }
        }));

        // Find button

        let find_button: gtk::ToggleButton = self.header.get_object("find-toggle-button").unwrap();
        let search_bar: gtk::SearchBar = self.builder.get_object("searchbar").unwrap();
        let search_input: gtk::SearchEntry = self.builder.get_object("search-entry").unwrap();
        find_button.connect_toggled(move |b| {
            if b.get_active() {
                search_bar.set_search_mode(true);
                search_input.grab_focus()
            } else {
                search_bar.set_search_mode(false)
            }
        });

        let main_pane: gtk::Paned = self.builder.get_object("main-pane").unwrap();
        let max_left = SimpleAction::new("max_left", None);
        max_left.connect_activate(with!(main_pane => move |_, _| {
            if let (Some(c1), Some(c2)) = (main_pane.get_child1(), main_pane.get_child2()) {
                match (c1.is_visible(), c2.is_visible()) {
                    (true, true)  => c2.set_visible(false),
                    (false, _)    => c1.set_visible(true),
                    _             => {}
                }
            }
        }));
        let max_right = SimpleAction::new("max_right", None);
        max_right.connect_activate(move |_, _| {
            if let (Some(c1), Some(c2)) = (main_pane.get_child1(), main_pane.get_child2()) {
                match (c1.is_visible(), c2.is_visible()) {
                    (true, true) => c1.set_visible(false),
                    (_, false)   => c2.set_visible(true),
                    _            => {}
                }
            }
        });
        let right_pane: gtk::Paned = self.builder.get_object("right-pane").unwrap();
        let max_top = SimpleAction::new("max_top", None);
        max_top.connect_activate(with!(right_pane => move |_, _| {
            if !right_pane.is_visible() {
                return ()
            }
            if let (Some(c1), Some(c2)) = (right_pane.get_child1(), right_pane.get_child2()) {
                match (c1.is_visible(), c2.is_visible()) {
                    (true, true)  => c2.set_visible(false),
                    (false, _)    => c1.set_visible(true),
                    _             => {}
                }
            }
        }));
        let max_bottom = SimpleAction::new("max_bottom", None);
        let input: gtk::TextView = self.builder.get_object("main-text-input").unwrap();
        max_bottom.connect_activate(move |_, _| {
            if !right_pane.is_visible() {
                return ()
            }
            if let (Some(c1), Some(c2)) = (right_pane.get_child1(), right_pane.get_child2()) {
                match (c1.is_visible(), c2.is_visible()) {
                    (true, true) => c1.set_visible(false),
                    (_, false)   => c2.set_visible(true),
                    _            => {}
                }
                input.grab_focus()
            }
        });

        window.add_action(&open);
        window.add_action(&max_left);
        window.add_action(&max_right);
        window.add_action(&max_top);
        window.add_action(&max_bottom);
        app.set_accels_for_action("win.send", &["<Shift>Return"]);
        app.set_accels_for_action("win.open", &["<Ctrl>o"]);
        app.set_accels_for_action("win.max_left", &["<Alt>Right"]);
        app.set_accels_for_action("win.max_right", &["<Alt>Left"]);
        app.set_accels_for_action("win.max_top", &["<Alt>Down"]);
        app.set_accels_for_action("win.max_bottom", &["<Alt>Up"]);

        let main: gtk::Box = self.builder.get_object("main").unwrap();
        window.add(&main);

        let bar: HeaderBar = self.header.get_object("header").unwrap();
        window.set_titlebar(Some(&bar));

        window.show_all()
    }

    fn setup_profiles(&self, app: &gtk::Application, submit: &gtk::Widget, list: &gtk::ListBox) {
        trace!(self.log, "setup_profiles");
        let profiles =
            match profile::load_profiles(&*self.profiles.lock().unwrap()) {
                Ok(pp) => pp,
                Err(e) => {
                    let details = format!("{}", e);
                    error!(self.log, "error loading profiles"; "error" => %details);
                    show_message(app, MessageType::Error, "Failed to load profiles", "", Some(&details));
                    return ()
                }
            };
        let img_size = gtk::IconSize::LargeToolbar.into();
        for p in &profiles {
            let row = gtk::ListBoxRow::new();
            let row_box = gtk::Box::new(Orientation::Horizontal, 12);
            let img = gtk::Image::new_from_icon_name("gtk-orientation-portrait", img_size);
            img.set_margin_top(6);
            img.set_margin_left(6);
            img.set_margin_bottom(6);
            img.set_margin_right(6);
            let lbl = gtk::Label::new(None);
            let id = p.handle.as_ref().map(|x| x.as_str()).or_else(
                  || p.email.as_ref().map(|x| x.as_str()).or_else(
                  || p.phone.as_ref().map(|x| x.as_str()))).unwrap_or("");
            let name = ffi::escape(p.name.as_str()).to_string_lossy();
            lbl.set_markup(&format!("<big><b>{}</b></big>\n{}", name, id));
            lbl.set_margin_top(6);
            lbl.set_margin_left(6);
            lbl.set_margin_bottom(6);
            lbl.set_margin_right(6);
            row_box.add(&img);
            row_box.add(&lbl);
            row.add(&row_box);
            ffi::set_data(&row, &ffi::KEY_ID, p.id.clone());
            list.insert(&row, -1)
        }
        list.connect_row_selected(with!(submit => move |_, row| {
            submit.set_sensitive(row.is_some())
        }));
        list.show_all()
    }

    fn setup_callbacks(&self, state: &Arc<State>, app: &gtk::Application, app_win: &gtk::ApplicationWindow) {
        let this = self.clone();

        let new_conv: gtk::ToolButton = self.builder.get_object("add-conv-button").unwrap();
        new_conv.connect_clicked(with!(this, state, app => move |_| this.show_new_conv(&state, &app)));
        new_conv.set_sensitive(state.is_online.load(Ordering::Relaxed));

        self.sig_online.connect(move |status| new_conv.set_sensitive(*status));

        let show_contacts: gtk::ToolButton = self.builder.get_object("show-cons-button").unwrap();
        show_contacts.connect_clicked(with!(this, state, app => move |_| {
            if !this.contacts.is_init() {
                let f =
                    this.load_local_contacts(&state, &app)
                        .map_err(with!(this, app => move |e| {
                            error!(this.log, "failed to load local contacts"; "error" => %e);
                            show_error(&app, &e, "Failed to load contacts", "")
                        }));
                this.futures.send(boxed(f)).unwrap()
            }
            this.send_btn.set_sensitive(false);
            this.convlist.unselect_all();
            this.mainview.remove_row(0);
            this.mainview.insert_row(0);
            this.mainview.attach(this.contacts.contact_view(), 0, 0, 1, 1);
            this.mainview.show_all()
        }));
        show_contacts.set_sensitive(true);

        self.contacts.set_refresh_action(with!(this, state, app => move |btn| {
            let future =
                this.load_remote_contacts(&state, &app)
                    .and_then(with!(this, state, app => move |()| this.load_remote_conversations(&state, &app)))
                    .map(with!(btn => move |()| { btn.set_sensitive(true) }))
                    .map_err(with!(this, app => move |e| {
                        btn.set_sensitive(true);
                        error!(this.log, "failed to load remote contacts"; "error" => %e);
                        show_error(&app, &e, "Failed to load contacts", "")
                    }));
            this.futures.send(boxed(future)).unwrap()
        }));
        self.contacts.set_refresh_enabled(state.is_online.load(Ordering::Relaxed));

        let input: gtk::TextView = self.builder.get_object("main-text-input").unwrap();

        self.convlist.connect_row_selected(with!(this, state, app, input => move |_, row| {
            if let Some(r) = row.as_ref() {
                ffi::get_data(r, &ffi::KEY_ID).map(|id| {
                    if let Some(ch) = this.channels.get(id) {
                        if !ch.is_init() {
                            this.on_message_demand(&state, &app, &id)
                        }
                        ch.set_read();
                        this.mainview.remove_row(0);
                        this.mainview.insert_row(0);
                        this.mainview.attach(ch.message_view(), 0, 0, 1, 1);
                        this.mainview.show_all();
                        let value = input.get_buffer()
                            .map(|buf| buf.get_char_count() > 0)
                            .unwrap_or(false);
                        this.send_btn.set_sensitive(value && ch.is_current());
                    } else {
                        this.send_btn.set_sensitive(false)
                    }
                });
            } else {
                this.send_btn.set_sensitive(false)
            }
        }));

        let button = self.send_btn.clone();

        input.get_buffer().map(|buf| buf.connect_changed(with!(button, this => move |buf| {
            let is_current =
                if let Some(row) = this.convlist.get_selected_row() {
                    ffi::get_data(&row, &ffi::KEY_ID)
                        .and_then(|id| {
                            this.channels.get(id).map(|chan| chan.is_current())
                        })
                } else {
                    None
                };
            let value = is_current == Some(true) && buf.get_char_count() > 0;
            button.set_sensitive(value)
        })));

        let send = SimpleAction::new("send", None);
        send.connect_activate(with!(this, state, input => move |_, _| {
            if !button.is_sensitive() {
                return ()
            }
            let row = this.convlist.get_selected_row();
            let cid = from_some!(row.as_ref().and_then(|r| ffi::get_data(r, &ffi::KEY_ID)));
            let buf = from_some!(input.get_buffer());
            let (mut s, mut e) = buf.get_bounds();
            let txt = from_some!(buf.get_text(&s, &e, false));
            let msg = MsgBuilder::new().text(txt.trim_right()).finish();
            let log = this.log.clone();
            let fut = this.send_message(&state, cid, msg).map_err(move |e| {
                error!(log, "failed to send message"; "error" => ?e)
            });
            this.futures.send(boxed(fut)).unwrap();
            buf.delete(&mut s, &mut e);
        }));
        self.send_btn.connect_clicked(with!(send => move |_| send.activate(None)));
        app_win.add_action(&send);

        let conv_view: gtk::ScrolledWindow = self.builder.get_object("conversation-view").unwrap();

        if let Some(vadj) = conv_view.get_vadjustment() {
            vadj.connect_value_changed(with!(this, state, app => move |va| {
                if va.get_value() == va.get_upper() - va.get_page_size() { // at bottom
                    this.on_conversation_demand(&state, &app)
                }
            }));
        }
    }

    fn proceed_registration(email: &gtk::Entry, name: &gtk::Entry, pass: &gtk::Entry) -> bool {
        3 <= name.get_text_length() && 5 <= email.get_text_length() && 8 <= pass.get_text_length()
    }

    fn setup_register_entry_handlers(submit: &gtk::Widget, email: &gtk::Entry, name: &gtk::Entry, pass: &gtk::Entry) {
        name.connect_key_release_event(with!(submit, email, pass => move |name, _| {
            submit.set_sensitive(Coax::proceed_registration(&email, &name, &pass));
            gtk::Inhibit(false)
        }));
        email.connect_key_release_event(with!(submit, name, pass => move |email, _| {
            submit.set_sensitive(Coax::proceed_registration(&email, &name, &pass));
            gtk::Inhibit(false)
        }));
        pass.connect_key_release_event(with!(submit, name, email => move |pass, _| {
            submit.set_sensitive(Coax::proceed_registration(&email, &name, &pass));
            gtk::Inhibit(false)
        }));
    }

    fn proceed_login(email: &gtk::Entry, pass: &gtk::Entry) -> bool {
        5 <= email.get_text_length() && 8 <= pass.get_text_length()
    }

    fn setup_login_entry_handlers(submit: &gtk::Widget, email: &gtk::Entry, pass: &gtk::Entry) {
        email.connect_key_release_event(with!(submit, pass => move |email, _| {
            submit.set_sensitive(Coax::proceed_login(&email, &pass));
            gtk::Inhibit(false)
        }));
        pass.connect_key_release_event(with!(submit, email => move |pass, _| {
            submit.set_sensitive(Coax::proceed_login(&email, &pass));
            gtk::Inhibit(false)
        }));
    }

    fn on_register(&self, app: &gtk::Application, n: Name<'static>, e: Email<'static>, p: Password<'static>) {
        debug!(self.log, "on_register"; "e-mail" => %e);
        let this   = self.clone();
        let actor  = Actor::new(&self.log, self.config.clone(), self.tls.clone());
        let future =
            self.pool_on.spawn_fn(move || {
                let params = user::register::Params::email(e, n, p);
                let client = actor.connect()?;
                let mut a  = actor.connected(client);
                a.register_user(&params).map_err(From::from)
            })
            .map(with!(app, this => move |()| {
                this.hide_info();
                let txt = "Please check your e-mail account and verify your address.";
                show_message(&app, MessageType::Info, "Account registered", txt, None);
            }))
            .map_err(with!(app, this => move |e| {
                this.hide_info();
                error!(this.log, "failed to register account"; "error" => %e);
                show_error(&app, &e, "Failed to register account", "")
            }));
        self.show_info("Registering your account ...");
        self.futures.send(boxed(future)).unwrap()
    }

    fn on_login(&self, app: &gtk::Application, app_win: &gtk::ApplicationWindow, disable: SimpleAction, enable: Vec<gtk::Widget>, e: Email<'static>, p: Password<'static>) {
        debug!(self.log, "on_login"; "e-mail" => %e);
        let (bcast_tx, bcast_rx) = std::sync::mpsc::channel();
        let actor  = Actor::new(&self.log, self.config.clone(), self.tls.clone());
        let this   = self.clone();
        let profs  = self.profiles.clone();
        let future =
            self.pool_on.spawn_fn(move || {
                let client       = actor.connect()?;
                let mut actor    = actor.connected(client);
                let params       = user::login::Params::email(e, p, Label::new("coax-gtk"));
                let user         = actor.login(&params)?;
                let mut actor_on = actor.online(user, bcast_tx);
                let me           = actor_on.me().clone();
                let is_new       = actor_on.is_new_client();
                profs.lock().unwrap().insert(&me)?;
                let mut inbox = actor_on.new_inbox()?;
                let wsock = inbox.connect()?;
                let state = State {
                    me:        me,
                    actor_off: Mutex::new(actor_on.clone_offline()?),
                    actor_on:  Mutex::new(Some(actor_on)),
                    inbox:     Mutex::new(Some(inbox.fork(wsock))),
                    is_sync:   AtomicBool::new(false),
                    is_online: AtomicBool::new(true),
                    ch_state:  Mutex::new(None)
                };
                Ok((Arc::new(state), is_new))
            })
            .and_then(with!(this, app, app_win => move |(state, is_new_client)| {
                this.setup_callbacks(&state, &app, &app_win);
                this.show_info("Loading conversations ...");
                set_subtitle(&app, Some(state.me.name.as_str()));
                this.ensure_user_res(&state, &state.me, false);
                let prof = ProfileView::new(&mut this.resources.user_mut(&state.me.id).unwrap());
                this.me_box.add(prof.vbox());
                disable.set_enabled(false);
                for e in &enable {
                    e.set_sensitive(true)
                }
                gtk::timeout_add(100, with!(this, state, app => move || {
                    for pkg in bcast_rx.try_iter() {
                        this.on_incoming(&state, &app, pkg)
                    }
                    Continue(true)
                }));
                if is_new_client {
                    let f = this.load_remote_conversations(&state, &app)
                        .and_then(with!(this, state => move |()| this.load_remote_contacts(&state, &app)))
                        .map(with!(state => move |()| state));
                    boxed(f)
                } else {
                    let f = this.load_local_conversations(&state, &app)
                        .map(with!(state => move |()| state));
                    boxed(f)
                }
            }))
            .and_then(with!(this => move |state| {
                this.show_info("Synchronising ...");
                this.notifications(&state, true).map(move |()| state)
            }))
            .and_then(with!(this => move |state| {
                this.hide_info();
                this.set_user_icon(&state, state.me.id.clone())
            }))
            .map_err(with!(app, this => move |e| {
                this.hide_info();
                error!(this.log, "failed to sign in"; "error" => %e);
                show_error(&app, &e, "Failed to sign in.", "")
            }));
        self.show_info("Signing in ...");
        self.futures.send(boxed(future)).unwrap()
    }

    fn on_profile(&self, app: &gtk::Application, app_win: &gtk::ApplicationWindow, disable: SimpleAction, enable: Vec<gtk::Widget>, uid: Option<UserId>) {
        debug!(self.log, "on_profile"; "user" => ?uid);
        let user_id =
            if let Some(u) = uid {
                u
            } else {
                show_message(app, MessageType::Error, "Invalid User-ID", "", None);
                return ()
            };

        let (bcast_tx, bcast_rx) = std::sync::mpsc::channel();
        let mut actor = Actor::new(&self.log, self.config.clone(), self.tls.clone());
        let this   = self.clone();
        let logger = self.log.clone();
        let future =
            self.pool_on.spawn_fn(move || {
                let user      = actor.profile(&user_id)?;
                let actor_off = actor.offline(user, bcast_tx);
                let state     = State {
                    me:        actor_off.me().clone(),
                    actor_off: Mutex::new(actor_off),
                    actor_on:  Mutex::new(None),
                    inbox:     Mutex::new(None),
                    is_sync:   AtomicBool::new(false),
                    is_online: AtomicBool::new(false),
                    ch_state:  Mutex::new(None)
                };
                Ok(Arc::new(state))
            })
            .and_then(with!(this, app, app_win => move |state| {
                this.setup_callbacks(&state, &app, &app_win);
                this.show_info("Loading conversations ...");
                set_subtitle(&app, Some(state.me.name.as_str()));
                this.ensure_user_res(&state, &state.me, false);
                let prof = ProfileView::new(&mut this.resources.user_mut(&state.me.id).unwrap());
                this.me_box.add(prof.vbox());
                disable.set_enabled(false);
                for e in &enable {
                    e.set_sensitive(true)
                }
                gtk::timeout_add(100, with!(this, state, app => move || {
                    for pkg in bcast_rx.try_iter() {
                        this.on_incoming(&state, &app, pkg)
                    }
                    Continue(true)
                }));
                this.load_local_conversations(&state, &app).map(with!(state => |()| state))
            }))
            .and_then(with!(this => move |state| {
                this.pool_on.spawn_fn(move || {
                    {
                        let actor_off     = state.actor_off.lock().unwrap();
                        let client        = actor_off.connect()?;
                        let mut actor_on  = actor_off.clone_online(client)?;
                        actor_on.renew_access()?;
                        actor_on.resolve_user(&state.me.id, false)?;
                        let mut i = actor_on.new_inbox()?;
                        let     w = i.connect()?;
                        *state.inbox.lock().unwrap()    = Some(i.fork(w));
                        *state.actor_on.lock().unwrap() = Some(actor_on);
                        state.is_online.store(true, Ordering::Relaxed);
                    }
                    Ok(state)
                })
            }))
            .and_then(with!(this => move |state| {
                this.show_info("Synchronising ...");
                this.notifications(&state, true).map(move |()| state)
            }))
            .map(with!(this => move |state| {
                this.hide_info();
                this.set_user_icon(&state, state.me.id.clone());
                this.sig_online.emit(true);
                state
            }))
            .and_then(with!(this => move |state| {
                this.resend_messages(&state)
            }))
            .map_err(with!(app => move |e| {
                this.hide_info();
                error!(logger, "failed to activate profile"; "error" => %e);
                show_error(&app, &e, "Failed to activate profile", "")
            }));
        self.show_info("Signing in ...");
        self.futures.send(boxed(future)).unwrap()
    }

    //
    // Callbacks
    //

    fn on_incoming(&self, state: &Arc<State>, app: &gtk::Application, pkg: Pkg) {
        trace!(self.log, "on_incoming");
        match pkg {
            Pkg::Connected => {
                state.is_online.store(true, Ordering::Relaxed);
                self.sig_online.emit(true);
                let logger = self.log.clone();
                let future = self.notifications(state, false)
                    .map_err(with!(logger => move |e| {
                        error!(logger, "failed to get notifications"; "error" => %e)
                    }));
                self.futures.send(boxed(future)).unwrap();
                self.hide_info()
            }
            Pkg::Disconnected => {
                state.is_online.store(false, Ordering::Relaxed);
                self.sig_online.emit(false);
                self.show_info("Connection lost. Reconnecting ...")
            }
            Pkg::Message(m)                   => self.on_message(state, app, m),
            Pkg::MessageUpdate(c, m, t, s)    => self.on_message_update(state, app, m, c, t, s),
            Pkg::Conversation(c)              => self.on_conversation(state, app, c),
            Pkg::Contact(u, c)                => self.on_contact(state, app, u, c),
            Pkg::MembersChange(s, d, c, m, u) => self.on_members_change(state, app, d, c, m, s, u)
        }
    }

    fn on_message(&self, state: &Arc<State>, app: &gtk::Application, m: coax_data::Message<'static>) {
        debug!(self.log, "on_message"; "conv" => %m.conv, "id" => %m.id);
        let this   = self.clone();
        let logger = self.log.clone();
        if let Some(ch) = self.channels.get(&m.conv) {
            if !ch.has_msg(&m.id) {
                self.ensure_user_res(state, &m.user, false);
                let mtime   = m.time.with_timezone(&self.timezone);
                let mut usr = self.resources.user_mut(&m.user.id).unwrap();
                self.show_notification(app, &usr, &m);
                if ch.is_init() {
                    match m.data {
                        MessageData::Text(txt) =>
                            ch.push_msg(&m.id, Message::text(Some(mtime), &mut usr, &txt)),
                        MessageData::Asset(ast) => {
                            if ast.atype == AssetType::Image {
                                let img = gtk::DrawingArea::new();
                                let msg = Image::new(mtime, &mut usr, img.clone(), app.get_active_window());
                                let aid = ast.id.clone();
                                msg.signal_save().connect(with!(state, app => move |p| {
                                    this.save_as(&state, &app, aid.clone(), p.clone())
                                }));
                                msg.start_spinner();
                                ch.push_msg(&m.id, Message::Image(msg));
                                let future = self.set_image(state, ast, m.conv.clone(), m.id.clone(), img)
                                    .map_err(with!(logger => move |e| {
                                        error!(logger, "failed to set image"; "error" => %e)
                                    }));
                                self.futures.send(boxed(future)).unwrap()
                            }
                        }
                        _ => {}
                    }
                } else {
                    ch.update_time(&mtime)
                }
                self.convlist.invalidate_sort();
                if !ch.is_selected() {
                    ch.set_unread()
                }
                if ch.conv_type() == ConvType::OneToOne && !state.is_sync.load(Ordering::Relaxed) {
                    let future = self.send_confirmation(state, &m.conv, &m.id)
                        .map_err(with!(logger => move |e| {
                            error!(logger, "failed to send confirmation"; "error" => %e)
                        }));
                    self.futures.send(boxed(future)).unwrap();
                }
            }
        } else {
            let conv_id = m.conv.to_string();
            info!(self.log, "message for unresolved conversation"; "conv" => %conv_id);
            let future = self.conversation(state, &m.conv)
                .map(with!(this, state, app => move |conv| {
                    if let Some(c) = conv {
                        this.on_conversation(&state, &app, c);
                        this.on_message(&state, &app, m)
                    } else {
                        error!(this.log, "failed to resolve conversation"; "id" => conv_id)
                    }
                }))
                .map_err(move |e| {
                    error!(logger, "on_message error"; "error" => %e)
                });
            self.futures.send(boxed(future)).unwrap()
        }
    }

    fn on_message_update(&self, state: &Arc<State>, app: &gtk::Application, id: String, c: ConvId, t: DateTime<UTC>, s: MessageStatus) {
        debug!(self.log, "on_message_update"; "conv" => %c, "id" => %id);
        if let Some(ch) = self.channels.get(&c) {
            if let Some(mut m) = ch.get_msg_mut(&id) {
                if let Message::Text(ref mut msg) = *m {
                    match s {
                        MessageStatus::Sent      => msg.set_time(t.with_timezone(&self.timezone)),
                        MessageStatus::Delivered => msg.set_delivered(t.with_timezone(&self.timezone)),
                        _                        => {}
                    }
                }
            }
        } else {
            info!(self.log, "message update for unresolved conversation"; "conv" => %c);
            let this   = self.clone();
            let logger = self.log.clone();
            let future = self.conversation(state, &c)
                .map(with!(this, state, app => move |conv| {
                    if let Some(c) = conv {
                        let cid = c.id.clone();
                        this.on_conversation(&state, &app, c);
                        this.on_message_update(&state, &app, id, cid, t, s)
                    } else {
                        error!(this.log, "failed to resolve conversation"; "id" => %c);
                    }
                }))
                .map_err(move |e| {
                    error!(logger, "on_message_update error"; "error" => %e)
                });
            self.futures.send(boxed(future)).unwrap()
        }
    }

    fn on_conversation(&self, state: &Arc<State>, app: &gtk::Application, mut conv: Conversation<'static>) {
        debug!(self.log, "on_conversation"; "conv" => %conv.id);
        if self.channels.contains_key(&conv.id) {
            debug!(self.log, "conversation already loaded"; "conv" => %conv.id);
            return ()
        }

        if conv.ctype == ConvType::SelfConv {
            debug!(self.log, "ignoring self conversation"; "conv" => %conv.id);
            return ()
        }

        let this = self.clone();
        let cid  = conv.id.clone();

        if conv.ctype == ConvType::Group {
            let ch = Channel::group(&conv.time.with_timezone(&self.timezone), &conv.id, &conv.name, conv.members.len());
            ch.set_current(conv.status == ConvStatus::Current);
            ch.signal_at_top().connect(with!(state, app, cid => move |_| {
                this.on_message_demand(&state, &app, &cid)
            }));
            self.convlist.add(ch.channel_row());
            self.channels.insert(conv.id, ch);
            self.convlist.show_all();
            return ()
        }

        // Set remote user name as conversation name if user is already in `self.resources`.
        if let Some(uid) = conv.members.iter().filter(|m| **m != state.me.id).next().cloned() {
            if let Some(mut u) = self.resources.user_mut(&uid) {
                conv.set_name(Name::new(u.name.clone()));
                let ch = Channel::one_to_one(&conv.time.with_timezone(&self.timezone), &conv.id, &mut u);
                ch.set_current(conv.status == ConvStatus::Current);
                ch.signal_at_top().connect(with!(state, app, cid => move |_| {
                    this.on_message_demand(&state, &app, &cid)
                }));
                self.convlist.add(ch.channel_row());
                self.channels.insert(conv.id, ch);
                self.convlist.show_all();
                return ()
            }
        }

        let user_id =
            if let Some(id) = conv.members.iter().filter(|m| **m != state.me.id).next().cloned() {
                id
            } else {
                warn!(self.log, "no user found in 1:1 conversation"; "conv" => %conv.id);
                return ()
            };

        let this   = self.clone();
        let future = self.user(state, user_id.clone(), true)
            .map(with!(this, state, app => move |u| {
                if let Some(user) = u {
                    if !this.channels.contains_key(&conv.id) {
                        this.ensure_user_res(&state, &user, false);
                        let mut usr = this.resources.user_mut(&user.id).unwrap();
                        let chn = Channel::one_to_one(&conv.time.with_timezone(&this.timezone), &conv.id, &mut usr);
                        chn.set_current(conv.status == ConvStatus::Current);
                        chn.signal_at_top().connect(with!(this, state, app, cid => move |_| {
                            this.on_message_demand(&state, &app, &cid)
                        }));
                        this.convlist.add(chn.channel_row());
                        this.channels.insert(conv.id, chn);
                        this.convlist.show_all()
                    }
                } else {
                    warn!(this.log, "user not found"; "id" => %user_id);
                    return ()
                }
            }))
            .map_err(with!(this => move |e| {
                error!(this.log, "failed to post-process one to one conversation"; "error" => %e)
            }));

        self.futures.send(boxed(future)).unwrap()
    }

    fn on_contact(&self, state: &Arc<State>, app: &gtk::Application, to: User<'static>, contact: Connection) {
        debug!(self.log, "on_contact"; "to" => %to.id);
        self.ensure_user_res(state, &to, true);
        let mut u = self.resources.user_mut(&to.id).unwrap();
        if let Some(mut cont) = self.contacts.get_mut(&to.id) {
            cont.block_handler(true);
            cont.set_status(contact.status);
            cont.block_handler(false);
            return ()
        }
        let this = self.clone();
        let uid  = to.id.clone();
        let cid  = contact.conv.clone();
        let cont = Contact::new(&mut u, &contact, with!(state, app => move |w, s| {
            this.on_connect_change(&state, &app, w, uid.clone(), cid.clone(), s)
        }));
        cont.set_enabled(state.is_online.load(Ordering::Relaxed));
        self.sig_online.connect(with!(cont => move |status| cont.set_enabled(*status)));
        self.contacts.add(&mut u, cont)
    }

    fn on_members_change(&self, state: &Arc<State>, app: &gtk::Application, dt: DateTime<UTC>, cid: ConvId, members: Vec<User<'static>>, s: ConvStatus, from: User<'static>) {
        debug!(self.log, "on_members_change"; "conv" => %cid);
        if let Some(chan) = self.channels.get(&cid) {
            let local = dt.with_timezone(&self.timezone);
            for m in members {
                if m.id == state.me.id {
                    chan.set_current(s == ConvStatus::Current)
                }
                if chan.is_init() {
                    let txt = match s {
                        ConvStatus::Current =>
                            if m.id == from.id {
                                format!("{} has joined this conversation.", m.name.as_str())
                            } else {
                                format!("{} has added {} to this conversation.", from.name.as_str(), m.name.as_str())
                            },
                        ConvStatus::Previous =>
                            if m.id == from.id {
                                format!("{} has left this conversation.", m.name.as_str())
                            } else {
                                format!("{} has removed {} from this conversation.", from.name.as_str(), m.name.as_str())
                            }
                    };
                    let mid = random_uuid().to_string();
                    chan.push_msg(&mid, Message::system(local, &txt));
                    if !chan.is_selected() {
                        chan.set_unread()
                    }
                }
            }
        } else {
            let this   = self.clone();
            let future = self.conversation(&state, &cid)
                .map(with!(this, state, app => move |conv| {
                    if let Some(c) = conv {
                        this.on_conversation(&state, &app, c);
                        this.on_members_change(&state, &app, dt, cid, members, s, from)
                    } else {
                        error!(this.log, "Failed to resolve conversation"; "id" => %cid)
                    }
                }))
                .map_err(with!(this => move |e| {
                    error!(this.log, "on_members_change error"; "error" => %e)
                }));
            self.futures.send(boxed(future)).unwrap()
        }
    }

    fn on_new_conv(&self, state: &Arc<State>, app: &gtk::Application, name: String, u: UserId) {
        trace!(self.log, "on_new_conv");

        enum Data<'a> {
            Sent,
            NoUser,
            Invalid(ConnectStatus),
            Conv(Conversation<'a>)
        }

        let this   = self.clone();
        let future =
            self.pool_on.spawn_fn(with!(state => move || {
                let mut actor_guard = state.actor_on.lock().unwrap();
                if let Some(ref mut a) = *actor_guard {
                    let n = Name::new(name);
                    match a.resolve_connection(&u)? {
                        Some(conn) =>
                            if conn.status != ConnectStatus::Accepted {
                                return Ok(Data::Invalid(conn.status))
                            },
                        None => {
                            if let Some(usr) = a.resolve_user(&u, true)? {
                                a.new_connection(&usr, n.replicate(), "Connection request")?;
                                return Ok(Data::Sent)
                            } else {
                                return Ok(Data::NoUser)
                            }
                        }
                    }
                    let c = a.new_conversation(n, &[u])?;
                    Ok(Data::Conv(c))
                } else {
                    Err(Error::InvalidAppState)
                }
            }))
            .map(with!(this, state, app => move |data| {
                match data {
                    Data::Conv(c) => this.on_conversation(&state, &app, c),
                    Data::Sent    => show_message(&app, MessageType::Info, "Connection request sent", "", None),
                    Data::NoUser  => show_message(&app, MessageType::Error, "User not found", "", None),
                    Data::Invalid(s) => {
                        let s = format!("The current connection status ({}) does not allow creating a new conversation with this user.", s.as_str());
                        show_message(&app, MessageType::Info, "Not allowed", &s, None)
                    }
                }
            }))
            .map_err(with!(app => move |e| {
                error!(this.log, "failed to create conversation"; "error" => %e);
                show_error(&app, &e, "Failed to create conversation", "")
            }));
        self.futures.send(boxed(future)).unwrap()
    }

    fn on_connect_change(&self, state: &Arc<State>, app: &gtk::Application, s: &gtk::ComboBoxText, uid: UserId, cid: ConvId, new: ConnectStatus) {
        debug!(self.log, "on_connect_change"; "user" => %uid);
        s.set_sensitive(false);
        let this   = self.clone();
        let future =
            self.pool_on.spawn_fn(with!(state => move || {
                let mut actor_guard = state.actor_on.lock().unwrap();
                if let Some(ref mut a) = *actor_guard {
                    a.update_connection(&uid, new).map_err(From::from)
                } else {
                    Err(Error::InvalidAppState)
                }
            }))
            .and_then(with!(this, state, app => move |updated| {
                if updated && new == ConnectStatus::Accepted {
                    boxed(this.conversation(&state, &cid).map(move |conv| conv.map(|c| this.on_conversation(&state, &app, c))))
                } else {
                    boxed(future::ok(None))
                }
            }))
            .map(with!(s => move |_| {
                s.set_sensitive(true)
            }))
            .map_err(with!(app, s => move |e| {
                s.set_sensitive(true);
                error!(this.log, "failed to update status"; "error" => %e);
                show_error(&app, &e, "Failed to update status", "")
            }));
        self.futures.send(boxed(future)).unwrap()
    }

    fn on_message_demand(&self, state: &Arc<State>, app: &gtk::Application, cid: &ConvId) {
        let logger = self.log.clone();
        let future = self.load_messages(state, app, cid)
            .map_err(with!(app => move |e| {
                error!(logger, "failed to load messages"; "error" => ?e);
                show_error(&app, &e, "Failed to load messages", "")
            }));
        self.futures.send(boxed(future)).unwrap()
    }

    fn on_conversation_demand(&self, state: &Arc<State>, app: &gtk::Application) {
        let logger = self.log.clone();
        let future = self.load_local_conversations(state, app)
            .map_err(with!(app => move |e| {
                error!(logger, "failed to load conversations"; "error" => ?e);
                show_error(&app, &e, "Failed to load conversations", "");
            }));
        self.futures.send(boxed(future)).unwrap()
    }

    fn save_as(&self, state: &Arc<State>, app: &gtk::Application, k: AssetKey<'static>, p: PathBuf) {
        trace!(self.log, "save as");
        let logger = self.log.clone();
        let future = self.pool_off.spawn_fn(with!(state => move || {
                let mut a = state.actor_off.lock().unwrap();
                a.save_asset_as(&k, &p).map_err(From::from)
            }))
            .map_err(with!(app => move |e| {
                error!(logger, "failed to save"; "error" => ?e);
                show_error(&app, &e, "Failed to save file", "");
            }));
        self.futures.send(boxed(future)).unwrap()
    }

    //
    // Futures
    //

    fn set_image(&self, state: &Arc<State>, ast: coax_data::Asset<'static>, c: ConvId, m: String, img: gtk::DrawingArea) -> impl Future<Item=(), Error=Error> {
        let future =
            if ast.status == AssetStatus::Local {
                let aid = ast.id.clone();
                self.pool_off.spawn_fn(with!(state => move || {
                    let mut a = state.actor_off.lock().unwrap();
                    Ok(a.asset_path(&aid))
                }))
            } else {
                self.pool_on.spawn_fn(with!(state => move || {
                    let mut actor_guard = state.actor_on.lock().unwrap();
                    if let Some(ref mut a) = *actor_guard {
                        a.download_asset(&ast.id, ast.token.as_ref())?;
                        a.decrypt_asset(&ast.id, &ast.cksum, &ast.key)?;
                        Ok(a.asset_path(&ast.id))
                    } else {
                        Err(Error::InvalidAppState)
                    }
                }))
            };
        let channels = self.channels.clone();
        future.map(move |path| {
            if let Some(ch) = channels.get(&c) {
                ch.get_msg(&m).map(|msg| if let Message::Image(ref m) = *msg { m.stop_spinner() });
            }
            let buf    = Pixbuf::new_from_file(path.to_string_lossy().as_ref()).unwrap(); // TODO
            let width  = buf.get_width();
            let height = buf.get_height();
            let w      = img.get_allocated_width();
            let h      = w * height / width;
            img.set_size_request(-1, h);
            img.connect_draw(move |img, ctx| {
                let w = img.get_allocated_width();
                let h = w * height / width;
                if w < width {
                    img.set_size_request(-1, h);
                    let b = buf.scale_simple(w, h, InterpType::Bilinear).unwrap(); // TODO
                    ctx.set_source_pixbuf(&b, 0.0, 0.0);
                } else {
                    img.set_size_request(-1, height);
                    ctx.set_source_pixbuf(&buf, 0.0, 0.0)
                }
                ctx.paint();
                gtk::Inhibit(false)
            });
        })
    }

    fn set_user_icon(&self, state: &Arc<State>, u: UserId) -> impl Future<Item=(), Error=Error> {
        let is_online = state.is_online.load(Ordering::Relaxed);
        debug!(self.log, "set user icon"; "id" => %u, "online" => is_online);
        let future =
            if is_online {
                self.pool_on.spawn_fn(with!(state, u => move || {
                    let mut actor_guard = state.actor_on.lock().unwrap();
                    if let Some(ref mut a) = *actor_guard {
                        if let Some(usr) = a.resolve_user(&u, true)? {
                            a.load_user_icon(&usr).map_err(From::from)
                        } else {
                            Ok(Vec::new())
                        }
                    } else {
                        Err(Error::InvalidAppState)
                    }
                }))
            } else {
                self.pool_off.spawn_fn(with!(state, u => move || {
                    let mut a = state.actor_off.lock().unwrap();
                    if let Some(usr) = a.load_user(&u)? {
                        a.load_user_icon(&usr).map_err(From::from)
                    } else {
                        Ok(Vec::new())
                    }
                }))
            };
        let this = self.clone();
        future.map(move |data| {
            if data.is_empty() {
                info!(this.log, "no user icon"; "user" => %u);
                return ()
            }
            if let Some(mut user) = this.resources.user_mut(&u) {
                user.set_icon(&data)
            } else {
                warn!(this.log, "no user resources"; "user" => %u)
            };
        })
    }

    fn send_confirmation(&self, state: &Arc<State>, c: &ConvId, id: &str) -> impl Future<Item=(), Error=Error> {
        debug!(self.log, "send confirmation"; "conv" => %c, "msg" => id);
        let msg  = MsgBuilder::new().delivered(id).finish();
        let this = self.clone();
        self.prepare_message(state, c, msg, Delivery::OneShot)
            .and_then(with!(this, state => move |(m, p)| {
                this.send(&state, p, m, Delivery::OneShot).map(|_| ())
            }))
    }

    fn send_message(&self, state: &Arc<State>, id: &ConvId, msg: GenericMessage) -> impl Future<Item=(), Error=Error> {
        debug!(self.log, "send message"; "conv" => %id, "id" => msg.get_message_id());
        let this = self.clone();
        let mid  = String::from(msg.get_message_id());
        let text = String::from(msg.get_text().get_content());
        future::lazy(with!(this, state, id, mid => move || {
                if let Some(ch) = this.channels.get(&id) {
                    this.ensure_user_res(&state, &state.me, false);
                    let mut usr = this.resources.user_mut(&state.me.id).unwrap();
                    if !ch.has_msg(&mid) {
                        let msg = TextMessage::new(None, &mut usr, &text);
                        msg.start_spinner();
                        ch.push_msg(&mid, Message::Text(msg))
                    }
                }
                future::ok(())
            }))
            .and_then(with!(this, state, id => move |()| {
                this.prepare_message(&state, &id, msg, Delivery::Persistent)
            }))
            .and_then(with!(this, state => move |(msg, params)| {
                this.send(&state, params, msg, Delivery::Persistent)
            }))
            .map(with!(this, id, mid => move |dt| {
                if let Some(ch) = this.channels.get(&id) {
                    let loc_time   = dt.with_timezone(&this.timezone);
                    let is_message =
                        if let Some(mut m) = ch.get_msg_mut(&mid) {
                            if let Message::Text(ref mut msg) = *m {
                                msg.stop_spinner();
                                msg.set_time(loc_time.clone());
                                true
                            } else {
                                false
                            }
                        } else {
                            false
                        };
                    if is_message {
                        ch.insert_delivery_date(&mid, loc_time.date());
                        ch.update_time(&loc_time);
                        this.convlist.invalidate_sort()
                    }
                };
            }))
            .map_err(with!(this, id, mid => move |e| {
                if let Some(ch) = this.channels.get(&id) {
                    if let Some(m) = ch.get_msg(&mid) {
                        if let Message::Text(ref msg) = *m {
                            msg.set_error()
                        }
                    }
                }
                e
            }))
    }

    fn load_local_conversations(&self, state: &Arc<State>, app: &gtk::Application) -> impl Future<Item=(), Error=Error> {
        let this   = self.clone();
        let pstate = state.ch_state.lock().unwrap().clone();
        debug!(self.log, "load conversations"; "paging_state" => ?pstate);
        self.pool_off.spawn_fn(with!(state => move || {
                let mut a = state.actor_off.lock().unwrap();
                a.load_conversations(pstate, 64).map_err(From::from)
            }))
            .map(with!(state, app => move |page| {
                for c in page.data {
                    this.on_conversation(&state, &app, c)
                }
                *state.ch_state.lock().unwrap() = Some(page.state);
            }))
    }

    fn load_remote_conversations(&self, state: &Arc<State>, app: &gtk::Application) -> impl Future<Item=(), Error=Error> {
        debug!(self.log, "load remote conversations");
        let this = self.clone();
        self.pool_on.spawn_fn(with!(state => move || {
                let mut actor_guard = state.actor_on.lock().unwrap();
                if let Some(ref mut a) = *actor_guard {
                    a.resolve_conversations().map_err(From::from)
                } else {
                    Err(Error::InvalidAppState)
                }
            }))
            .and_then(with!(this, state, app => move |()| {
                this.load_local_conversations(&state, &app)
            }))
    }

    fn load_messages(&self, state: &Arc<State>, app: &gtk::Application, cid: &ConvId) -> impl Future<Item=(), Error=Error> {
        let this    = self.clone();
        let conv_id = cid.clone();
        let pstate  = self.channels.get(&cid).and_then(|ch| ch.paging_state());
        debug!(self.log, "load conversation messages"; "id" => %cid, "paging_state" => ?pstate);
        self.pool_off.spawn_fn(with!(state => move || {
                let mut a = state.actor_off.lock().unwrap();
                a.load_messages(&conv_id, pstate, 32).map_err(From::from)
            }))
            .map(with!(this, state, cid, app => move |mm| {
                if let Some(chan) = this.channels.get(&cid) {
                    for m in mm.data {
                        if chan.has_msg(&m.id) {
                            continue
                        }
                        this.ensure_user_res(&state, &m.user, false);
                        let local   = m.time.with_timezone(&this.timezone);
                        let mut usr = this.resources.user_mut(&m.user.id).unwrap();
                        let message = match m.data {
                            MessageData::Text(txt) => {
                                let mut msg = TextMessage::new(None, &mut usr, &txt);
                                if m.status == MessageStatus::Created {
                                    msg.set_error()
                                } else {
                                    msg.set_time(local)
                                }
                                Message::Text(msg)
                            }
                            MessageData::Asset(ast) => {
                                let img = gtk::DrawingArea::new();
                                let msg = Image::new(local, &mut usr, img.clone(), app.get_active_window());
                                let aid = ast.id.clone();
                                msg.signal_save().connect(with!(this, state, app => move |p| {
                                    this.save_as(&state, &app, aid.clone(), p.clone())
                                }));
                                msg.start_spinner();
                                let future = this.set_image(&state, ast, cid.clone(), m.id.clone(), img)
                                    .map_err(with!(this => move |e| {
                                        error!(this.log, "failed to set image"; "error" => ?e)
                                    }));
                                this.futures.send(boxed(future)).unwrap();
                                Message::Image(msg)
                            }
                            MessageData::MemberJoined(None) =>
                                Message::system(local, &format!("{} has joined this conversation.", usr.name)),
                            MessageData::MemberJoined(Some(m)) =>
                                Message::system(local, &format!("{} has added {} to this conversation.", usr.name, m.name.as_str())),
                            MessageData::MemberLeft(None) =>
                                Message::system(local, &format!("{} has left this conversation.", usr.name)),
                            MessageData::MemberLeft(Some(m)) =>
                                Message::system(local, &format!("{} has removed {} from this conversation.", usr.name, m.name.as_str()))
                        };
                        chan.push_front_msg(&m.id, message)
                    }
                    chan.set_paging_state(mm.state);
                    chan.set_init()
                };
            }))
    }

    fn load_local_contacts(&self, state: &Arc<State>, app: &gtk::Application) -> impl Future<Item=(), Error=Error> {
        trace!(self.log, "load contacts");
        let this  = self.clone();
        self.pool_off.spawn_fn(with!(state => move || {
                let mut a = state.actor_off.lock().unwrap();
                a.load_contacts().map_err(From::from)
            }))
            .map(with!(state, app => move |cc| {
                for (u, c) in cc {
                    this.on_contact(&state, &app, u, c)
                }
                this.contacts.set_init()
            }))
    }

    fn load_remote_contacts(&self, state: &Arc<State>, app: &gtk::Application) -> impl Future<Item=(), Error=Error> {
        debug!(self.log, "load remote contacts");
        let this  = self.clone();
        self.pool_on.spawn_fn(with!(state => move || {
                let mut actor_guard = state.actor_on.lock().unwrap();
                if let Some(ref mut a) = *actor_guard {
                    a.resolve_user_connections().map_err(From::from)
                } else {
                    Err(Error::InvalidAppState)
                }
            }))
            .and_then(with!(this, state, app => move |()| {
                this.load_local_contacts(&state, &app)
            }))
    }

    fn prepare_message(&self, state: &Arc<State>, id: &ConvId, msg: GenericMessage, del: Delivery) -> impl Future<Item=(GenericMessage, send::Params), Error=Error> {
        debug!(self.log, "prepare message future"; "conv" => %id, "id" => msg.get_message_id());
        self.pool_off.spawn_fn(with!(state, id => move || {
            let mut a = state.actor_off.lock().unwrap();
            if del != Delivery::OneShot {
                a.store_message(&id, &msg)?;
            }
            let p = a.prepare_message(&id, &msg)?;
            a.enqueue(msg.get_message_id().as_bytes(), &p, &msg)?;
            Ok((msg, p))
        }))
    }

    fn send(&self, state: &Arc<State>, mut params: send::Params, msg: GenericMessage, del: Delivery) -> impl Future<Item=DateTime<UTC>, Error=Error> {
        debug!(self.log, "send future"; "conv" => %params.conv, "id" => msg.get_message_id());
        let logger = self.log.clone();
        self.pool_on.spawn_fn(with!(state => move || {
            loop {
                {
                    let mut actor_guard = state.actor_on.lock().unwrap();
                    if let Some(ref mut a) = *actor_guard {
                        match a.send_message(&mut params, &msg, del) {
                            Ok(dt) => {
                                a.dequeue(msg.get_message_id().as_bytes(), &params.conv)?;
                                return Ok(dt)
                            }
                            Err(e@coax_actor::Error::MsgSend(ClientError::Error(send::Error::NotFound))) => {
                                a.dequeue(msg.get_message_id().as_bytes(), &params.conv)?;
                                return Err(e.into())
                            }
                            Err(e) =>
                                error!(logger, "failed to send message"; "id" => msg.get_message_id(), "error" => %e)
                        }
                    } else {
                        return Err(Error::InvalidAppState)
                    }
                }
                thread::sleep(Duration::from_secs(3))
            }
        }))
    }

    fn resend_messages(&self, state: &Arc<State>) -> impl Future<Item=(), Error=Error> {
        trace!(self.log, "re-send messages future");
        self.pool_on.spawn_fn(with!(state => move || {
            let mut actor_guard = state.actor_on.lock().unwrap();
            if let Some(ref mut a) = *actor_guard {
                a.resend().map_err(From::from)
            } else {
                Err(Error::InvalidAppState)
            }
        }))
    }

    fn conversation(&self, state: &Arc<State>, id: &ConvId) -> impl Future<Item=Option<Conversation<'static>>, Error=Error> {
        trace!(self.log, "load conversation future");
        self.pool_on.spawn_fn(with!(state, id => move || {
            let mut actor_guard = state.actor_on.lock().unwrap();
            if let Some(ref mut a) = *actor_guard {
                a.resolve_conversation(&id).map_err(From::from)
            } else {
                Err(Error::InvalidAppState)
            }
        }))
    }

    fn user(&self, state: &Arc<State>, id: UserId, allow_local: bool) -> impl Future<Item=Option<User<'static>>, Error=Error> {
        trace!(self.log, "user future");
        if allow_local {
            self.pool_off.spawn_fn(with!(state => move || {
                let mut a = state.actor_off.lock().unwrap();
                a.load_user(&id).map_err(From::from)
            }))
        } else {
            self.pool_on.spawn_fn(with!(state => move || {
                let mut actor_guard = state.actor_on.lock().unwrap();
                if let Some(ref mut a) = *actor_guard {
                    a.resolve_user(&id, false).map_err(From::from)
                } else {
                    Err(Error::InvalidAppState)
                }
            }))
        }
    }

    fn notifications(&self, state: &Arc<State>, initial: bool) -> impl Future<Item=(), Error=Error> {
        trace!(self.log, "notifications future");
        let logger = self.log.clone();
        self.pool_on.spawn_fn(with!(state => move || {
            let mut actor_guard = state.actor_on.lock().unwrap();
            if let Some(ref mut a) = *actor_guard {
                state.is_sync.store(true, Ordering::Relaxed);
                loop {
                    debug!(logger, "actor getting notifications");
                    match a.notifications(!initial) {
                        Ok(true)  => {}
                        Ok(false) => {
                            state.is_sync.store(false, Ordering::Relaxed);
                            break
                        }
                        Err(e) => {
                            state.is_sync.store(false, Ordering::Relaxed);
                            return Err(e.into())
                        }
                    }
                }
                Ok(())
            } else {
                Err(Error::InvalidAppState)
            }
        }))
    }

    //
    // Misc
    //

    fn show_new_conv(&self, state: &Arc<State>, app: &gtk::Application) {
        trace!(self.log, "show_new_conv");
        let this = self.clone();
        let builder = Builder::new_from_string(include_str!("gtk/new-conversation.ui"));
        let window: Window = builder.get_object("new-conv-window").unwrap();
        let submit: Button = builder.get_object("submit-button").unwrap();
        let cancel: Button = builder.get_object("cancel-button").unwrap();
        cancel.connect_clicked(with!(window => move |_| window.hide()));

        let name: gtk::Entry = builder.get_object("name-entry").unwrap();
        let user: gtk::Entry = builder.get_object("user-entry").unwrap();

        submit.connect_clicked(with!(this, state, app, window, name, user => move |_| {
            window.hide();
            if let Some(u) = user.get_text().and_then(|s| UserId::from_str(&s)) {
                this.on_new_conv(&state, &app, name.get_text().unwrap_or("N/A".into()), u)
            } else {
                show_message(&app, MessageType::Error, "Invalid UserId", "", None)
            }
        }));

        window.set_transient_for(app.get_active_window().as_ref());
        window.show_all();
    }

    fn ensure_user_res(&self, state: &Arc<State>, u: &User, reload: bool) {
        if self.resources.has_user(&u.id) && !reload {
            return ()
        }
        debug!(self.log, "adding user resources"; "user" => %u.id);
        self.resources.add_user(u);
        let logger = self.log.clone();
        let future = self.set_user_icon(&state, u.id.clone())
            .map_err(move |e| {
                error!(logger, "failed to set user icon"; "error" => ?e)
            });
        self.futures.send(boxed(future)).unwrap()
    }

    fn show_info(&self, txt: &str) {
        self.info.set_markup(txt);
        self.revealer.set_reveal_child(true)
    }

    fn hide_info(&self) {
        self.revealer.set_reveal_child(false)
    }

    fn show_notification(&self, app: &gtk::Application, u: &res::User, m: &coax_data::Message) {
        if app.get_active_window().as_ref().map(|w| w.has_toplevel_focus()).unwrap_or(false) {
            return ()
        }
        if let MessageData::Text(ref txt) = m.data {
            Notification::new()
                .appname("coax")
                .summary(&format!("New message from {}", u.name))
                .body(&txt.chars().take(128).collect::<String>())
                .icon("coax")
                .hint(NotificationHint::Category("im.received".into()))
                .show()
                .map(|_| ())
                .unwrap_or_else(|e| {
                    warn!(self.log, "error showing system notification"; "error" => ?e)
                })
        }
    }
}

fn set_subtitle(app: &gtk::Application, s: Option<&str>) {
    app.get_active_window()
       .and_then(|w| w.get_titlebar())
       .and_then(|t| t.downcast::<gtk::HeaderBar>().ok())
       .map(|h| h.set_subtitle(s));
}

fn show_error(app: &gtk::Application, e: &Error, msg: &str, sec: &str) {
    let details = format!("{}", e);
    show_message(app, MessageType::Error, msg, sec, Some(&details))
}

fn show_message(app: &gtk::Application, mtype: MessageType, msg: &str, sec: &str, details: Option<&str>) {
    let win = app.get_active_window();
    let flags = gtk::DIALOG_MODAL;
    let dialog = MessageDialog::new(win.as_ref(), flags, mtype, ButtonsType::Close, msg);
    dialog.set_secondary_text(Some(sec));
    if let Some(d) = details {
        let label = gtk::Label::new(Some(d));
        label.set_max_width_chars(64);
        label.set_line_wrap(true);
        label.set_margin_left(12);
        label.set_margin_top(6);
        label.set_margin_right(12);
        label.set_margin_bottom(6);
        let exp = gtk::Expander::new(Some("Details"));
        exp.add(&label);
        exp.set_expanded(false);
        dialog.get_content_area().add(&exp)
    }
    dialog.connect_response(|d, _| d.hide());
    dialog.show_all();
    dialog.run();
}

#[inline]
fn boxed<'a, F: Future + 'a>(f: F) -> Box<Future<Item=F::Item, Error=F::Error> + 'a> {
    Box::new(f)
}

