use std::borrow::{Borrow, Cow};
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::{self, Entry};
use std::io::Write;
use std::iter::FromIterator;

use chrono::{DateTime, UTC};
use json::{ToJson, Encoder, EncodeResult};
use json::{FromJson, Decoder, DecodeError, DecodeResult, Utf8Buffer};
use json::ast::{Json, Ref};
use proteus::keys as proteus;
use rustc_serialize::base64::{ToBase64, FromBase64};
use types::*;
use util;

json_str_type!(Model);

// Client ///////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub struct Client<'a> {
    pub id:           ClientId<'a>,
    pub class:        Option<Class>,
    pub time:         Option<DateTime<UTC>>,
    pub ctype:        Option<Type>,
    pub cookie_label: Option<Label<'a>>,
    pub label:        Option<Label<'a>>,
    pub model:        Option<Model<'a>>
}

impl<'a> Client<'a> {
    pub fn new(id: ClientId<'a>, cl: Class) -> Client<'a> {
        Client {
            id:           id,
            class:        Some(cl),
            time:         None,
            ctype:        None,
            cookie_label: None,
            label:        None,
            model:        None
        }
    }

    pub fn set_time(&mut self, t: DateTime<UTC>) {
        self.time = Some(t)
    }

    pub fn set_type(&mut self, t: Type) {
        self.ctype = Some(t)
    }

    pub fn set_label(&mut self, l: Label<'a>) {
        self.label = Some(l)
    }

    pub fn set_cookie_label(&mut self, l: Label<'a>) {
        self.cookie_label = Some(l)
    }

    pub fn set_model(&mut self, m: Model<'a>) {
        self.model = Some(m)
    }

    pub fn from_json(j: &Json) -> DecodeResult<Client<'a>> {
        let r  = Ref::new(j);
        let id = r.get("id").string().map(|s| ClientId::new(String::from(s)));
        Ok(Client {
            id:           from_some!(id, DecodeError::Expected("id")),
            class:        r.get("class").string().and_then(Class::from_str),
            time:         r.get("time").value().and_then(|t| util::datetime_from_json(t).ok()),
            ctype:        r.get("type").string().and_then(Type::from_str),
            cookie_label: r.get("cookie").string().map(|s| Label::new(String::from(s))),
            label:        r.get("label").string().map(|s| Label::new(String::from(s))),
            model:        r.get("model").string().map(|s| Model::new(String::from(s)))
        })
    }
}

impl<'a> FromJson for Client<'a> {
    fn decode<I: Iterator<Item=char>>(d: &mut Decoder<I>) -> DecodeResult<Self> {
        let mut b = [0; 16];
        let mut u = Utf8Buffer::new(&mut b);
        object! {
            let decoder = d;
            let buffer  = &mut u;
            Client {
                id:           req. "id"     => d.from_json(),
                class:        opt. "class"  => d.from_json(),
                time:         opt. "time"   => d.optional(util::parse_datetime),
                ctype:        opt. "type"   => d.from_json(),
                cookie_label: opt. "cookie" => d.from_json(),
                label:        opt. "label"  => d.from_json(),
                model:        opt. "model"  => d.from_json()
            }
        }
    }
}

// PubClientView ////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub struct PubClientView<'a> {
    pub id:    ClientId<'a>,
    pub class: Class
}

impl<'a> FromJson for PubClientView<'a> {
    fn decode<I: Iterator<Item=char>>(d: &mut Decoder<I>) -> DecodeResult<Self> {
        let mut b = [0; 16];
        let mut u = Utf8Buffer::new(&mut b);
        object! {
            let decoder = d;
            let buffer  = &mut u;
            PubClientView {
                id:    req. "id"    => d.from_json(),
                class: req. "class" => d.from_json()
            }
        }
    }
}

// Signaling keys ///////////////////////////////////////////////////////////

json_bytes_type!(Cipher);
json_bytes_type!(Mac);

#[derive(Clone, Debug)]
pub struct SignalingKeys<'a> {
    pub enc: Cipher<'a>,
    pub mac: Mac<'a>
}

impl<'a> SignalingKeys<'a> {
    pub fn new() -> SignalingKeys<'a> {
        SignalingKeys {
            enc: Cipher::new(proteus::rand_bytes(32)),
            mac: Mac::new(proteus::rand_bytes(32))
        }
    }
}

