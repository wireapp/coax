use std::sync::atomic::{AtomicBool, Ordering};
use chashmap::{CHashMap, WriteGuard};
use coax_api::types::UserId;
use coax_api::user::ConnectStatus;
use coax_data::Connection;
use ffi;
use gtk::{self, Align};
use gtk::prelude::*;
use res;

pub struct Contacts {
    list:    gtk::ListBox,
    refresh: gtk::Button,
    view:    gtk::ScrolledWindow,
    model:   CHashMap<UserId, Contact>,
    init:    AtomicBool
}

impl Contacts {
    pub fn new() -> Contacts {
        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);

        let lst = gtk::ListBox::new();
        lst.set_vexpand(true);
        lst.set_hexpand(true);
        vbox.add(&lst);

        let refresh = gtk::Button::new_with_label("Reload all contacts");
        refresh.set_sensitive(false);
        vbox.add(&refresh);

        let win = gtk::ScrolledWindow::new(None, None);
        win.add(&vbox);

        Contacts {
            list:    lst,
            refresh: refresh,
            view:    win,
            model:   CHashMap::new(),
            init:    AtomicBool::new(false)
        }
    }

    pub fn add<F>(&self, u: &mut res::User, c: &Connection, k: F)
        where F: Fn(&gtk::ComboBoxText, ConnectStatus) + 'static
    {
        let contact = Contact::new(u, c, k);
        self.list.add(&contact.row);

        let sep = gtk::Separator::new(gtk::Orientation::Horizontal);
        sep.set_margin_left(12);
        sep.set_margin_right(12);
        let row = gtk::ListBoxRow::new();
        row.add(&sep);
        row.show_all();
        self.list.add(&row);

        self.model.insert(u.id.clone(), contact);
    }

    pub fn contact_view(&self) -> &gtk::ScrolledWindow {
        &self.view
    }

    pub fn get_mut(&self, id: &UserId) -> Option<WriteGuard<UserId, Contact>> {
        self.model.get_mut(id)
    }

    pub fn is_init(&self) -> bool {
        self.init.load(Ordering::Relaxed)
    }

    pub fn set_init(&self) {
        self.init.store(true, Ordering::Relaxed)
    }

    pub fn set_refresh_action<F>(&self, f: F)
        where F: Fn(gtk::Button) + 'static
    {
        self.refresh.connect_clicked(move |b| {
            b.set_sensitive(false);
            f(b.clone())
        });
        self.refresh.set_sensitive(true)
    }
}

#[derive(Debug, Clone)]
pub struct Contact {
    row:     gtk::ListBoxRow,
    status:  gtk::ComboBoxText,
    icon:    gtk::Image,
    handler: u64
}

impl Contact {
    fn new<F>(usr: &mut res::User, contact: &Connection, k: F) -> Contact
        where F: Fn(&gtk::ComboBoxText, ConnectStatus) + 'static
    {
        let row = gtk::ListBoxRow::new();
        let grid = gtk::Grid::new();
        grid.set_margin_left(6);
        grid.set_margin_right(6);
        grid.set_row_spacing(12);

        let img = usr.icon_large();
        img.set_margin_left(12);
        img.set_margin_top(12);
        img.set_margin_right(12);
        img.set_margin_bottom(12);
        grid.attach(&img, 0, 0, 1, 1);

        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
        vbox.set_margin_top(12);

        let name = gtk::Label::new(None);
        name.set_markup(&format!("<span size=\"x-large\"><b>{}</b></span>", usr.name));
        name.set_halign(Align::Center);
        name.set_hexpand(true);
        vbox.add(&name);

        let handle = gtk::Label::new(usr.handle.as_ref().map(|s| s.as_ref()));
        handle.set_halign(Align::Center);
        handle.set_hexpand(true);
        handle.set_margin_bottom(12);
        vbox.add(&handle);

        let status = gtk::ComboBoxText::new();
        status.set_halign(Align::Center);
        vbox.add(&status);

        grid.attach(&vbox, 1, 0, 1, 1);

        row.add(&grid);
        row.show_all();

        let mut cr = Contact {
            row:     row,
            status:  status,
            icon:    img,
            handler: 0
        };

        cr.set_status(contact.status);
        cr.handler = cr.status.connect_changed(move |s| {
            if let Some(cs) = s.get_active_id().and_then(|x| ConnectStatus::from_str(&x)) {
                k(s, cs)
            }
        });

        cr
    }

    pub fn set_status(&mut self, s: ConnectStatus) {
        use self::ConnectStatus::*;
        self.status.remove_all();
        self.status.append(Some(s.as_str()), s.as_str());
        self.status.set_active(0);
        match s {
            Accepted => {
                self.status.append(Some(Blocked.as_str()), Blocked.as_str())
            }
            Pending => {
                self.status.append(Some(Accepted.as_str()), Accepted.as_str());
                self.status.append(Some(Blocked.as_str()), Blocked.as_str());
                self.status.append(Some(Ignored.as_str()), Ignored.as_str())
            }
            Sent => {
                self.status.append(Some(Cancelled.as_str()), Cancelled.as_str());
                self.status.append(Some(Blocked.as_str()), Blocked.as_str())
            }
            Cancelled => {
                self.status.append(Some(Blocked.as_str()), Blocked.as_str())
            }
            Ignored => {
                self.status.append(Some(Accepted.as_str()), Accepted.as_str());
                self.status.append(Some(Blocked.as_str()), Blocked.as_str())
            }
            Blocked => {
                self.status.append(Some(Accepted.as_str()), Accepted.as_str())
            }
        }
    }

    pub fn block_handler(&self, block: bool) {
        ffi::block_handler(&self.status, self.handler, block);
    }
}

