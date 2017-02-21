use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};

use chrono::{Date, DateTime, Local};
use coax_api::types::{Name, ConvId};
use ffi;
use fnv::FnvHashMap;
use gdk_pixbuf::{InterpType, Pixbuf};
use gtk::{self, Align};
use gtk::prelude::*;
use res;
use util::hash;

#[derive(Clone)]
pub struct Channel {
    image:        gtk::Image,
    name_label:   gtk::Label,
    date_label:   gtk::Label,
    time_label:   gtk::Label,
    channel_row:  gtk::ListBoxRow,
    message_list: gtk::ListBox,
    message_view: gtk::ScrolledWindow,
    model:        FnvHashMap<u64, Message>,
    init:         bool,
    autoscroll:   Rc<AtomicBool>,
    date_lower:   Date<Local>,
    date_upper:   Date<Local>
}

impl Channel {
    pub fn one_to_one(dt: &DateTime<Local>, id: &ConvId, u: &mut res::User) -> Channel {
        Channel::new(dt, id, &Some(Name::new(u.name.clone())), u.icon())
    }

    pub fn group(dt: &DateTime<Local>, id: &ConvId, n: &Option<Name>) -> Channel {
        let img = {
            let buf = Pixbuf::new_from_resource("/coax/icons/bubbles.png").unwrap();
            let ico = buf.scale_simple(32, 32, InterpType::Bilinear).unwrap();
            gtk::Image::new_from_pixbuf(Some(&ico))
        };
        Channel::new(dt, id, n, img)
    }

    fn new(dt: &DateTime<Local>, id: &ConvId, n: &Option<Name>, img: gtk::Image) -> Channel {
        let channel_row = gtk::ListBoxRow::new();
        let grid = gtk::Grid::new();
        grid.set_margin_left(6);
        grid.set_margin_top(6);
        grid.set_margin_right(6);
        grid.set_margin_bottom(6);

        img.set_margin_left(6);
        img.set_margin_top(6);
        img.set_margin_right(6);
        img.set_margin_bottom(6);
        grid.attach(&img, 0, 0, 1, 2);

        let name_label = gtk::Label::new(None);
        ffi::set_ellipsis(&name_label);
        name_label.set_max_width_chars(64);
        name_label.set_margin_left(6);
        name_label.set_margin_top(6);
        name_label.set_margin_right(6);
        name_label.set_margin_bottom(6);
        name_label.set_hexpand(true);
        name_label.set_halign(Align::Fill);
        name_label.set_xalign(0.0);
        grid.attach(&name_label, 1, 0, 1, 2);

        let time_label = gtk::Label::new(None);
        time_label.set_margin_left(6);
        time_label.set_margin_top(6);
        time_label.set_margin_right(6);
        grid.attach(&time_label, 2, 0, 1, 1);

        let date_label = gtk::Label::new(None);
        date_label.set_margin_left(6);
        date_label.set_margin_right(6);
        date_label.set_margin_bottom(6);
        grid.attach(&date_label, 2, 1, 1, 1);

        channel_row.add(&grid);
        ffi::set_data(&channel_row, &ffi::KEY_ID, id.clone());
        ffi::set_data(&channel_row, &ffi::TSTAMP, dt.timestamp());

        let message_list = gtk::ListBox::new();
        message_list.set_vexpand(true);
        message_list.set_hexpand(true);
        message_list.set_selection_mode(gtk::SelectionMode::None);

        let message_view = gtk::ScrolledWindow::new(None, None);
        message_view.add(&message_list);

        let autoscroll = Rc::new(AtomicBool::new(true));
        if let Some(vadj) = message_view.get_vadjustment() {
            vadj.connect_value_changed(with!(autoscroll => move |va| {
                let at_bottom = va.get_value() == va.get_upper() - va.get_page_size();
                autoscroll.store(at_bottom, Ordering::Relaxed)
            }));
        }
        message_list.connect_size_allocate(with!(message_view, autoscroll => move |_, _| {
            if !autoscroll.load(Ordering::Relaxed) {
                return ()
            }
            if let Some(vadj) = message_view.get_vadjustment() {
                vadj.set_value(vadj.get_upper() - vadj.get_page_size());
                message_view.set_vadjustment(&vadj)
            }
        }));

        let ch = Channel {
            image:        img,
            name_label:   name_label,
            date_label:   date_label,
            time_label:   time_label,
            channel_row:  channel_row,
            message_list: message_list,
            message_view: message_view,
            model:        FnvHashMap::default(),
            init:         false,
            autoscroll:   autoscroll,
            date_lower:   dt.date(),
            date_upper:   dt.date()
        };

        ch.set_name(n.as_ref().unwrap_or(&Name::new("N/A")).as_str());
        ch.set_time(dt);
        ch
    }

    pub fn channel_row(&self) -> &gtk::ListBoxRow {
        &self.channel_row
    }

    pub fn message_view(&self) -> &gtk::ScrolledWindow {
        &self.message_view
    }

    pub fn has_msg(&self, k: &str) -> bool {
        self.model.contains_key(&hash(k))
    }

    pub fn get_msg(&self, k: &str) -> Option<&Message> {
        self.model.get(&hash(k))
    }

    pub fn get_msg_mut(&mut self, k: &str) -> Option<&mut Message> {
        self.model.get_mut(&hash(k))
    }

    pub fn insert(&mut self, i: i32, m: Message) {
        self.message_list.insert(&m.row, i)
    }

    pub fn add_front(&mut self, m: Message) {
        self.message_list.prepend(&m.row);
        if let Some(ref time) = m.dtime {
            self.date_lower = time.date()
        }
    }

    pub fn add(&mut self, m: Message) {
        self.message_list.add(&m.row);
        if let Some(ref time) = m.dtime {
            self.date_upper = time.date()
        }
    }

