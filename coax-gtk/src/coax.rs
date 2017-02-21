use std;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::CString;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use channel::{Channel, Message};
use chrono::{DateTime, Local, UTC};
use coax_actor::{Actor, Error, Pkg};
use coax_actor::actor::{Init, Connected, Offline, Online};
use coax_actor::config;
use coax_api::conv::ConvType;
use coax_api::message::send;
use coax_api::types::{Label, Name, Email, Password, UserHandle, UserId, ConvId};
use coax_api::user::{self, ConnectStatus};
use coax_api_proto::{Builder as MsgBuilder, GenericMessage};
use coax_data::{self, User, Conversation, Connection, MessageData, MessageStatus};
use coax_data::db::{self, Page, PagingState};
use coax_data::profiles::ProfileDb;
use coax_net::http::tls;
use contact::Contacts;
use ffi;
use futures::{self, Future};
use futures_spawn::SpawnHelper;
use futures_threadpool::{self as pool, ThreadPool};
use gio::{self, MenuModel, SimpleAction};
use glib_sys;
use gtk::prelude::*;
use gtk::{self, Builder, Button, MenuButton, HeaderBar, Window};
use gtk::{MessageDialog, MessageType, ButtonsType, Orientation};
use poll::Loop;
use profile;
use res;
use slog::Logger;

enum Io {
    Init(Actor<Init>),
    Connected(Actor<Connected>),
    Offline(Actor<Offline>),
    Online(Actor<Online>)
}

#[derive(Clone)]
pub struct Coax {
    log:      Logger,
    pool_act: ThreadPool, // interactive threads
    pool_rem: ThreadPool, // sending & receiving threads (remote)
    pool_loc: ThreadPool, // local threads
    futures:  Sender<Box<Future<Item=(), Error=()>>>,
    profiles: Arc<Mutex<ProfileDb>>,
    builder:  gtk::Builder,
    info:     gtk::Label,
    revealer: gtk::Revealer,
    mainview: gtk::Grid,
    convlist: gtk::ListBox,
    send_btn: gtk::Button,
    timezone: Local,
    channels: Rc<RefCell<HashMap<ConvId, Channel>>>,
    contacts: Rc<RefCell<Contacts>>,
    me:       Rc<RefCell<User<'static>>>,
    me_pict:  Rc<RefCell<gtk::Image>>,
    res:      Rc<RefCell<res::Resources>>,
    actor:    Arc<Mutex<Option<Io>>>,
    sync:     Arc<Mutex<Option<Actor<Online>>>>,
    inbox:    Arc<Mutex<Option<JoinHandle<()>>>>
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
        let actor    = Actor::new(&log, cfg, tls)?;
        let builder  = Builder::new_from_string(include_str!("gtk/main.ui"));
        let info     = builder.get_object("info-label").unwrap();
        let sendbtn  = builder.get_object("send-button").unwrap();
        let mainview = builder.get_object("mainview").unwrap();

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

        let usr = User::new(UserId::rand(), Name::new(""));

        let coax = Coax {
            log:      log,
            pool_act: pool::Builder::new().pool_size(1).name_prefix("act-").create(),
            pool_rem: pool::Builder::new().pool_size(1).name_prefix("rem-").create(),
            pool_loc: pool::Builder::new().pool_size(1).name_prefix("loc-").create(),
            futures:  tx,
            profiles: Arc::new(Mutex::new(pdb)),
            builder:  builder,
            info:     info,
            revealer: revealer,
            mainview: mainview,
            convlist: convlist,
            send_btn: sendbtn,
            timezone: Local::now().timezone(),
            contacts: Rc::new(RefCell::new(Contacts::new())),
            channels: Rc::new(RefCell::new(HashMap::new())),
            me:       Rc::new(RefCell::new(usr)),
            me_pict:  Rc::new(RefCell::new(gtk::Image::new())),
            res:      Rc::new(RefCell::new(res::Resources::new())),
            actor:    Arc::new(Mutex::new(Some(Io::Init(actor)))),
            sync:     Arc::new(Mutex::new(None)),
            inbox:    Arc::new(Mutex::new(None))
        };

        app.connect_startup(Coax::startup);
        app.connect_activate(move |app| coax.activate(app));
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