impl<'a> ToJson for SignalingKeys<'a> {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> EncodeResult<()> {
        e.object()?;
            e.key("enckey")?; e.to_json(&self.enc)?;
            e.key("mackey")?; e.to_json(&self.mac)?;
        e.end()
    }
}

// Client type //////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug)]
pub enum Type {
    Permanent,
    Temporary
}

impl Type {
    pub fn from_str(s: &str) -> Option<Type> {
        match s {
            "temporary" => Some(Type::Temporary),
            "permanent" => Some(Type::Permanent),
            _           => None
        }
    }

    pub fn from_u8(b: u8) -> Option<Type> {
        match b {
            0 => Some(Type::Permanent),
            1 => Some(Type::Temporary),
            _ => None
        }
    }
}

impl Into<u8> for Type {
    fn into(self) -> u8 {
        match self {
            Type::Permanent => 0,
            Type::Temporary => 1
        }
    }
}

impl FromJson for Type {
    fn decode<I: Iterator<Item=char>>(d: &mut Decoder<I>) -> DecodeResult<Self> {
        let s = d.string()?;
        Type::from_str(s.as_str())
            .ok_or(DecodeError::Message("unknown client type"))
    }
}

impl ToJson for Type {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> EncodeResult<()> {
        match *self {
            Type::Temporary => e.string("temporary"),
            Type::Permanent => e.string("permanent")
        }
    }
}

// Client class /////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug)]
pub enum Class {
    Phone,
    Tablet,
    Desktop
}

impl Into<u8> for Class {
    fn into(self) -> u8 {
        match self {
            Class::Phone   => 0,
            Class::Tablet  => 1,
            Class::Desktop => 2
        }
    }
}

impl Class {
    pub fn from_str(s: &str) -> Option<Class> {
        match s {
            "phone"   => Some(Class::Phone),
            "tablet"  => Some(Class::Tablet),
            "desktop" => Some(Class::Desktop),
            _         => None
        }
    }

    pub fn from_u8(b: u8) -> Option<Class> {
        match b {
            0 => Some(Class::Phone),
            1 => Some(Class::Tablet),
            2 => Some(Class::Desktop),
            _ => None
        }
    }
}

impl FromJson for Class {
    fn decode<I: Iterator<Item=char>>(d: &mut Decoder<I>) -> DecodeResult<Self> {
        let s = d.string()?;
        Class::from_str(s.as_str())
            .ok_or(DecodeError::Message("unknown client class"))
    }
}

impl ToJson for Class {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> EncodeResult<()> {
        match *self {
            Class::Phone   => e.string("phone"),
            Class::Tablet  => e.string("tablet"),
            Class::Desktop => e.string("desktop")
        }
    }
}

// User2Clients /////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub struct User2Clients<'a> {
    value: HashMap<UserId, HashSet<ClientId<'a>>>
}

impl<'a> User2Clients<'a> {
    pub fn new() -> User2Clients<'a> {
        User2Clients {
            value: HashMap::new()
        }
    }

    pub fn add(&mut self, u: UserId, c: &'a [ClientId<'a>]) {
        match self.value.entry(u) {
            Entry::Vacant(v) => {
                v.insert(HashSet::from_iter(c.iter().map(|i| i.replicate())));
            }
            Entry::Occupied(mut v) => {
                v.get_mut().extend(c.iter().map(|i| i.replicate()))
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
}

impl<'a> IntoIterator for &'a User2Clients<'a> {
    type Item     = (&'a UserId, &'a HashSet<ClientId<'a>>);
    type IntoIter = hash_map::Iter<'a, UserId, HashSet<ClientId<'a>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.value.iter()
    }
}

impl<'a> ToJson for User2Clients<'a> {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> EncodeResult<()> {
        e.object()?;
            for (u, cc) in self.value.iter() {
                e.key(u.to_string())?;
                e.array()?;
                    for c in cc.iter() {
                        e.string(c.as_str())?
                    }
                e.end()?;
            }
        e.end()
    }
}

impl<'a> FromJson for User2Clients<'a> {
    fn decode<I: Iterator<Item=char>>(d: &mut Decoder<I>) -> DecodeResult<Self> {
        d.object()?;
        let mut u2c = HashMap::new();
        while d.has_more()? {
            if let Some(u) = UserId::from_str(&d.key()?) {
                let clients: Vec<ClientId<'static>> = d.from_json()?;
                u2c.insert(u, HashSet::from_iter(clients));
            } else {
                return Err(DecodeError::Expected("user id"))
            }
        }
        Ok(User2Clients { value: u2c })
    }
}

// Client mismatch //////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub struct ClientMismatch<'a> {
    pub time:      DateTime<UTC>,
    pub redundant: User2Clients<'a>,
    pub missing:   User2Clients<'a>,
    pub deleted:   User2Clients<'a>
}