    pub fn push_front_msg(&mut self, id: &str, m: Message) {
        self.message_list.prepend(&m.row);
        if let Some(ref time) = m.dtime {
            self.date_lower = time.date()
        }
        self.model.insert(hash(id), m);
    }

    pub fn push_msg(&mut self, id: &str, m: Message) {
        self.message_list.add(&m.row);
        if let Some(ref time) = m.dtime {
            self.date_upper = time.date();
            self.update_time(time);
        }
        self.model.insert(hash(id), m);
    }

    pub fn is_init(&self) -> bool {
        self.init
    }

    pub fn set_init(&mut self) {
        self.init = true
    }

    pub fn oldest_date(&self) -> Date<Local> {
        self.date_lower
    }

    pub fn newest_date(&self) -> Date<Local> {
        self.date_upper
    }

    pub fn set_newest_date(&mut self, d: Date<Local>) {
        self.date_upper = d
    }

    pub fn update_time(&self, dt: &DateTime<Local>) {
        self.set_time(dt);
        self.update_tstamp(dt.timestamp())
    }

    fn update_tstamp(&self, dt: i64) {
        ffi::set_data(&self.channel_row, &ffi::TSTAMP, dt)
    }

    fn set_time(&self, dt: &DateTime<Local>) {
        let tstr = dt.format("%T").to_string();
        let dstr = dt.format("%F").to_string();
        self.time_label.set_markup(&format!("<small>{}</small>", tstr));
        self.date_label.set_markup(&format!("<small>{}</small>", dstr))
    }

    fn set_name(&self, name: &str) {
        let nstr = ffi::escape(name).to_string_lossy();
        self.name_label.set_markup(&format!("<big><b>{}</b></big>", nstr))
    }
}

#[derive(Debug, Clone)]
pub struct Message {
    dtime: Option<DateTime<Local>>,
    row:   gtk::ListBoxRow,
    grid:  gtk::Grid,
    icon:  gtk::Image,
    time:  gtk::Label
}

impl Message {
    pub fn text(dt: Option<DateTime<Local>>, u: &mut res::User, txt: &str) -> Message {
        let row = gtk::ListBoxRow::new();
        let grid = gtk::Grid::new();
        grid.set_margin_left(6);
        grid.set_margin_top(6);
        grid.set_margin_right(6);
        grid.set_margin_bottom(6);
        grid.set_column_spacing(12);

        let img = u.icon();
        img.set_margin_bottom(6);
        grid.attach(&img, 0, 0, 1, 1);

        let nme = gtk::Label::new(None);
        nme.set_markup(&format!("<small><b>{}</b></small>", u.name));
        nme.set_margin_bottom(6);
        nme.set_halign(Align::Start);
        grid.attach(&nme, 1, 0, 1, 1);

        let lbl = gtk::Label::new(Some(txt));
        lbl.set_selectable(true);
        lbl.set_margin_bottom(6);
        lbl.set_hexpand(true);
        lbl.set_halign(Align::Fill);
        lbl.set_xalign(0.0);
        lbl.set_line_wrap(true);
        grid.attach(&lbl, 1, 1, 1, 1);

        let time = gtk::Label::new(None);
        if let Some(t) = dt {
            let tstr = t.format("%T").to_string();
            time.set_markup(&format!("<small>{}</small>", tstr))
        }
        grid.attach(&time, 2, 0, 1, 1);

        row.add(&grid);
        row.show_all();

        Message {
            dtime: dt,
            row:   row,
            grid:  grid,
            icon:  img,
            time:  time
        }
    }

    pub fn date(d: Date<Local>) -> Message {
        let row  = gtk::ListBoxRow::new();
        let grid = gtk::Grid::new();
        grid.set_margin_left(6);
        grid.set_margin_top(6);
        grid.set_margin_right(6);
        grid.set_margin_bottom(6);

        let time = gtk::Label::new(None);
        time.set_hexpand(true);
        time.set_halign(Align::Fill);
        let dstr = d.format("%F").to_string();
        time.set_markup(&format!("<big><b>{}</b></big>", dstr));
        grid.attach(&time, 0, 0, 1, 1);

        row.add(&grid);
        row.show_all();

        Message {
            dtime: Some(d.and_hms(0, 0, 0)),
            row:   row,
            grid:  grid,
            icon:  gtk::Image::new(),
            time:  time
        }
    }

    pub fn index(&self) -> i32 {
        self.row.get_index()
    }

    pub fn set_time(&mut self, dt: DateTime<Local>) {
        if let Some(w) = self.grid.get_child_at(2, 0) {
            self.grid.remove(&w)
        }
        let tstr = dt.format("%T").to_string();
        self.time.set_markup(&format!("<small>{}</small>", tstr));
        self.dtime = Some(dt);
        self.grid.attach(&self.time, 2, 0, 1, 1)
    }

    pub fn set_error(&self) {
        let img = gtk::Image::new_from_icon_name("emblem-important", gtk::IconSize::SmallToolbar.into());
        img.set_margin_right(6);
        if let Some(w) = self.grid.get_child_at(2, 0) {
            self.grid.remove(&w)
        }
        img.show();
        self.grid.attach(&img, 2, 0, 1, 1)
    }

    pub fn start_spinner(&self) {
        let spinner = gtk::Spinner::new();
        spinner.set_margin_right(6);
        spinner.start();
        if let Some(w) = self.grid.get_child_at(2, 0) {
            self.grid.remove(&w)
        }
        spinner.show();
        self.grid.attach(&spinner, 2, 0, 1, 1)
    }

    pub fn stop_spinner(&self) {
        if let Some(w) = self.grid.get_child_at(2, 0) {
            self.grid.remove(&w)
        }
    }
}

