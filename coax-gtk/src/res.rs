use std::collections::HashMap;
use coax_api::types::{UserId, ConvId};
use coax_data;
use ffi;
use gdk_pixbuf::{InterpType, Pixbuf, PixbufLoader};
use glib::translate::FromGlibPtr;
use glib_sys::gpointer;
use gtk;
use gtk_sys;

pub struct User {
    pub id:    UserId,
    pub name:  String,
        pict:  Pixbuf,
        picts: Vec<gpointer>,
        icon:  Pixbuf,
        icons: Vec<gpointer>
}

impl User {
    pub fn set_icon(&mut self, data: &[u8]) {
        let loader = PixbufLoader::new();
        loader.loader_write(data).unwrap(); // TODO
        loader.close().unwrap(); // TODO
        let buf   = loader.get_pixbuf().unwrap(); // TODO
        self.icon = buf.scale_simple(32, 32, InterpType::Bilinear).unwrap(); // TODO
        self.pict = buf.scale_simple(100, 100, InterpType::Bilinear).unwrap(); // TODO
        for p in self.picts.iter().filter(|p| !p.is_null()) {
            let img = unsafe {
                gtk::Image::from_glib_none(*p as *mut gtk_sys::GtkImage)
            };
            img.set_from_pixbuf(Some(&self.pict))
        }
        for p in self.icons.iter().filter(|p| !p.is_null()) {
            let img = unsafe {
                gtk::Image::from_glib_none(*p as *mut gtk_sys::GtkImage)
            };
            img.set_from_pixbuf(Some(&self.icon))
        }
    }

    pub fn pict(&mut self) -> gtk::Image {
        self.cleanup();
        let img = gtk::Image::new_from_pixbuf(Some(&self.pict));
        let ptr = ffi::add_weak_ptr(&img);
        self.picts.push(ptr);
        img
    }

    pub fn icon(&mut self) -> gtk::Image {
        self.cleanup();
        let img = gtk::Image::new_from_pixbuf(Some(&self.icon));
        let ptr = ffi::add_weak_ptr(&img);
        self.icons.push(ptr);
        img
    }

    fn cleanup(&mut self) {
        self.picts.retain(|ptr| !ptr.is_null());
        self.icons.retain(|ptr| !ptr.is_null())
    }
}

impl<'a, 'b> From<&'b coax_data::User<'a>> for User {
    fn from(u: &'b coax_data::User<'a>) -> User {
        let buf = Pixbuf::new_from_resource("/coax/icons/user.png").unwrap();
        let ico = buf.scale_simple(32, 32, InterpType::Bilinear).unwrap(); // TODO
        let pic = buf.scale_simple(100, 100, InterpType::Bilinear).unwrap(); // TODO
        User {
            id:    u.id.clone(),
            name:  ffi::escape(u.name.as_str()).to_string_lossy().into_owned(),
            pict:  pic,
            picts: Vec::new(),
            icon:  ico,
            icons: Vec::new()
        }
    }
}

pub struct Conv {
    pub name: gtk::Label,
    pub icon: gtk::Image
}

pub struct Resources {
    pub user: HashMap<UserId, User>,
    pub conv: HashMap<ConvId, Conv>
}

impl Resources {
    pub fn new() -> Resources {
        Resources {
            user: HashMap::new(),
            conv: HashMap::new()
        }
    }

    pub fn has_user(&mut self, u: &UserId) -> bool {
        self.user.contains_key(u)
    }

    pub fn add_user(&mut self, u: &coax_data::User) {
        self.user.insert(u.id.clone(), u.into());
    }

    pub fn user(&self, id: &UserId) -> Option<&User> {
        self.user.get(id)
    }

    pub fn user_mut(&mut self, id: &UserId) -> Option<&mut User> {
        self.user.get_mut(id)
    }
}
