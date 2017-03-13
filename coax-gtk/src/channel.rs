use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};

use chrono::{Date, DateTime, Local};
use coax_api::conv::ConvType;
use coax_data::ConvStatus;
use coax_api::types::{Name, ConvId};
use ffi;
use fnv::FnvHashMap;
use gdk_pixbuf::{InterpType, Pixbuf};
use gio;
use gtk::{self, Align};
use gtk::prelude::*;
use res;
use util::hash;

#[derive(Clone)]
pub struct Channel {
    ctype:        ConvType,
    image:        gtk::Image,
    name_label:   gtk::Label,
    sub_label:    gtk::Label,
    date_label:   gtk::Label,
    time_label:   gtk::Label,
    channel_row:  gtk::ListBoxRow,
    message_list: gtk::ListBox,
    message_view: gtk::ScrolledWindow,
    model:        FnvHashMap<u64, Message>,
    init:         bool,
    status:       ConvStatus,
    autoscroll:   Rc<AtomicBool>,
    date_lower:   Date<Local>,
    date_upper:   Date<Local>
}

impl Channel {
    pub fn one_to_one(dt: &DateTime<Local>, id: &ConvId, u: &mut res::User) -> Channel {
        let ch = Channel::new(ConvType::OneToOne, dt, id, &Some(Name::new(u.name.clone())), u.icon_medium());
        u.handle.as_ref().map(|h| ch.set_sub(h.as_str()));
        ch
    }

    pub fn group(dt: &DateTime<Local>, id: &ConvId, n: &Option<Name>, len: usize) -> Channel {
        let img = {
            let buf = Pixbuf::new_from_resource("/coax/icons/bubbles.png").unwrap();
            let ico = buf.scale_simple(48, 48, InterpType::Bilinear).unwrap();
            gtk::Image::new_from_pixbuf(Some(&ico))
        };
        let ch = Channel::new(ConvType::Group, dt, id, n, img);
        ch.set_sub(&format!("{} participants", len));
        ch
    }

    fn new(ct: ConvType, dt: &DateTime<Local>, id: &ConvId, n: &Option<Name>, img: gtk::Image) -> Channel {
        let channel_row = gtk::ListBoxRow::new();
        channel_row.set_name("channel-row");
        let grid = gtk::Grid::new();
        grid.set_margin_left(6);
        grid.set_margin_top(6);
        grid.set_margin_right(6);
        grid.set_margin_bottom(6);

        img.set_margin_right(12);
        grid.attach(&img, 0, 0, 1, 2);

        let name_label = gtk::Label::new(None);
        name_label.set_name("channel-name");
        ffi::set_ellipsis(&name_label);
        name_label.set_max_width_chars(64);
        name_label.set_margin_left(6);
        name_label.set_margin_top(6);
        name_label.set_margin_right(6);
        name_label.set_hexpand(true);
        name_label.set_halign(Align::Fill);
        name_label.set_xalign(0.0);
        grid.attach(&name_label, 1, 0, 1, 1);

        let sub_label = gtk::Label::new(None);
        sub_label.set_name("channel-subtitle");
        sub_label.get_style_context().map(|ctx| ctx.add_class("dim-label"));
        ffi::set_ellipsis(&sub_label);
        sub_label.set_max_width_chars(64);
        sub_label.set_margin_left(6);
        sub_label.set_margin_right(6);
        sub_label.set_margin_bottom(6);
        sub_label.set_hexpand(true);
        sub_label.set_halign(Align::Fill);
        sub_label.set_xalign(0.0);
        grid.attach(&sub_label, 1, 1, 1, 1);

        let time_label = gtk::Label::new(None);
        time_label.set_name("channel-time");
        time_label.get_style_context().map(|ctx| ctx.add_class("dim-label"));
        time_label.set_margin_left(6);
        time_label.set_margin_top(6);
        time_label.set_margin_right(6);
        grid.attach(&time_label, 2, 0, 1, 1);

        let date_label = gtk::Label::new(None);
        date_label.set_name("channel-time");
        date_label.get_style_context().map(|ctx| ctx.add_class("dim-label"));
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
            ctype:        ct,
            image:        img,
            name_label:   name_label,
            sub_label:    sub_label,
            date_label:   date_label,
            time_label:   time_label,
            channel_row:  channel_row,
            message_list: message_list,
            message_view: message_view,
            model:        FnvHashMap::default(),
            init:         false,
            status:       ConvStatus::Current,
            autoscroll:   autoscroll,
            date_lower:   dt.date(),
            date_upper:   dt.date()
        };

