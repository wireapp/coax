use coax_api::types::UserId;
use coax_api::user::ConnectStatus;
use coax_data::Connection;
use ffi;
use fnv::FnvHashMap;
use gtk::{self, Align};
use gtk::prelude::*;
use res;

#[derive(Clone)]
pub struct Contacts {
    list:    gtk::ListBox,
    refresh: gtk::Button,
    view:    gtk::ScrolledWindow,
    model:   FnvHashMap<UserId, Contact>,
    init:    bool
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
            model:   FnvHashMap::default(),
            init:    false
        }
    }

    pub fn add<F>(&mut self, u: &mut res::User, c: &Connection, k: F)
        where F: Fn(&gtk::ComboBoxText, ConnectStatus) + 'static
    {
        let contact = Contact::new(u, c, k);
        self.list.add(&contact.row);
        self.model.insert(u.id.clone(), contact);
    }

    pub fn contact_view(&self) -> &gtk::ScrolledWindow {
        &self.view
    }

    pub fn get_mut(&mut self, id: &UserId) -> Option<&mut Contact> {
        self.model.get_mut(id)
    }

    pub fn is_init(&self) -> bool {
        self.init
    }

    pub fn set_init(&mut self) {
        self.init = true
    }

    pub fn set_refresh_action<F>(&self, f: F)
        where F: Fn() + 'static
    {
        self.refresh.connect_clicked(move |_| f());
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

        let name = gtk::Label::new(None);
        name.set_markup(&format!("<span size=\"x-large\"><b>{}</b></span>", usr.name));
        name.set_halign(Align::Center);
        name.set_hexpand(true);
        name.set_margin_top(12);
        grid.attach(&name, 0, 0, 1, 1);

        let status = gtk::ComboBoxText::new();
        status.set_halign(Align::Center);
        grid.attach(&status, 0, 1, 1, 1);

        let img = usr.icon_large();
        img.set_margin_left(12);
        img.set_margin_top(6);
        img.set_margin_right(6);
        img.set_margin_bottom(12);
        img.set_halign(Align::Start);
        grid.attach(&img, 0, 2, 1, 1);

        let sep = gtk::Separator::new(gtk::Orientation::Horizontal);
        grid.attach(&sep, 0, 3, 1, 1);

        grid.insert_column(1);

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
            Blocked => {}
        }
    }

    pub fn block_handler(&self, block: bool) {
        ffi::block_handler(&self.status, self.handler, block);
    }
}

