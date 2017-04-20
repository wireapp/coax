use chashmap::{CHashMap, WriteGuard};
use coax_api::types::UserId;
use coax_data;
use ffi;
use gdk_pixbuf::{InterpType, Pixbuf, PixbufLoader};
use glib::translate::FromGlibPtrNone;
use glib_sys::gpointer;
use gtk;
use gtk_sys;

pub struct User {
    pub id:           UserId,
    pub name:         String,
    pub handle:       Option<String>,
        icon_small:   Pixbuf,
        small_icons:  Vec<gpointer>,
        icon_medium:  Pixbuf,
        medium_icons: Vec<gpointer>,
        icon_large:   Pixbuf,
        large_icons:  Vec<gpointer>,
}

impl User {
    pub fn set_icon(&mut self, data: &[u8]) {
        let loader = PixbufLoader::new();
        loader.loader_write(data).unwrap(); // TODO
        loader.close().unwrap(); // TODO
        let buf = loader.get_pixbuf().unwrap(); // TODO
        self.icon_small  = buf.scale_simple(32, 32, InterpType::Bilinear).unwrap(); // TODO
        self.icon_medium = buf.scale_simple(48, 48, InterpType::Bilinear).unwrap(); // TODO
        self.icon_large  = buf.scale_simple(200, 200, InterpType::Bilinear).unwrap(); // TODO
        for p in self.small_icons.iter().filter(|p| !p.is_null()) {
            let img = unsafe {
                gtk::Image::from_glib_none(*p as *mut gtk_sys::GtkImage)
            };
            img.set_from_pixbuf(Some(&self.icon_small))
        }
        for p in self.medium_icons.iter().filter(|p| !p.is_null()) {
            let img = unsafe {
                gtk::Image::from_glib_none(*p as *mut gtk_sys::GtkImage)
            };
            img.set_from_pixbuf(Some(&self.icon_medium))
        }
        for p in self.large_icons.iter().filter(|p| !p.is_null()) {
            let img = unsafe {
                gtk::Image::from_glib_none(*p as *mut gtk_sys::GtkImage)
            };
            img.set_from_pixbuf(Some(&self.icon_large))
        }
    }

    pub fn icon_medium(&mut self) -> gtk::Image {
        self.cleanup();
        let img = gtk::Image::new_from_pixbuf(Some(&self.icon_medium));
        let ptr = ffi::add_weak_ptr(&img);
        self.medium_icons.push(ptr);
        img
    }

    pub fn icon_small(&mut self) -> gtk::Image {
        self.cleanup();
        let img = gtk::Image::new_from_pixbuf(Some(&self.icon_small));
        let ptr = ffi::add_weak_ptr(&img);
        self.small_icons.push(ptr);
        img
    }

    pub fn icon_large(&mut self) -> gtk::Image {
        self.cleanup();
        let img = gtk::Image::new_from_pixbuf(Some(&self.icon_large));
        let ptr = ffi::add_weak_ptr(&img);
        self.large_icons.push(ptr);
        img
    }

    pub fn update(&mut self, u: &coax_data::User) {
        self.name = ffi::escape(u.name.as_str()).to_string_lossy().into_owned();
        self.handle = u.handle.as_ref().map(|h| format!("@{}", ffi::escape(h.as_str()).to_string_lossy()));
    }

    fn cleanup(&mut self) {
        self.small_icons.retain(|ptr| !ptr.is_null());
        self.medium_icons.retain(|ptr| !ptr.is_null());
        self.large_icons.retain(|ptr| !ptr.is_null())
    }
}

impl<'a, 'b> From<&'b coax_data::User<'a>> for User {
    fn from(u: &'b coax_data::User<'a>) -> User {
        let buf = Pixbuf::new_from_resource("/coax/icons/user.png").unwrap();
        let ico32  = buf.scale_simple(32, 32, InterpType::Bilinear).unwrap(); // TODO
        let ico48  = buf.scale_simple(48, 48, InterpType::Bilinear).unwrap(); // TODO
        let ico120 = buf.scale_simple(200, 200, InterpType::Bilinear).unwrap(); // TODO
        User {
            id:           u.id.clone(),
            name:         ffi::escape(u.name.as_str()).to_string_lossy().into_owned(),
            handle:       u.handle.as_ref().map(|h| format!("@{}", ffi::escape(h.as_str()).to_string_lossy())),
            icon_small:   ico32,
            small_icons:  Vec::new(),
            icon_medium:  ico48,
            medium_icons: Vec::new(),
            icon_large:   ico120,
            large_icons:  Vec::new()
        }
    }
}

pub struct Resources {
    users: CHashMap<UserId, User>
}

impl Resources {
    pub fn new() -> Resources {
        Resources { users: CHashMap::new() }
    }

    pub fn has_user(&self, u: &UserId) -> bool {
        self.users.contains_key(u)
    }

    pub fn add_user(&self, u: &coax_data::User) {
        if let Some(mut x) = self.users.get_mut(&u.id) {
            x.update(u)
        } else {
            self.users.insert(u.id.clone(), u.into());
        }
    }

    pub fn user_mut(&self, id: &UserId) -> Option<WriteGuard<UserId, User>> {
        self.users.get_mut(id)
    }
}