impl<'a> FromJson for ClientMismatch<'a> {
    fn decode<I: Iterator<Item=char>>(d: &mut Decoder<I>) -> DecodeResult<Self> {
        let mut b = [0; 16];
        let mut u = Utf8Buffer::new(&mut b);
        object! {
            let decoder = d;
            let buffer  = &mut u;
            ClientMismatch {
                time:      req. "time"      => util::parse_datetime(d),
                redundant: req. "redundant" => d.from_json(),
                missing:   req. "missing"   => d.from_json(),
                deleted:   req. "deleted"   => d.from_json()
            }
        }
    }
}


pub mod register {

    use std::borrow::Cow;
    use std::io::Write;
    use json::{ToJson, Encoder, EncodeResult};
    use prekeys::{LastPreKey, PreKey};
    use super::*;

    #[derive(Clone, Debug)]
    pub struct Params<'a> {
        pub prekeys:      Cow<'a, [PreKey]>,
        pub last_prekey:  Cow<'a, LastPreKey>,
        pub sig_keys:     SignalingKeys<'a>,
        pub ctype:        Type,
        pub class:        Class,
        pub cookie_label: Label<'a>,
        pub label:        Option<Label<'a>>,
        pub password:     Option<Password<'a>>,
        pub model:        Option<Model<'a>>
    }

    impl<'a> ToJson for Params<'a> {
        fn encode<W: Write>(&self, e: &mut Encoder<W>) -> EncodeResult<()> {
            e.object()?;
                e.key("prekeys")?;
                e.array()?;
                    for k in self.prekeys.iter() {
                        e.to_json(k)?
                    }
                e.end()?;
                e.key("lastkey")?;  e.to_json(&self.last_prekey)?;
                e.key("sigkeys")?;  e.to_json(&self.sig_keys)?;
                e.key("type")?;     e.to_json(&self.ctype)?;
                e.key("class")?;    e.to_json(&self.class)?;
                e.key("label")?;    e.to_json(&self.label)?;
                e.key("model")?;    e.to_json(&self.model)?;
                e.key("cookie")?;   e.to_json(&self.cookie_label)?;
                e.key("password")?; e.to_json(&self.password)?;
            e.end()
        }
    }

    quick_error! {
        #[derive(Debug)]
        /// Client registration error
        pub enum Error {
            TooManyClients {
                display("user has too many clients registered")
            }
            PasswordRequired {
                display("authentication via password required")
            }
            Other(e: ApiError<'static>) {
                display("api error: {}", e)
                cause(e)
            }
        }
    }

    impl From<ApiError<'static>> for Error {
        fn from(e: ApiError<'static>) -> Error {
            match (e.code, e.label.as_ref()) {
                (403, "too-many-clients") => Error::TooManyClients,
                (403, "missing-auth")     => Error::PasswordRequired,
                _                         => Error::Other(e)
            }
        }
    }
}

pub mod delete {

    use std::io::Write;
    use json::{ToJson, Encoder, EncodeResult};
    use types::*;

    #[derive(Clone, Debug)]
    pub struct Params<'a> {
        pub password: Password<'a>
    }

    impl<'a> Params<'a> {
        pub fn new(p: Password<'a>) -> Params<'a> {
            Params {
                password: p
            }
        }
    }

    impl<'a> ToJson for Params<'a> {
        fn encode<W: Write>(&self, e: &mut Encoder<W>) -> EncodeResult<()> {
            e.object()?;
                e.key("password")?; e.to_json(&self.password)?;
            e.end()
        }
    }

    quick_error! {
        #[derive(Debug)]
        /// Client delete error
        pub enum Error {
            NotFound {
                display("client not found")
            }
            Other(e: ApiError<'static>) {
                display("api error: {}", e)
                cause(e)
            }
        }
    }

    impl From<ApiError<'static>> for Error {
        fn from(e: ApiError<'static>) -> Error {
            match e.code {
                404 => Error::NotFound,
                _   => Error::Other(e)
            }
        }
    }
}

