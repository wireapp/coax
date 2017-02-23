use std::fs::DirBuilder;
use std::path::PathBuf;

use coax_actor::config;
use coax_actor::error::Error;
use coax_data::profiles::{Profile, ProfileDb};
use gtk::{self, Align};
use gtk::prelude::*;
use res;
use slog::Logger;

pub fn open_profile_db(g: &Logger, cfg: &config::Main) -> Result<ProfileDb, Error> {
    let mut root = PathBuf::from(&cfg.data.root);
    if !root.exists() {
        DirBuilder::new().create(&root)?;
    }
    root.push("profiles.db");
    let ps = root.to_str().ok_or(Error::Message("/data/root contains invalid utf-8"))?;
    let db = ProfileDb::open(&g, ps)?;
    db.setup_schema()?;
    Ok(db)
}

pub fn load_profiles<'a>(db: &ProfileDb) -> Result<Vec<Profile<'a>>, Error> {
    db.select().map_err(From::from)
}

#[derive(Clone)]
pub struct ProfileView {
    vbox:   gtk::Box,
    name:   gtk::Label,
    handle: gtk::Label,
    icon:   gtk::Image
}

impl ProfileView {
    pub fn new(usr: &mut res::User) -> ProfileView {
        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);

        let name = gtk::Label::new(None);
        name.set_markup(&format!("<span size=\"x-large\"><b>{}</b></span>", usr.name));
        name.set_halign(Align::Center);
        name.set_hexpand(true);
        vbox.add(&name);

        let handle = gtk::Label::new(usr.handle.as_ref().map(|s| s.as_ref()));
        handle.set_halign(Align::Center);
        handle.set_hexpand(true);
        vbox.add(&handle);

        let sep = gtk::Separator::new(gtk::Orientation::Horizontal);
        sep.set_margin_left(6);
        sep.set_margin_right(6);
        vbox.add(&sep);

        let img = usr.icon_large();
        img.set_margin_left(6);
        img.set_margin_top(6);
        img.set_margin_right(6);
        img.set_margin_bottom(12);
        vbox.add(&img);
        vbox.show_all();

        ProfileView {
            vbox:   vbox,
            name:   name,
            handle: handle,
            icon:   img
        }
    }

    pub fn vbox(&self) -> &gtk::Box {
        &self.vbox
    }
}
