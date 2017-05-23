use chashmap::{CHashMap, WriteGuard};
use coax_api::types::{Name, Handle, UserId};
use coax_data;
use ffi;
use gdk_pixbuf::{InterpType, Pixbuf, PixbufLoader};
use signals::Signal;

pub enum Change {
    IconSmall(Pixbuf),
    IconMedium(Pixbuf),
    IconLarge(Pixbuf),
    Name(String),
    Handle(String)
}

pub struct User {
    pub id:          UserId,
    pub name:        String,
    pub handle:      Option<String>,
    pub sig_change:  Signal<'static, Change, ()>,
    pub icon_small:  Pixbuf,
    pub icon_medium: Pixbuf,
    pub icon_large:  Pixbuf
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
        self.sig_change.emit(Change::IconSmall(self.icon_small.clone()));
        self.sig_change.emit(Change::IconMedium(self.icon_medium.clone()));
        self.sig_change.emit(Change::IconLarge(self.icon_large.clone()));
    }

    pub fn update(&mut self, u: &coax_data::User) {
        self.set_name(&u.name);
        u.handle.as_ref().map(|h| self.set_handle(h));
    }

    pub fn set_name(&mut self, n: &Name) {
        self.name = ffi::escape(n.as_str()).to_string_lossy().into_owned();
        self.sig_change.emit(Change::Name(n.as_str().into()));
    }

    pub fn set_handle(&mut self, h: &Handle) {
        self.handle = Some(format!("@{}", ffi::escape(h.as_str()).to_string_lossy()));
        self.sig_change.emit(Change::Handle(h.as_str().into()));
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
            icon_medium:  ico48,
            icon_large:   ico120,
            sig_change:   Signal::new()
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