        ch.set_name(n.as_ref().unwrap_or(&Name::new("N/A")));
        ch.set_time(dt);
        ch
    }

    pub fn conv_type(&self) -> ConvType {
        self.ctype
    }

    pub fn is_init(&self) -> bool {
        self.init
    }

    pub fn set_init(&mut self) {
        self.init = true
    }

    pub fn status(&self) -> ConvStatus {
        self.status
    }

    pub fn set_status(&mut self, s: ConvStatus) {
        self.status = s
    }

    pub fn channel_row(&self) -> &gtk::ListBoxRow {
        &self.channel_row
    }

    pub fn is_selected(&self) -> bool {
        self.channel_row.is_selected()
    }

    pub fn set_unread(&self) {
        self.channel_row.get_style_context().map(|ctx| ctx.add_class("unread"));
    }

    pub fn set_read(&self) {
        self.channel_row.get_style_context().map(|ctx| ctx.remove_class("unread"));
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

    pub fn push_front_msg(&mut self, id: &str, m: Message) {
        if let Some(time) = m.datetime() {
            if time.date() != self.date_lower && !self.model.is_empty() {
                self.push_front_date()
            }
            self.date_lower = time.date()
        }
        self.message_list.prepend(m.row());
        self.model.insert(hash(id), m);
    }

    pub fn push_msg(&mut self, id: &str, m: Message) {
        if let Some(time) = m.datetime() {
            if time.date() != self.date_upper || self.model.is_empty() {
                let dm = Message::date(time.date());
                self.message_list.add(dm.row())
            }
            self.update_time(time)
        }
        self.message_list.add(m.row());
        self.model.insert(hash(id), m);
    }

    pub fn insert_delivery_date(&mut self, k: &str, d: Date<Local>) {
        let ix = self.get_msg(k).map(Message::index).unwrap_or(-1);
        if ix != -1 && d != self.date_upper {
            let dm = Message::date(d);
            self.date_upper = d;
            self.message_list.insert(dm.row(), ix);
        }
    }

    pub fn push_front_date(&mut self) {
        let dm = Message::date(self.date_lower);
        self.message_list.prepend(dm.row())
    }

    pub fn update_time(&mut self, dt: &DateTime<Local>) {
        self.date_upper = dt.date();
        self.set_time(dt);
        self.update_tstamp(dt.timestamp())
    }

    fn update_tstamp(&self, dt: i64) {
        ffi::set_data(&self.channel_row, &ffi::TSTAMP, dt)
    }

    fn set_time(&self, dt: &DateTime<Local>) {
        let tstr = dt.format("%T").to_string();
        let dstr = dt.format("%F").to_string();
        self.time_label.set_text(&tstr);
        self.date_label.set_text(&dstr)
    }

    fn set_name(&self, n: &Name) {
        self.name_label.set_text(n.as_str())
    }

    fn set_sub(&self, txt: &str) {
        self.sub_label.set_text(txt)
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Text(TextMessage),
    Image(Image),
    Date(DateHeader),
    System(SystemMessage)
}

impl Message {
    pub fn text(dt: Option<DateTime<Local>>, u: &mut res::User, txt: &str) -> Message {
        Message::Text(TextMessage::new(dt, u, txt))
    }

    pub fn date(d: Date<Local>) -> Message {
        Message::Date(DateHeader::new(d))
    }

    pub fn system(dt: DateTime<Local>, txt: &str) -> Message {
        Message::System(SystemMessage::new(dt, txt))
    }

    pub fn row(&self) -> &gtk::ListBoxRow {
        match *self {
            Message::Text(ref msg)   => &msg.row,
            Message::Image(ref msg)  => &msg.row,
            Message::Date(ref msg)   => &msg.row,
            Message::System(ref msg) => &msg.row
        }
    }

    pub fn index(&self) -> i32 {
        match *self {
            Message::Text(ref msg)   => msg.row.get_index(),
            Message::Image(ref msg)  => msg.row.get_index(),
            Message::Date(ref msg)   => msg.row.get_index(),
            Message::System(ref msg) => msg.row.get_index()
        }
    }

    pub fn datetime(&self) -> Option<&DateTime<Local>> {
        match *self {
            Message::Text(ref msg)   => msg.datetime.as_ref(),
            Message::Image(ref msg)  => Some(&msg.datetime),
            Message::System(ref msg) => Some(&msg.datetime),
            Message::Date(_)         => None
        }
    }
}

#[derive(Debug, Clone)]
pub struct TextMessage {
    datetime:  Option<DateTime<Local>>,
    row:       gtk::ListBoxRow,
    grid:      gtk::Grid,
    time:      gtk::Label,
    delivered: bool
}

impl TextMessage {
    pub fn new(dt: Option<DateTime<Local>>, u: &mut res::User, txt: &str) -> TextMessage {
        let row = gtk::ListBoxRow::new();
        let grid = gtk::Grid::new();
        grid.set_margin_left(6);
        grid.set_margin_top(6);
        grid.set_margin_right(6);
        grid.set_margin_bottom(6);
        grid.set_column_spacing(12);

        grid.attach(&u.icon_small(), 0, 0, 1, 1);

        let nme = gtk::Label::new(Some(u.name.as_ref()));
        nme.set_name("text-sender");
        nme.set_tooltip_text(u.handle.as_ref().map(|h| h.as_ref()));
        nme.set_halign(Align::Start);
        grid.attach(&nme, 1, 0, 1, 1);

        let time = gtk::Label::new(None);
        time.set_name("text-time");
        time.get_style_context().map(|ctx| ctx.add_class("dim-label"));
        if let Some(t) = dt {
            time.set_text(&t.format("%T").to_string());
            time.set_tooltip_text(Some(t.format("%F").to_string().as_ref()))
        }
        grid.attach(&time, 2, 0, 1, 1);

        let lbl = gtk::Label::new(Some(txt));
        lbl.set_selectable(true);
        lbl.set_margin_top(6);
        lbl.set_margin_bottom(6);
        lbl.set_hexpand(true);
        lbl.set_valign(Align::Start);
        lbl.set_halign(Align::Fill);
        lbl.set_xalign(0.0);
        lbl.set_line_wrap(true);
        grid.attach(&lbl, 1, 1, 1, 1);

        row.add(&grid);
        row.show_all();

        TextMessage {
            datetime:  dt,
            row:       row,
            grid:      grid,
            time:      time,
            delivered: false
        }
    }

    pub fn set_delivered(&mut self, dt: DateTime<Local>) {
        if self.delivered {
            return ()
        }
        let check = gtk::Label::new(Some("\u{2705}"));
        check.set_valign(Align::Start);
        check.get_style_context().map(|ctx| ctx.add_class("dim-label"));
        let tooltip = dt.format("Delivered at %T").to_string();
        check.set_tooltip_text(Some(tooltip.as_ref()));
        check.show();
        self.grid.attach(&check, 2, 1, 1, 1);
        self.delivered = true
    }

    pub fn set_time(&mut self, dt: DateTime<Local>) {
        if let Some(w) = self.grid.get_child_at(2, 0) {
            self.grid.remove(&w)
        }
        let tstr = dt.format("%T").to_string();
        self.time.set_text(&tstr);
        self.time.set_tooltip_text(Some(dt.format("%F").to_string().as_ref()));
        self.datetime = Some(dt);
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

#[derive(Debug, Clone)]
pub struct Image {
    datetime: DateTime<Local>,
    row:      gtk::ListBoxRow,
    image:    gtk::DrawingArea,
    grid:     gtk::Grid,
    time:     gtk::Label
}

impl Image {
    pub fn new(dt: DateTime<Local>, u: &mut res::User, img: gtk::DrawingArea, win: Option<gtk::Window>) -> Image {
        let row = gtk::ListBoxRow::new();
        let grid = gtk::Grid::new();
        grid.set_margin_left(6);
        grid.set_margin_top(6);
        grid.set_margin_right(6);
        grid.set_margin_bottom(6);
        grid.set_column_spacing(12);

        let ico = u.icon_small();
        grid.attach(&ico, 0, 0, 1, 1);

        let nme = gtk::Label::new(Some(u.name.as_ref()));
        nme.set_name("text-sender");
        nme.set_tooltip_text(u.handle.as_ref().map(|h| h.as_ref()));
        nme.set_halign(Align::Start);
        grid.attach(&nme, 1, 0, 1, 1);

        let time = gtk::Label::new(None);
        time.set_name("text-time");
        time.get_style_context().map(|ctx| ctx.add_class("dim-label"));
        let tstr = dt.format("%T").to_string();
        time.set_text(&tstr);
        time.set_tooltip_text(Some(dt.format("%F").to_string().as_ref()));
        grid.attach(&time, 2, 0, 1, 1);

        img.set_margin_top(6);
        img.set_margin_bottom(6);
        img.set_hexpand(true);
        img.set_vexpand(true);
        grid.attach(&img, 1, 1, 1, 1);

        let menu = gio::Menu::new();
        menu.append("Save as ...", "image.save");

        let menu_actions = gio::SimpleActionGroup::new();

        let save_action = gio::SimpleAction::new("save", None);
        save_action.connect_activate(move |_, _| {
            let dialog = gtk::FileChooserDialog::new(Some("Save as ..."), win.as_ref(), gtk::FileChooserAction::Save);
            dialog.add_button("Cancel", gtk::ResponseType::Cancel.into());
            dialog.add_button("Save", gtk::ResponseType::Accept.into());
            dialog.set_do_overwrite_confirmation(true);
            if dialog.run() == gtk::ResponseType::Accept.into() {
                println!("Saving image to {:?}", dialog.get_filename())
            }
            dialog.destroy();
        });

        menu_actions.insert(&save_action);

        let menu_btn = gtk::MenuButton::new();
        menu_btn.get_style_context().map(|ctx| ctx.add_class("dim-label"));
        menu_btn.set_valign(Align::Start);
        menu_btn.set_relief(gtk::ReliefStyle::None);
        menu_btn.insert_action_group("image", Some(&menu_actions));
        menu_btn.set_menu_model(Some(&menu));
        grid.attach(&menu_btn, 2, 1, 1, 1);

        row.add(&grid);
        row.show_all();

        Image {
            datetime: dt,
            row:      row,
            image:    img,
            grid:     grid,
            time:     time
        }
    }

    pub fn start_spinner(&self) {
        let spinner = gtk::Spinner::new();
        spinner.set_margin_left(12);
        spinner.set_margin_top(12);
        spinner.set_margin_right(12);
        spinner.set_margin_bottom(12);
        spinner.set_hexpand(true);
        spinner.set_vexpand(true);
        spinner.set_size_request(32, 32);
        spinner.start();
        if let Some(w) = self.grid.get_child_at(1, 1) {
            self.grid.remove(&w)
        }
        spinner.show();
        self.grid.attach(&spinner, 1, 1, 1, 1)
    }

    pub fn stop_spinner(&self) {
        if let Some(w) = self.grid.get_child_at(1, 1) {
            self.grid.remove(&w)
        }
        self.grid.attach(&self.image, 1, 1, 1, 1)
    }
}

#[derive(Debug, Clone)]
pub struct DateHeader { row: gtk::ListBoxRow }

impl DateHeader {
    pub fn new(d: Date<Local>) -> DateHeader {
        let row = gtk::ListBoxRow::new();
        row.set_name("date-header");

        let tstr = d.format("%F").to_string();
        let time = gtk::Label::new(Some(tstr.as_ref()));
        time.set_name("date-text");
        time.get_style_context().map(|ctx| ctx.add_class("dim-label"));
        time.set_margin_left(6);
        time.set_margin_top(6);
        time.set_margin_right(6);
        time.set_margin_bottom(6);
        time.set_hexpand(true);
        time.set_halign(Align::Fill);

        row.add(&time);
        row.show_all();

        DateHeader { row: row }
    }
}

#[derive(Debug, Clone)]
pub struct SystemMessage {
    datetime: DateTime<Local>,
    row:      gtk::ListBoxRow
}

impl SystemMessage {
    pub fn new(dt: DateTime<Local>, txt: &str) -> SystemMessage {
        let row = gtk::ListBoxRow::new();
        row.set_name("system-header");

        let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        hbox.set_margin_left(6);
        hbox.set_margin_top(6);
        hbox.set_margin_right(6);
        hbox.set_margin_bottom(6);

        let hdr = gtk::Label::new(Some("Note"));
        hdr.set_name("system-category");
        hdr.get_style_context().map(|ctx| ctx.add_class("dim-label"));
        hdr.set_margin_right(6);
        hdr.set_halign(Align::Start);
        hbox.add(&hdr);

        let lbl = gtk::Label::new(Some(txt));
        lbl.set_hexpand(true);
        lbl.set_halign(Align::Fill);
        lbl.set_line_wrap(true);
        hbox.add(&lbl);

        let tstr = dt.format("%T").to_string();
        let time = gtk::Label::new(Some(tstr.as_ref()));
        time.set_name("system-time");
        time.get_style_context().map(|ctx| ctx.add_class("dim-label"));
        time.set_tooltip_text(Some(dt.format("%F").to_string().as_ref()));
        time.set_margin_left(6);
        hbox.add(&time);

        row.add(&hbox);
        row.show_all();

        SystemMessage {
            datetime: dt,
            row:      row
        }
    }
}