    fn activate(&self, app: &gtk::Application) {
        trace!(self.log, "activate");
        let this = self.clone();

        let window = gtk::ApplicationWindow::new(app);
        window.set_size_request(800, 600);

        let main: gtk::Box = self.builder.get_object("main").unwrap();

        let conv_bar: gtk::Toolbar    = self.builder.get_object("conv-toolbar").unwrap();
        let new_conv: gtk::ToolButton = self.builder.get_object("add-conv-button").unwrap();
        new_conv.connect_clicked(with!(this, app => move |_| this.show_new_conv(&app)));

        let show_contacts: gtk::ToolButton = self.builder.get_object("show-cons-button").unwrap();
        show_contacts.connect_clicked(with!(this, app => move |_| {
            if !this.contacts.borrow().is_init() {
                this.load_contacts(&app)
            }
            this.send_btn.set_sensitive(false);
            this.convlist.unselect_all();
            this.mainview.remove_row(0);
            this.mainview.insert_row(0);
            this.mainview.attach(this.contacts.borrow().contact_view(), 0, 0, 1, 1);
            this.mainview.show_all()
        }));

        let header_builder = Builder::new_from_string(include_str!("gtk/header.ui"));
        let bar: HeaderBar = header_builder.get_object("header").unwrap();
        let menu: MenuButton = header_builder.get_object("menu-button").unwrap();

        let profile_menu: gtk::MenuButton = header_builder.get_object("profile-menu").unwrap();
        profile_menu.set_sensitive(false);

        let profile_name: gtk::Label = header_builder.get_object("profile-name-label").unwrap();

        {
            *self.me_pict.borrow_mut() = header_builder.get_object("profile-image").unwrap()
        }

        let menu_builder = Builder::new_from_string(include_str!("gtk/button-menu.ui"));
        let model: MenuModel = menu_builder.get_object("button-menu").unwrap();
        menu.set_menu_model(Some(&model));

        // Open menu action

        let open = SimpleAction::new("open", None);
        open.connect_activate(with!(this, app, window, profile_name, profile_menu, conv_bar => move |open, _| {
            let builder = Builder::new_from_string(include_str!("gtk/open-account.ui"));
            let notebook: gtk::Notebook = builder.get_object("open-notebook").unwrap();
            let dialog: Window = builder.get_object("open-account-window").unwrap();
            dialog.set_transient_for(Some(&window));

            let cancel: Button = builder.get_object("cancel-button").unwrap();
            cancel.connect_clicked(with!(dialog => move |_| dialog.hide()));

            let submit: Button = builder.get_object("submit-button").unwrap();
            submit.set_sensitive(false);

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
                    submit.set_sensitive(value);
                })
            };

            submit.connect_clicked(with!(this, app, open, dialog, profile_name, profile_menu, conv_bar => move |_| {
                dialog.hide();
                let enable = vec![profile_menu.clone().upcast::<gtk::Widget>(), conv_bar.clone().upcast::<gtk::Widget>()];
                match notebook.get_current_page() {
                    Some(0) => {
                        let row = from_some!(profiles_list.get_selected_row());
                        let id = ffi::get_data(&row, &ffi::KEY_ID);
                        with! { profile_name, open =>
                            this.on_profile(&app, profile_name, open, enable, id.cloned())
                        }
                    }
                    Some(1) => {
                        let email = login_email.get_text().unwrap_or(String::new());
                        let pass = login_pass.get_text().unwrap_or(String::new());
                        with! { profile_name, open =>
                            this.on_login(&app, profile_name, open, enable, Email::new(email), Password::new(pass))
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

            dialog.show_all()
        }));

        // Find button

        let find_button: gtk::ToggleButton = header_builder.get_object("find-toggle-button").unwrap();
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

        self.convlist.connect_row_selected(with!(this, app => move |_, row| {
            if let Some(r) = row.as_ref() {
                ffi::get_data(r, &ffi::KEY_ID).map(|id| {
                    if let Some(ch) = this.channels.borrow().get(id) {
                        if !ch.is_init() {
                            this.load_messages(&app, id)
                        }
                        this.mainview.remove_row(0);
                        this.mainview.insert_row(0);
                        this.mainview.attach(ch.message_view(), 0, 0, 1, 1);
                        this.mainview.show_all();
                        this.send_btn.set_sensitive(true)
                    } else {
                        this.send_btn.set_sensitive(false)
                    }
                });
            } else {
                this.send_btn.set_sensitive(false)
            }
        }));

        let input: gtk::TextView = self.builder.get_object("main-text-input").unwrap();

        let send   = SimpleAction::new("send", None);
        let button = self.send_btn.clone();
        send.connect_activate(with!(this => move |_, _| {
            if !button.is_sensitive() {
                return ()
            }
            let row = this.convlist.get_selected_row();
            let cid = from_some!(row.as_ref().and_then(|r| ffi::get_data(r, &ffi::KEY_ID)));
            let buf = from_some!(input.get_buffer());
            let (mut s, mut e) = buf.get_bounds();
            let txt = from_some!(buf.get_text(&s, &e, false));
            let msg = MsgBuilder::new().text(txt).finish();
            this.send_message(cid, msg);
            buf.delete(&mut s, &mut e);
        }));
        self.send_btn.connect_clicked(with!(send => move |_| send.activate(None)));

        window.add_action(&open);
        window.add_action(&send);
        app.set_accels_for_action("win.send", &["<Shift>Return"]);

        window.add(&main);
        window.set_titlebar(Some(&bar));

        window.show_all()
    }

    fn setup_profiles(&self, app: &gtk::Application, submit: &Button, list: &gtk::ListBox) {
        trace!(self.log, "setup_profiles");
        let profiles =
            match profile::load_profiles(&*self.profiles.lock().unwrap()) {
                Ok(pp) => pp,
                Err(e) => {
                    let details = format!("{}", e);
                    error!(self.log, "error loading profiles"; "error" => details);
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
    }

    fn proceed_registration(email: &gtk::Entry, name: &gtk::Entry, pass: &gtk::Entry) -> bool {
        3 <= name.get_text_length() && 5 <= email.get_text_length() && 8 <= pass.get_text_length()
    }

    fn setup_register_entry_handlers(submit: &Button, email: &gtk::Entry, name: &gtk::Entry, pass: &gtk::Entry) {
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

    fn setup_login_entry_handlers(submit: &Button, email: &gtk::Entry, pass: &gtk::Entry) {
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
        debug!(self.log, "on_register"; "e-mail" => e.as_str());
        let this   = self.clone();
        let actor  = self.actor.clone();
        let future =
            self.pool_act.spawn_fn(move || {
                let mut act = actor.lock().unwrap();
                Coax::ensure_connected(&mut *act)?;
                let params = user::register::Params::new(UserHandle::Email(e), n, p);
                if let Some(Io::Connected(ref mut a)) = *act {
                    a.register_user(&params)
                } else {
                    Err(Error::Message("invalid app state"))
                }
            })
            .map(with!(app, this => move |()| {
                this.hide_info();
                let txt = "Please check your e-mail account and verify your address.";
                show_message(&app, MessageType::Info, "Account registered", txt, None);
            }))
            .map_err(with!(app, this => move |e| {
                this.hide_info();
                error!(this.log, "failed to register account"; "error" => format!("{}", e));
                show_error(&app, e, "Failed to register account", "")
            }));
        self.show_info("Registering your account ...");
        self.futures.send(Box::new(future)).unwrap()
    }

    fn on_login(&self, app: &gtk::Application, profile: gtk::Label, disable: SimpleAction, enable: Vec<gtk::Widget>, e: Email<'static>, p: Password<'static>) {
        debug!(self.log, "on_login"; "e-mail" => e.as_str());
        let (bcast_tx, bcast_rx) = std::sync::mpsc::channel();

        let this   = self.clone();
        let actor  = self.actor.clone();
        let sync   = self.sync.clone();
        let inbox  = self.inbox.clone();
        let profs  = self.profiles.clone();
        let future =
            self.pool_act.spawn_fn(move || {
                let mut act  = actor.lock().unwrap();
                Coax::ensure_connected(&mut *act)?;
                let params = user::login::Params::new(UserHandle::Email(e), p, Label::new("coax-gtk"));
                if let Some(Io::Connected(mut a)) = act.take() {
                    match a.login(&params) {
                        Ok(usr) => {
                            let a2 = a.online(usr, bcast_tx);
                            let me = a2.me().clone();
                            *act = Some(Io::Online(a2));
                            profs.lock().unwrap().insert(&me)?
                        }
                        Err(e) => {
                            *act = Some(Io::Connected(a));
                            return Err(e)
                        }
                    }
                } else {
                    return Err(Error::Message("invalid app state"))
                }
                if let Some(Io::Online(ref mut a)) = *act {
                    let mut i = a.new_inbox()?;
                    let     w = i.connect()?;
                    *inbox.lock().unwrap() = Some(i.fork(w));
                    *sync.lock().unwrap() = Some(a.clone()?);
                    Ok(a.me().clone())
                } else {
                    Err(Error::Message("invalid app state"))
                }
            })
            .map(with!(this, app => move |me| {
                let name = ffi::escape(me.name.as_str()).to_string_lossy();
                set_subtitle(&app, Some(me.name.as_str()));
                this.res.borrow_mut().add_user(&me);
                this.set_user_icon(me.id.clone());
                *this.me.borrow_mut() = me;
                profile.set_markup(&format!("<big><b>{}</b></big>", name));
                disable.set_enabled(false);
                for e in &enable {
                    e.set_sensitive(true)
                }
                gtk::timeout_add(500, with!(this, app => move || {
                    for pkg in bcast_rx.try_iter() {
                        this.on_incoming(&app, pkg);
                    }
                    Continue(true)
                }));
                this.load_local_conversations(&app);
                this.load_remote_conversations(&app)
            }))
            .and_then(with!(this => move |()| {
                this.pool_rem.spawn(this.notifications(true))
            }))
            .map(with!(this => move |_| {
                this.hide_info();
                this.ensure_user_res(&*this.me.borrow());
                let mut res = this.res.borrow_mut();
                *this.me_pict.borrow_mut() = res.user_mut(&this.me.borrow().id).unwrap().pict();
            }))
            .map_err(with!(app, this => move |e| {
                this.hide_info();
                error!(this.log, "failed to login"; "error" => format!("{}", e));
                show_error(&app, e, "Failed to login", "")
            }));
        self.show_info("Signing in ...");
        self.futures.send(Box::new(future)).unwrap()
    }

    fn on_profile(&self, app: &gtk::Application, profile: gtk::Label, disable: SimpleAction, enable: Vec<gtk::Widget>, uid: Option<UserId>) {
        debug!(self.log, "on_profile"; "user" => uid.as_ref().map(UserId::to_string).unwrap_or(String::new()));
        let user_id =
            if let Some(u) = uid {
                u
            } else {
                show_message(app, MessageType::Error, "Invalid User-ID", "", None);
                return ()
            };

        let (bcast_tx, bcast_rx) = std::sync::mpsc::channel();

        let this   = self.clone();
        let logger = self.log.clone();
        let actor  = self.actor.clone();
        let sync   = self.sync.clone();
        let inbox  = self.inbox.clone();
        let future =
            self.pool_act.spawn_fn(with!(actor => move || {
                let mut act = actor.lock().unwrap();
                if let Some(Io::Init(mut a)) = act.take() {
                    match a.profile(&user_id) {
                        Ok(usr) => {
                            let a2 = a.offline(usr, bcast_tx);
                            let me = a2.me().clone();
                            *act = Some(Io::Offline(a2));
                            Ok(me)
                        }
                        Err(e) => {
                            *act = Some(Io::Init(a));
                            Err(e)
                        }
                    }
                } else {
                    Err(Error::Message("invalid app state"))
                }
            }))
            .map(with!(this, app => move |me| {
                let name = ffi::escape(me.name.as_str()).to_string_lossy();
                set_subtitle(&app, Some(me.name.as_str()));
                this.res.borrow_mut().add_user(&me);
                this.set_user_icon(me.id.clone());
                *this.me.borrow_mut() = me;
                profile.set_markup(&format!("<big><b>{}</b></big>", name));
                disable.set_enabled(false);
                for e in &enable {
                    e.set_sensitive(true)
                }
                gtk::timeout_add(500, with!(this, app => move || {
                    for pkg in bcast_rx.try_iter() {
                        this.on_incoming(&app, pkg);
                    }
                    Continue(true)
                }));
                this.load_local_conversations(&app)
            }))
            .and_then(with!(this => move |_| {
                this.pool_act.spawn_fn(move || {
                    let mut act = actor.lock().unwrap();
                    Coax::ensure_online(&mut *act)?;
                    if let Some(Io::Online(ref mut a)) = *act {
                        let mut i = a.new_inbox()?;
                        let     w = i.connect()?;
                        *inbox.lock().unwrap() = Some(i.fork(w));
                        *sync.lock().unwrap() = Some(a.clone()?);
                        Ok(())
                    } else {
                        Err(Error::Message("invalid app state"))
                    }
                })
            }))
            .and_then(with!(this => move |_| {
                this.pool_rem.spawn(this.notifications(true))
            }))
            .map(with!(this => move |_| {
                this.hide_info()
            }))
            .and_then(with!(this => move |_| {
                this.ensure_user_res(&*this.me.borrow());
                let mut res = this.res.borrow_mut();
                *this.me_pict.borrow_mut() = res.user_mut(&this.me.borrow().id).unwrap().pict();
                this.pool_rem.spawn(this.resend_messages())
            }))
            .map_err(with!(app => move |e| {
                error!(logger, "failed to activate profile"; "error" => format!("{}", e));
                show_error(&app, e, "Failed to activate profile", "")
            }));
        self.show_info("Signing in ...");
        self.futures.send(Box::new(future)).unwrap()
    }

    //
    // Callbacks
    //

    fn on_incoming(&self, app: &gtk::Application, pkg: Pkg) -> bool {
        trace!(self.log, "on_incoming");
        match pkg {
            Pkg::Connected => {
                let logger = self.log.clone();
                let future = self.pool_rem.spawn(self.notifications(false))
                    .map_err(with!(logger => move |e| {
                        error!(logger, "failed to get notifications"; "error" => format!("{}", e))
                    }));
                self.futures.send(Box::new(future)).unwrap();
                self.hide_info()
            }
            Pkg::Disconnected              => self.show_info("Connection lost. Reconnecting ..."),
            Pkg::Message(m)                => self.on_message(m),
            Pkg::MessageUpdate(c, m, t, s) => self.on_message_update(m, c, t, s),
            Pkg::Conversation(c)           => self.on_conversation(c),
            Pkg::Contact(u, c)             => self.on_contact(app, u, c),
            Pkg::MembersAdd(t, c, m)       => self.on_members_add(app, t, c, m),
            Pkg::Fin                       => return true
        }
        false
    }

    fn on_message(&self, m: coax_data::Message<'static>) {
        debug!(self.log, "on_message"; "conv" => m.conv.to_string(), "id" => m.id);
        let this   = self.clone();
        let logger = self.log.clone();
        if let Some(mut ch) = self.channels.borrow_mut().get_mut(&m.conv) {
            if !ch.has_msg(&m.id) {
                self.ensure_user_res(&m.user);
                let mtime = m.time.with_timezone(&self.timezone);
                if mtime.date() != ch.newest_date() {
                    ch.add(Message::date(mtime.date()))
                }
                let mut res = self.res.borrow_mut();
                let mut usr = res.user_mut(&m.user.id).unwrap();
                match m.data {
                    MessageData::Text(ref txt) => {
                        let msg = Message::text(Some(mtime), &mut usr, txt);
                        ch.push_msg(&m.id, msg)
                    }
                    MessageData::MemberJoined => {
                        let txt = format!("{} has joined this conversation.", usr.name);
                        let msg = Message::text(Some(mtime), &mut usr, &txt);
                        ch.push_msg(&m.id, msg)
                    }
                }
                self.convlist.invalidate_sort()
            }
        } else {
            let conv_id = m.conv.to_string();
            info!(self.log, "message for unknown conversation"; "conv" => conv_id);
            let future = self.pool_act.spawn(self.conversation(&m.conv))
                .map(with!(this  => move |conv| {
                    if let Some(c) = conv {
                        this.on_conversation(c);
                        this.on_message(m)
                    } else {
                        error!(this.log, "Failed to resolve conversation"; "id" => conv_id)
                    }
                }))
                .map_err(with!(logger => move |e| {
                    error!(logger, "on_message error"; "error" => format!("{}", e))
                }));
            self.futures.send(Box::new(future)).unwrap()
        }
    }

    fn on_message_update(&self, id: String, c: ConvId, t: DateTime<UTC>, s: MessageStatus) {
        debug!(self.log, "on_message_update"; "conv" => c.to_string(), "id" => id);
        if let Some(mut ch) = self.channels.borrow_mut().get_mut(&c) {
            if let Some(mut msg) = ch.get_msg_mut(&id) {
                if s == MessageStatus::Sent {
                    msg.set_time(t.with_timezone(&self.timezone))
                }
            }
        } else {
            error!(self.log, "message update for unknown conversation"; "conv" => c.to_string())
        }
    }

    fn on_conversation(&self, mut conv: Conversation<'static>) {
        debug!(self.log, "on_conversation"; "conv" => conv.id.to_string());
        if self.channels.borrow().contains_key(&conv.id) {
            debug!(self.log, "conversation already loaded"; "conv" => conv.id.to_string());
            return ()
        }

        if conv.ctype != ConvType::OneToOne {
            let ch = Channel::group(&conv.time.with_timezone(&self.timezone), &conv.id, &conv.name);
            self.convlist.add(ch.channel_row());
            self.channels.borrow_mut().insert(conv.id, ch);
            self.convlist.show_all();
            return ()
        }

        // Set remote user name as conversation name if user is already in `self.res`.
        let me = self.me.borrow();
        if let Some(uid) = conv.members.iter().filter(|m| **m != me.id).next().cloned() {
            let mut res = self.res.borrow_mut();
            if let Some(mut u) = res.user_mut(&uid) {
                conv.set_name(Name::new(u.name.clone()));
                let ch = Channel::one_to_one(&conv.time.with_timezone(&self.timezone), &conv.id, &mut u);
                self.convlist.add(ch.channel_row());
                self.channels.borrow_mut().insert(conv.id, ch);
                self.convlist.show_all();
                return ()
            }
        }

        let user_id =
            if let Some(id) = conv.members.iter().filter(|m| **m != me.id).next().cloned() {
                id
            } else {
                warn!(self.log, "no user found in 1:1 conversation"; "conv" => conv.id.to_string());
                return ()
            };

        let this   = self.clone();
        let future = self.pool_act.spawn(self.user(user_id.clone()))
            .map(with!(this => move |u| {
                if let Some(user) = u {
                    if !this.channels.borrow().contains_key(&conv.id) {
                        this.ensure_user_res(&user);
                        let mut res = this.res.borrow_mut();
                        let mut usr = res.user_mut(&user.id).unwrap();
                        let     chn = Channel::one_to_one(&conv.time.with_timezone(&this.timezone), &conv.id, &mut usr);
                        this.convlist.add(chn.channel_row());
                        this.channels.borrow_mut().insert(conv.id, chn);
                        this.convlist.show_all()
                    }
                } else {
                    warn!(this.log, "user not found"; "id" => user_id.to_string());
                    return ()
                }
            }))
            .map_err(with!(this => move |e| {
                error!(this.log, "failed to post-process one to one conversation"; "error" => format!("{}", e))
            }));

        self.futures.send(Box::new(future)).unwrap()
    }

    fn on_contact(&self, app: &gtk::Application, to: User<'static>, contact: Connection) {
        debug!(self.log, "on_contact"; "to" => to.id.to_string());
        self.ensure_user_res(&to);
        let mut r = self.res.borrow_mut();
        let mut u = r.user_mut(&to.id).unwrap();
        let mut c = self.contacts.borrow_mut();
        if let Some(mut cont) = c.get_mut(&to.id) {
            cont.block_handler(true);
            cont.set_status(contact.status);
            cont.block_handler(false);
            return ()
        }
        let this = self.clone();
        let uid  = to.id.clone();
        let cid  = contact.conv.clone();
        c.add(&mut u, &contact, with!(app => move |w, s| {
            this.on_connect_change(&app, w, uid.clone(), cid.clone(), s)
        }));
    }

    fn on_members_add(&self, app: &gtk::Application, time: DateTime<UTC>, cid: ConvId, members: Vec<User<'static>>) {
        debug!(self.log, "on_members_add"; "conv" => cid.to_string());
        if let Some(mut ch) = self.channels.borrow_mut().get_mut(&cid) {
            let local_time = time.with_timezone(&self.timezone);
            for m in members {
                let id = m.id.as_uuid().simple().to_string();
                if ch.has_msg(&id) {
                    continue
                }
                self.ensure_user_res(&m);
                if local_time.date() != ch.newest_date() {
                    ch.add(Message::date(local_time.date()))
                }
                let txt = format!("{} has joined this conversation.", m.name.as_str());
                let msg = Message::text(Some(local_time), &mut (&m).into(), &txt);
                ch.push_msg(&id, msg);
                self.convlist.invalidate_sort()
            }
        } else {
            let this   = self.clone();
            let future = self.pool_act.spawn(self.conversation(&cid))
                .map(with!(this, app  => move |conv| {
                    if let Some(c) = conv {
                        this.on_conversation(c);
                        this.on_members_add(&app, time, cid, members)
                    } else {
                        error!(this.log, "Failed to resolve conversation"; "id" => cid.to_string())
                    }
                }))
                .map_err(with!(this => move |e| {
                    error!(this.log, "on_message error"; "error" => format!("{}", e))
                }));
            self.futures.send(Box::new(future)).unwrap()
        }
    }

    fn on_new_conv(&self, app: &gtk::Application, name: String, u: UserId) {
        trace!(self.log, "on_new_conv");

        enum Data<'a> {
            Sent,
            NoUser,
            Invalid(ConnectStatus),
            Conv(Conversation<'a>)
        }

        let this   = self.clone();
        let actor  = self.actor.clone();
        let future =
            self.pool_act.spawn_fn(move || {
                let mut act = actor.lock().unwrap();
                if let Some(Io::Online(ref mut a)) = *act {
                    let n = Name::new(name);
                    match a.resolve_connection(&u)? {
                        Some(conn) =>
                            if conn.status != ConnectStatus::Accepted {
                                return Ok(Data::Invalid(conn.status))
                            },
                        None => {
                            if let Some(usr) = a.resolve_user(&u)? {
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
                    Err(Error::Message("invalid app state"))
                }
            })
            .map(with!(this, app => move |data| {
                match data {
                    Data::Conv(c) => this.on_conversation(c),
                    Data::Sent    => show_message(&app, MessageType::Info, "Connection request sent", "", None),
                    Data::NoUser  => show_message(&app, MessageType::Error, "User not found", "", None),
                    Data::Invalid(s) => {
                        let s = format!("The current connection status ({}) does not allow creating a new conversation with this user.", s.as_str());
                        show_message(&app, MessageType::Info, "Not allowed", &s, None)
                    }
                }
            }))
            .map_err(with!(app => move |e| {
                error!(this.log, "failed to create conversation"; "error" => format!("{}", e));
                show_error(&app, e, "Failed to create conversation", "")
            }));
        self.futures.send(Box::new(future)).unwrap()
    }

    fn on_connect_change(&self, app: &gtk::Application, s: &gtk::ComboBoxText, uid: UserId, cid: ConvId, new: ConnectStatus) {
        debug!(self.log, "on_connect_change"; "user" => uid.to_string());
        s.set_sensitive(false);
        let this   = self.clone();
        let actor  = self.actor.clone();
        let future =
            self.pool_act.spawn_fn(move || {
                let mut act = actor.lock().unwrap();
                if let Some(Io::Online(ref mut a)) = *act {
                    a.update_connection(&uid, new)
                } else {
                    Err(Error::Message("invalid app state"))
                }
            })
            .and_then(with!(this => move |updated| {
                if updated && new == ConnectStatus::Accepted {
                    let f = this.pool_act.spawn(this.conversation(&cid)).map(move |conv| conv.map(|c| this.on_conversation(c)));
                    Box::new(f) as Box<Future<Item=Option<()>, Error=Error>>
                } else {
                    Box::new(futures::finished(None))
                }
            }))
            .map(with!(s => move |_| {
                s.set_sensitive(true)
            }))
            .map_err(with!(app => move |e| {
                error!(this.log, "failed to update status"; "error" => format!("{}", e));
                show_error(&app, e, "Failed to update status", "")
            }));
        self.futures.send(Box::new(future)).unwrap()
    }

    //
    // Actions
    //

    fn set_user_icon(&self, u: UserId) {
        debug!(self.log, "set user icon"; "id" => u.to_string());
        let this   = self.clone();
        let future = self.pool_act.spawn(self.user_icon(u.clone()))
            .map(with!(this, u => move |data| {
                if data.is_empty() {
                    info!(this.log, "no user icon"; "user" => u.to_string());
                    return ()
                }
                let mut res = this.res.borrow_mut();
                if let Some(ref mut user) = res.user_mut(&u) {
                    user.set_icon(&data)
                } else {
                    warn!(this.log, "no user resources"; "user" => u.to_string())
                }
            }))
            .map_err(move |e| {
                error!(this.log, "error loading user icon"; "user" => u.to_string(), "error" => format!("{}", e))
            });
        self.futures.send(Box::new(future)).unwrap()
    }

    fn send_message(&self, id: &ConvId, msg: GenericMessage) {
        debug!(self.log, "send message"; "conv" => id.to_string(), "id" => msg.get_message_id());
        let this = self.clone();
        let mid  = String::from(msg.get_message_id());
        let text = String::from(msg.get_text().get_content());
        let future =
            futures::lazy(with!(this, id, mid => move || {
                if let Some(mut ch) = this.channels.borrow_mut().get_mut(&id) {
                    this.ensure_user_res(&*this.me.borrow());
                    let mut res = this.res.borrow_mut();
                    let mut usr = res.user_mut(&this.me.borrow().id).unwrap();
                    if !ch.has_msg(&mid) {
                        let msg = Message::text(None, &mut usr, &text);
                        msg.start_spinner();
                        ch.push_msg(&mid, msg)
                    }
                }
                futures::finished(())
            }))
            .and_then(with!(this, id => move |()| {
                this.pool_loc.spawn(this.prepare_message(&id, msg))
            }))
            .and_then(with!(this => move |(msg, params)| {
                this.pool_rem.spawn(this.send(params, msg))
            }))
            .map(with!(this, id, mid => move |dt| {
                let mut channels = this.channels.borrow_mut();
                if let Some(mut ch) = channels.get_mut(&id) {
                    let loc_time  = dt.with_timezone(&this.timezone);
                    let msg_index =
                        if let Some(mut msg) = ch.get_msg_mut(&mid) {
                            msg.stop_spinner();
                            msg.set_time(loc_time.clone());
                            msg.index()
                        } else {
                            -1
                        };
                    if msg_index != -1 {
                        if loc_time.date() != ch.newest_date() {
                            ch.insert(msg_index, Message::date(loc_time.date()));
                            ch.set_newest_date(loc_time.date())
                        }
                        ch.update_time(&loc_time);
                        this.convlist.invalidate_sort()
                    }
                }
            }))
            .map_err(with!(this, id, mid => move |e| {
                error!(this.log, "error sending message"; "error" => format!("{}", e));
                let channels = this.channels.borrow();
                if let Some(ch) = channels.get(&id) {
                    if let Some(msg) = ch.get_msg(&mid) {
                        msg.set_error()
                    }
                }
            }));
        self.futures.send(Box::new(future)).unwrap()
    }

    fn load_local_conversations(&self, app: &gtk::Application) {
        debug!(self.log, "load conversations");
        let this   = self.clone();
        let future = self.pool_act.spawn(self.local_conversations(None)) // TODO
            .map(with!(this => move |page| {
                for c in page.data {
                    this.on_conversation(c)
                }
            }))
            .map_err(with!(app => move |e| {
                error!(this.log, "failed to load conversation"; "error" => format!("{}", e));
                show_error(&app, e, "Failed to load conversations", "")
            }));
        self.futures.send(Box::new(future)).unwrap()
    }

    fn load_remote_conversations(&self, app: &gtk::Application) {
        debug!(self.log, "load remote conversations");
        let this   = self.clone();
        let actor  = self.actor.clone();
        let future =
            self.pool_act.spawn_fn(move || {
                let mut act = actor.lock().unwrap();
                if let Some(Io::Online(ref mut a)) = *act {
                    if a.is_new_client() {
                        a.resolve_conversations()
                    } else {
                        Ok(())
                    }
                } else {
                    Err(Error::Message("invalid app state"))
                }
            })
            .map_err(with!(app => move |e| {
                error!(this.log, "failed to load remote conversations"; "error" => format!("{}", e));
                show_error(&app, e, "Failed to load remote conversations", "")
            }));
        self.futures.send(Box::new(future)).unwrap()
    }

    fn load_messages(&self, app: &gtk::Application, cid: &ConvId) {
        debug!(self.log, "load conversation messages"; "id" => cid.to_string());
        let this   = self.clone();
        let future = self.pool_act.spawn(self.messages(cid, None)) // TODO
            .map(with!(this, cid => move |mm| {
                if let Some(mut chan) = this.channels.borrow_mut().get_mut(&cid) {
                    let is_empty = mm.data.is_empty();
                    for m in mm.data {
                        if chan.has_msg(&m.id) {
                            continue
                        }
                        this.ensure_user_res(&m.user);
                        let mut res = this.res.borrow_mut();
                        let mut usr = res.user_mut(&m.user.id).unwrap();
                        let mut msg = match m.data {
                            MessageData::Text(txt) =>
                                Message::text(None, &mut usr, &txt),
                            MessageData::MemberJoined => {
                                let txt = format!("{} has joined this conversation.", usr.name);
                                Message::text(None, &mut usr, &txt)
                            }
                        };
                        let mtime   = m.time.with_timezone(&this.timezone);
                        let daydiff = mtime.date() != chan.oldest_date();
                        if m.status == MessageStatus::Created {
                            msg.set_error()
                        } else {
                            msg.set_time(mtime)
                        }
                        if daydiff {
                            let date = chan.oldest_date();
                            chan.add_front(Message::date(date))
                        }
                        chan.push_front_msg(&m.id, msg)
                    }
                    if !is_empty {
                        let date = chan.oldest_date();
                        chan.add_front(Message::date(date));
                    }
                    chan.set_init()
                };
            }))
            .map_err(with!(app => move |e| {
                error!(this.log, "failed to load messages"; "error" => format!("{}", e));
                show_error(&app, e, "Failed to load messages", "")
            }));
        self.futures.send(Box::new(future)).unwrap()
    }

    fn load_contacts(&self, app: &gtk::Application) {
        trace!(self.log, "load contacts");
        let this   = self.clone();
        let future = self.pool_act.spawn(self.contacts())
            .map(with!(this, app => move |cc| {
                for (u, c) in cc {
                    this.on_contact(&app, u, c)
                }
                let mut c = this.contacts.borrow_mut();
                c.set_init()
            }))
            .map_err(with!(app => move |e| {
                error!(this.log, "failed to load contacts"; "error" => format!("{}", e));
                show_error(&app, e, "Failed to load contacts", "")
            }));
        self.futures.send(Box::new(future)).unwrap()
    }

    fn show_new_conv(&self, app: &gtk::Application) {
        trace!(self.log, "show_new_conv");
        let this = self.clone();
        let builder = Builder::new_from_string(include_str!("gtk/new-conversation.ui"));
        let window: Window = builder.get_object("new-conv-window").unwrap();
        let submit: Button = builder.get_object("submit-button").unwrap();
        let cancel: Button = builder.get_object("cancel-button").unwrap();
        cancel.connect_clicked(with!(window => move |_| window.hide()));

        let name: gtk::Entry = builder.get_object("name-entry").unwrap();
        let user: gtk::Entry = builder.get_object("user-entry").unwrap();

        submit.connect_clicked(with!(this, app, window, name, user => move |_| {
            window.hide();
            if let Some(u) = user.get_text().and_then(|s| UserId::from_str(&s)) {
                this.on_new_conv(&app, name.get_text().unwrap_or("N/A".into()), u)
            } else {
                show_message(&app, MessageType::Error, "Invalid UserId", "", None)
            }
        }));

        window.set_transient_for(app.get_active_window().as_ref());
        window.show_all();
    }

    // Ensure actor is in `Online` state.
    fn ensure_online(actor: &mut Option<Io>) -> Result<(), Error> {
        match actor.take() {
            Some(Io::Offline(a)) =>
                match a.connect() {
                    Ok(c) => {
                        let mut a2 = a.online(c);
                        if let Err(e) = a2.renew_access() {
                            *actor = Some(Io::Offline(a2.offline()));
                            Err(e)
                        } else {
                            *actor = Some(Io::Online(a2));
                            Ok(())
                        }
                    }
                    Err(e) => {
                        *actor = Some(Io::Offline(a));
                        Err(e)
                    }
                },
            Some(Io::Online(a)) => {
                *actor = Some(Io::Online(a));
                Ok(())
            }
            None  => Err(Error::Message("no actor")),
            other => {
                *actor = other;
                Err(Error::Message("impossible actor transition"))
            }
        }
    }

    // Ensure actor is in `Connected` state.
    fn ensure_connected(actor: &mut Option<Io>) -> Result<(), Error> {
        match actor.take() {
            Some(Io::Init(a)) =>
                match a.connect() {
                    Ok(c) => {
                        *actor = Some(Io::Connected(a.connected(c)));
                        Ok(())
                    }
                    Err(e) => {
                        *actor = Some(Io::Init(a));
                        Err(e)
                    }
                },
            Some(Io::Connected(a)) => {
                *actor = Some(Io::Connected(a));
                Ok(())
            }
            Some(Io::Offline(a)) => {
                *actor = Some(Io::Init(a.init()));
                Coax::ensure_connected(actor)
            }
            Some(Io::Online(a)) => {
                *actor = Some(Io::Init(a.init()));
                Coax::ensure_connected(actor)
            }
            None => Err(Error::Message("no actor"))
        }
    }

    fn ensure_user_res(&self, u: &User) {
        let mut res = self.res.borrow_mut();
        if !res.has_user(&u.id) {
            res.add_user(u);
            self.set_user_icon(u.id.clone())
        }
    }

    fn show_info(&self, txt: &str) {
        self.info.set_markup(txt);
        self.revealer.set_reveal_child(true)
    }

    fn hide_info(&self) {
        self.revealer.set_reveal_child(false)
    }

    //
    // Futures
    //

    fn user_icon(&self, u: UserId) -> impl Future<Item=Vec<u8>, Error=Error> {
        trace!(self.log, "user icon future");
        let actor = self.actor.clone();
        futures::lazy(move || {
            let mut act = actor.lock().unwrap();
            match *act {
                Some(Io::Online(ref mut a)) =>
                    if let Some(usr) = a.resolve_user(&u)? {
                        a.load_user_icon(&usr)
                    } else {
                        Ok(Vec::new())
                    },
                Some(Io::Offline(ref mut a)) =>
                    if let Some(usr) = a.load_user(&u)? {
                        a.load_user_icon(&usr)
                    } else {
                        Ok(Vec::new())
                    },
                _ => Err(Error::Message("invalid app state"))
            }
        })
    }

    fn prepare_message(&self, id: &ConvId, msg: GenericMessage) -> impl Future<Item=(GenericMessage, send::Params), Error=Error> {
        debug!(self.log, "prepare message future"; "conv" => id.to_string(), "id" => msg.get_message_id());
        let actor = self.actor.clone();
        futures::lazy(with!(id => move || {
            let mut act = actor.lock().unwrap();
            if let Some(Io::Online(ref mut a)) = *act {
                a.store_message(&id, &msg)?;
                let p = a.prepare_message(&id, &msg)?;
                a.enqueue(msg.get_message_id().as_bytes(), &p, &msg)?;
                Ok((msg, p))
            } else {
                Err(Error::Message("invalid app state"))
            }
        }))
    }

    fn send(&self, mut params: send::Params, msg: GenericMessage) -> impl Future<Item=DateTime<UTC>, Error=Error> {
        debug!(self.log, "send future"; "conv" => params.conv.to_string(), "id" => msg.get_message_id());
        let actor = self.actor.clone();
        futures::lazy(move || {
            loop {
                {
                    let mut act = actor.lock().unwrap();
                    if let Some(Io::Online(ref mut a)) = *act {
                        if let Ok(dt) = a.send_message(&mut params, &msg) {
                            a.dequeue(msg.get_message_id().as_bytes(), &params.conv)?;
                            return Ok(dt)
                        }
                    } else {
                        return Err(Error::Message("invalid app state"))
                    }
                }
                thread::sleep(Duration::from_secs(3))
            }
        })
    }

    fn resend_messages(&self) -> impl Future<Item=(), Error=Error> {
        trace!(self.log, "re-send messages future");
        let actor = self.actor.clone();
        futures::lazy(move || {
            let mut act = actor.lock().unwrap();
            if let Some(Io::Online(ref mut a)) = *act {
                a.resend()
            } else {
                Err(Error::Message("invalid app state"))
            }
        })
    }

    fn conversation(&self, id: &ConvId) -> impl Future<Item=Option<Conversation<'static>>, Error=Error> {
        trace!(self.log, "load conversation future");
        let actor = self.actor.clone();
        futures::lazy(with!(id => move || {
            let mut act = actor.lock().unwrap();
            if let Some(Io::Online(ref mut a)) = *act {
                a.resolve_conversation(&id)
            } else {
                Err(Error::Message("invalid app state"))
            }
        }))
    }

    fn user(&self, id: UserId) -> impl Future<Item=Option<User<'static>>, Error=Error> {
        trace!(self.log, "user future");
        let actor = self.actor.clone();
        futures::lazy(move || {
            let mut act = actor.lock().unwrap();
            match *act {
                Some(Io::Offline(ref mut a)) => a.load_user(&id),
                Some(Io::Online(ref mut a))  => a.resolve_user(&id),
                _                            => Err(Error::Message("invalid app state"))
            }
        })
    }

    fn local_conversations(&self, from: Option<PagingState<db::C>>) -> impl Future<Item=Page<Vec<coax_data::Conversation<'static>>, db::C>, Error=Error> {
        trace!(self.log, "conversations future");
        let actor = self.actor.clone();
        futures::lazy(move || {
            let mut act = actor.lock().unwrap();
            match *act {
                Some(Io::Online(ref mut a))  => a.load_conversations(from, 64),
                Some(Io::Offline(ref mut a)) => a.load_conversations(from, 64),
                _                            => Err(Error::Message("invalid app state"))
            }
        })
    }

    fn messages(&self, cid: &ConvId, from: Option<PagingState<db::M>>) -> impl Future<Item=Page<Vec<coax_data::Message<'static>>, db::M>, Error=Error> {
        trace!(self.log, "messages future");
        let actor = self.actor.clone();
        let conv_id = cid.clone();
        futures::lazy(move || {
            let mut act = actor.lock().unwrap();
            match *act {
                Some(Io::Online(ref mut a))  => a.load_messages(&conv_id, from, 64),
                Some(Io::Offline(ref mut a)) => a.load_messages(&conv_id, from, 64),
                _                            => Err(Error::Message("invalid app state"))
            }
        })
    }

    fn contacts(&self) -> impl Future<Item=Vec<(User<'static>, Connection)>, Error=Error> {
        trace!(self.log, "contacts future");
        let actor = self.actor.clone();
        futures::lazy(move || {
            let mut act = actor.lock().unwrap();
            match *act {
                Some(Io::Online(ref mut a))  => a.load_contacts(),
                Some(Io::Offline(ref mut a)) => a.load_contacts(),
                _                            => Err(Error::Message("invalid app state"))
            }
        })
    }

    fn notifications(&self, initial: bool) -> impl Future<Item=(), Error=Error> {
        trace!(self.log, "notifications future");
        let sync   = self.sync.clone();
        let logger = self.log.clone();
        futures::lazy(move || {
            let mut actor = sync.lock().unwrap();
            if let Some(ref mut a) = *actor {
                loop {
                    debug!(logger, "actor getting notifications");
                    if !a.notifications(!initial)? {
                        break
                    }
                }
                Ok(())
            } else {
                Err(Error::Message("invalid app state"))
            }
        })
    }
}

fn set_subtitle(app: &gtk::Application, s: Option<&str>) {
    app.get_active_window()
       .and_then(|w| w.get_titlebar())
       .and_then(|t| t.downcast::<gtk::HeaderBar>().ok())
       .map(|h| h.set_subtitle(s));
}

fn show_error(app: &gtk::Application, e: Error, msg: &str, sec: &str) {
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

