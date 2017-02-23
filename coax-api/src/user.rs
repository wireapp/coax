use std::borrow::{Borrow, Cow};
use std::io::Write;

use chrono::{DateTime, UTC};
use json::{ToJson, Encoder, EncodeResult};
use json::{FromJson, Decoder, DecodeError, DecodeResult, Utf8Buffer};
use json::ast::{Json, Ref};
use types::*;
use util;

// User /////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub struct User<'a> {
    pub id:      UserId,
    pub name:    Name<'a>,
    pub handle:  Option<Handle<'a>>,
    pub email:   Option<Email<'a>>,
    pub phone:   Option<Phone<'a>>,
    pub service: Option<ServiceRef>,
    pub deleted: Option<bool>,
    pub assets:  Cow<'a, [Asset<'a>]>
}

impl<'a> User<'a> {
    pub fn new(id: UserId, name: Name<'a>) -> User<'a> {
        User {
            id:      id,
            name:    name,
            handle:  None,
            email:   None,
            phone:   None,
            service: None,
            deleted: None,
            assets:  Cow::Owned(Vec::new())
        }
    }

    pub fn set_handle(&mut self, h: Handle<'a>) {
        self.handle = Some(h)
    }

    pub fn set_email(&mut self, e: Email<'a>) {
        self.email = Some(e)
    }

    pub fn set_phone(&mut self, p: Phone<'a>) {
        self.phone = Some(p)
    }

    pub fn set_service(&mut self, s: ServiceRef) {
        self.service = Some(s)
    }

    pub fn set_deleted(&mut self, d: bool) {
        self.deleted = Some(d)
    }

    pub fn add_asset(&mut self, a: Asset<'a>) {
        self.assets.to_mut().push(a)
    }

    pub fn from_json(j: &Json) -> DecodeResult<User<'a>> {
        let r = Ref::new(j);
        let i = r.get("id").string().and_then(UserId::from_str);
        let n = r.get("name").string().map(|s| Name::new(String::from(s)));
        let h = r.get("handle").string().map(|s| Handle::new(String::from(s)));
        let e = r.get("email").string().map(|s| Email::new(String::from(s)));
        let p = r.get("phone").string().map(|s| Phone::new(String::from(s)));
        let s = r.get("service").value().map(ServiceRef::from_json);
        let a = util::map_json_slice(r.get("assets"), Asset::from_json).map(Cow::Owned)?;
        Ok(User {
            id:      from_some!(i, DecodeError::Expected("id")),
            name:    from_some!(n, DecodeError::Expected("name")),
            handle:  h,
            email:   e,
            phone:   p,
            service: from_some_ok!(s),
            deleted: r.get("deleted").bool(),
            assets:  a
        })
    }
}

impl<'a> FromJson for User<'a> {
    fn decode<I: Iterator<Item=char>>(d: &mut Decoder<I>) -> DecodeResult<Self> {
        let mut b = [0; 16];
        let mut u = Utf8Buffer::new(&mut b);
        object! {
            let decoder = d;
            let buffer  = &mut u;
            User {
                id:      req. "id"      => d.from_json(),
                name:    req. "name"    => d.from_json(),
                handle:  opt. "handle"  => d.optional(Decoder::from_json),
                email:   opt. "email"   => d.optional(Decoder::from_json),
                phone:   opt. "phone"   => d.optional(Decoder::from_json),
                service: opt. "service" => d.optional(Decoder::from_json),
                deleted: opt. "deleted" => d.optional(Decoder::from_json),
                assets:  req. "assets"  => array!(d, d.from_json()).map(Cow::Owned)
            }
        }
    }
}

// User update //////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub struct UserUpdate<'a> {
    pub id:     UserId,
    pub name:   Option<Name<'a>>,
    pub handle: Option<Handle<'a>>,
    pub email:  Option<Email<'a>>,
    pub phone:  Option<Phone<'a>>,
    pub assets: Option<Cow<'a, [Asset<'a>]>>
}

impl<'a> UserUpdate<'a> {
    pub fn new(id: UserId) -> UserUpdate<'a> {
        UserUpdate {
            id:      id,
            name:    None,
            handle:  None,
            email:   None,
            phone:   None,
            assets:  None
        }
    }

    pub fn from_json(j: &Json) -> DecodeResult<UserUpdate<'a>> {
        let r = Ref::new(j);
        let i = r.get("id").string().and_then(UserId::from_str);
        let n = r.get("name").string().map(|s| Name::new(String::from(s)));
        let h = r.get("handle").string().map(|s| Handle::new(String::from(s)));
        let e = r.get("email").string().map(|s| Email::new(String::from(s)));
        let p = r.get("phone").string().map(|s| Phone::new(String::from(s)));
        let a = r.get("assets").value()
                 .map(|j| util::map_json_slice(Ref::new(j), Asset::from_json))
                 .map(|a| a.map(Cow::Owned));
        Ok(UserUpdate {
            id:     from_some!(i, DecodeError::Expected("id")),
            name:   n,
            handle: h,
            email:  e,
            phone:  p,
            assets: from_some_ok!(a)
        })
    }
}

// Asset ////////////////////////////////////////////////////////////////////

json_str_type!(AssetKey);
json_str_type!(AssetToken);

#[derive(Clone, Debug)]
pub enum AssetType { Image }

impl AssetType {
    pub fn from_str(s: &str) -> Option<AssetType> {
        match s {
            "image" => Some(AssetType::Image),
            _       => None
        }
    }
}

impl ToJson for AssetType {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> EncodeResult<()> {
        match *self {
            AssetType::Image => e.string("image")
        }
    }
}

impl FromJson for AssetType {
    fn decode<I: Iterator<Item=char>>(d: &mut Decoder<I>) -> DecodeResult<Self> {
        let s = d.string()?;
        AssetType::from_str(s.as_str()).ok_or(DecodeError::Expected("asset type"))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AssetSize {
    Preview,
    Complete
}

impl AssetSize {
    pub fn from_str(s: &str) -> Option<AssetSize> {
        match s {
            "preview"  => Some(AssetSize::Preview),
            "complete" => Some(AssetSize::Complete),
            _          => None
        }
    }
}

impl ToJson for AssetSize {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> EncodeResult<()> {
        match *self {
            AssetSize::Preview  => e.string("preview"),
            AssetSize::Complete => e.string("complete")
        }
    }
}

impl FromJson for AssetSize {
    fn decode<I: Iterator<Item=char>>(d: &mut Decoder<I>) -> DecodeResult<Self> {
        let s = d.string()?;
        AssetSize::from_str(s.as_str()).ok_or(DecodeError::Expected("asset size"))
    }
}

#[derive(Clone, Debug)]
pub struct Asset<'a> {
    pub key:  AssetKey<'a>,
    pub typ:  AssetType,
    pub size: AssetSize
}

impl<'a> Asset<'a> {
    pub fn new(k: AssetKey<'a>, t: AssetType, s: AssetSize) -> Asset<'a> {
        Asset {
            key:  k,
            typ:  t,
            size: s
        }
    }

    pub fn from_json(j: &Json) -> DecodeResult<Asset<'a>> {
        let r = Ref::new(j);
        let k = r.get("key").string().map(|s| AssetKey::new(String::from(s)));
        let t = r.get("type").string().and_then(AssetType::from_str);
        let s = r.get("size").string().and_then(AssetSize::from_str);
        Ok(Asset {
            key:  from_some!(k, DecodeError::Expected("asset key")),
            typ:  from_some!(t, DecodeError::Expected("asset type")),
            size: from_some!(s, DecodeError::Expected("asset size"))
        })
    }
}

impl<'a> FromJson for Asset<'a> {
    fn decode<I: Iterator<Item=char>>(d: &mut Decoder<I>) -> DecodeResult<Self> {
        let mut b = [0; 8];
        let mut u = Utf8Buffer::new(&mut b);
        object! {
            let decoder = d;
            let buffer  = &mut u;
            Asset {
                key:  req. "key"  => d.from_json(),
                typ:  req. "type" => d.from_json(),
                size: req. "size" => d.from_json()
            }
        }
    }
}

// User connections /////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ConnectStatus {
    Accepted,
    Blocked,
    Pending,
    Ignored,
    Sent,
    Cancelled
}

impl ConnectStatus {
    pub fn from_str(s: &str) -> Option<ConnectStatus> {
        match s {
            "accepted"  => Some(ConnectStatus::Accepted),
            "blocked"   => Some(ConnectStatus::Blocked),
            "pending"   => Some(ConnectStatus::Pending),
            "ignored"   => Some(ConnectStatus::Ignored),
            "sent"      => Some(ConnectStatus::Sent),
            "cancelled" => Some(ConnectStatus::Cancelled),
            _           => None
        }
    }

    pub fn as_str(&self) -> &str {
        match *self {
            ConnectStatus::Accepted  => "accepted",
            ConnectStatus::Blocked   => "blocked",
            ConnectStatus::Pending   => "pending",
            ConnectStatus::Ignored   => "ignored",
            ConnectStatus::Sent      => "sent",
            ConnectStatus::Cancelled => "cancelled"
        }
    }

    pub fn from_u8(b: u8) -> Option<ConnectStatus> {
        match b {
            0 => Some(ConnectStatus::Accepted),
            1 => Some(ConnectStatus::Blocked),
            2 => Some(ConnectStatus::Pending),
            3 => Some(ConnectStatus::Ignored),
            4 => Some(ConnectStatus::Sent),
            5 => Some(ConnectStatus::Cancelled),
            _ => None
        }
    }
}

impl Into<u8> for ConnectStatus {
    fn into(self) -> u8 {
        match self {
            ConnectStatus::Accepted  => 0,
            ConnectStatus::Blocked   => 1,
            ConnectStatus::Pending   => 2,
            ConnectStatus::Ignored   => 3,
            ConnectStatus::Sent      => 4,
            ConnectStatus::Cancelled => 5
        }
    }
}

#[derive(Clone, Debug)]
pub struct Connection<'a> {
    pub status:      ConnectStatus,
    pub conv:        ConvId,
    pub from:        UserId,
    pub to:          UserId,
    pub message:     Option<Cow<'a, str>>,
    pub last_update: DateTime<UTC>
}

impl<'a> Connection<'a> {
    pub fn new(s: ConnectStatus, c: ConvId, from: UserId, to: UserId, tm: DateTime<UTC>) -> Connection<'a> {
        Connection {
            status:      s,
            conv:        c,
            from:        from,
            to:          to,
            message:     None,
            last_update: tm
        }
    }

    pub fn set_message<M: Into<Cow<'a, str>>>(&mut self, m: M) {
        self.message = Some(m.into())
    }

    pub fn from_json(j: &Json) -> DecodeResult<Connection<'a>> {
        let r = Ref::new(j);
        let s = r.get("status").string().and_then(ConnectStatus::from_str);
        let f = r.get("from").string().and_then(UserId::from_str);
        let t = r.get("to").string().and_then(UserId::from_str);
        let d = r.get("last_update").value().map(util::datetime_from_json);
        let c = r.get("conversation").string().and_then(ConvId::from_str);
        let m = r.get("message").string().map(|s| Cow::Owned(String::from(s)));
        Ok(Connection {
            status:      from_some!(s, DecodeError::Expected("status")),
            conv:        from_some!(c, DecodeError::Expected("conversation")),
            from:        from_some!(f, DecodeError::Expected("from")),
            to:          from_some!(t, DecodeError::Expected("to")),
            message:     m,
            last_update: from_some_ok!(d, DecodeError::Expected("last_update"))
        })
    }
}

impl<'a> FromJson for Connection<'a> {
    fn decode<I: Iterator<Item=char>>(d: &mut Decoder<I>) -> DecodeResult<Self> {
        let mut b = [0; 24];
        let mut u = Utf8Buffer::new(&mut b);
        object! {
            let decoder = d;
            let buffer  = &mut u;
            Connection {
                status: req. "status" =>
                    ConnectStatus::from_str(&d.string()?)
                           .ok_or(DecodeError::Message("invalid connection status")),
                from:        req. "from"         => d.from_json(),
                to:          req. "to"           => d.from_json(),
                last_update: req. "last_update"  => util::parse_datetime(d),
                conv:        req. "conversation" => d.from_json(),
                message:     opt. "message"      => d.optional(|d| d.string().map(Cow::Owned))
            }
        }
    }
}

// User identity ////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub struct Identity<'a> {
    pub id:    UserId,
    pub email: Option<Email<'a>>,
    pub phone: Option<Phone<'a>>
}

impl<'a> Identity<'a> {
    pub fn from_json(j: &Json) -> DecodeResult<Identity<'a>> {
        let r = Ref::new(j);
        let i = r.get("id").string().and_then(UserId::from_str);
        Ok(Identity {
            id:    from_some!(i, DecodeError::Expected("id")),
            email: r.get("email").string().map(|s| Email::new(String::from(s))),
            phone: r.get("phone").string().map(|s| Phone::new(String::from(s)))
        })
    }
}

pub mod register {

    use std::io::Write;
    use json::{ToJson, Encoder, EncodeResult};
    use types::*;

    #[derive(Clone, Debug)]
    /// User registration parameters.
    pub struct Params<'a> {
        pub name:  Name<'a>,
        pub email: Option<Email<'a>>,
        pub phone: Option<Phone<'a>>,
        pub pass:  Option<Password<'a>>
    }

    impl<'a> Params<'a> {
        pub fn email(x: Email<'a>, n: Name<'a>, p: Password<'a>) -> Params<'a> {
            Params {
                name:  n,
                email: Some(x),
                phone: None,
                pass:  Some(p)
            }
        }

        pub fn phone(x: Phone<'a>, n: Name<'a>, p: Password<'a>) -> Params<'a> {
            Params {
                name:  n,
                email: None,
                phone: Some(x),
                pass:  Some(p)
            }
        }
    }

    impl<'a> ToJson for Params<'a> {
        fn encode<W: Write>(&self, e: &mut Encoder<W>) -> EncodeResult<()> {
            e.object()?;
                e.key("name")?; e.to_json(&self.name)?;
                if let Some(ref x) = self.email {
                    e.key("email")?; e.to_json(x)?
                }
                if let Some(ref x) = self.phone {
                    e.key("phone")?; e.to_json(x)?
                }
                if let Some(ref x) = self.pass {
                    e.key("password")?; e.to_json(x)?
                }
            e.end()
        }
    }

    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            InUse {
                display("account in use")
            }
            Blacklisted {
                display("blacklisted e-mail or phone")
            }
            Other(e: ApiError<'static>) {
                display("api error: {}", e)
                cause(e)
            }
        }
    }

    impl From<ApiError<'static>> for Error {
        fn from(e: ApiError<'static>) -> Error {
            match e.label.as_ref() {
                "blacklisted-phone" | "blacklisted-email" => Error::Blacklisted,
                "key-exists" => Error::InUse,
                _            => Error::Other(e)
            }
        }
    }
}

pub mod login {

    use std::io::Write;
    use json::{ToJson, Encoder, EncodeResult};
    use types::*;

    #[derive(Clone, Debug)]
    /// Login parameters.
    pub struct Params<'a> {
        pub email:  Option<Email<'a>>,
        pub handle: Option<Handle<'a>>,
        pub phone:  Option<Phone<'a>>,
        pub pass:   Password<'a>,
        pub label:  Label<'a>
    }

    impl<'a> Params<'a> {
        pub fn handle(x: Handle<'a>, p: Password<'a>, l: Label<'a>) -> Params<'a> {
            Params {
                email:  None,
                handle: Some(x),
                phone:  None,
                pass:   p,
                label:  l
            }
        }

        pub fn email(x: Email<'a>, p: Password<'a>, l: Label<'a>) -> Params<'a> {
            Params {
                email:  Some(x),
                handle: None,
                phone:  None,
                pass:   p,
                label:  l
            }
        }

        pub fn phone(x: Phone<'a>, p: Password<'a>, l: Label<'a>) -> Params<'a> {
            Params {
                email:  None,
                handle: None,
                phone:  Some(x),
                pass:   p,
                label:  l
            }
        }
    }

    impl<'a> ToJson for Params<'a> {
        fn encode<W: Write>(&self, e: &mut Encoder<W>) -> EncodeResult<()> {
            e.object()?;
                e.key("handle")?;   e.to_json(&self.handle)?;
                e.key("email")?;    e.to_json(&self.email)?;
                e.key("phone")?;    e.to_json(&self.phone)?;
                e.key("password")?; e.to_json(&self.pass)?;
                e.key("label")?;    e.to_json(&self.label)?;
            e.end()
        }
    }

    quick_error! {
        #[derive(Debug)]
        /// Login error
        pub enum Error {
            Invalid {
                display("invalid login credentials")
            }
            TooFrequent {
                display("logins happen to frequently")
            }
            MissingCookie {
                display("cookie is missing")
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
                403 => Error::Invalid,
                429 => Error::TooFrequent,
                _   => Error::Other(e)
            }
        }
    }
}

pub mod connect {

    use std::borrow::{Borrow, Cow};
    use std::io::Write;
    use json::{ToJson, Encoder, EncodeResult};
    use types::*;

    #[derive(Clone, Debug)]
    /// User connection request.
    pub struct Params<'a> {
        pub user:    UserId,
        pub name:    Name<'a>,
        pub message: Cow<'a, str>
    }

    impl<'a> Params<'a> {
        pub fn new<M: Into<Cow<'a, str>>>(u: UserId, n: Name<'a>, m: M) -> Option<Params<'a>> {
            let name_len = n.as_str().len();
            let message  = m.into();
            let mesg_len = message.len();
            if name_len < 1 || name_len > 256 || mesg_len < 1 || mesg_len > 256 {
                return None
            }
            Some(Params {
                user:    u,
                name:    n,
                message: message
            })
        }
    }

    impl<'a> ToJson for Params<'a> {
        fn encode<W: Write>(&self, e: &mut Encoder<W>) -> EncodeResult<()> {
            e.object()?;
                e.key("user")?;    e.to_json(&self.user)?;
                e.key("name")?;    e.to_json(&self.name)?;
                e.key("message")?; e.string(self.message.borrow())?;
            e.end()
        }
    }

    quick_error! {
        #[derive(Debug)]
        /// Connection request error.
        pub enum Error {
            InvalidUser {
                display("invalid user")
            }
            UnverifiedUser {
                display("user has no verified identity (e.g. e-mail or phone)")
            }
            TooManyConnections {
                display("user has too many connections")
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
                (400, "invalid-user")     => Error::InvalidUser,
                (403, "no-identity")      => Error::UnverifiedUser,
                (403, "connection-limit") => Error::TooManyConnections,
                _                         => Error::Other(e)
            }
        }
    }

    pub mod update {
        use std::io::Write;
        use json::{ToJson, Encoder, EncodeResult};
        use super::super::ConnectStatus;
        use types::*;

        #[derive(Clone, Debug)]
        /// User connection update request.
        pub struct Params {
            pub user:   UserId,
            pub status: ConnectStatus
        }

        impl Params {
            pub fn new(u: UserId, s: ConnectStatus) -> Params {
                Params {
                    user:   u,
                    status: s
                }
            }
        }

        impl ToJson for Params {
            fn encode<W: Write>(&self, e: &mut Encoder<W>) -> EncodeResult<()> {
                e.object()?;
                    e.key("status")?; e.string(self.status.as_str())?;
                e.end()
            }
        }

        quick_error! {
            #[derive(Debug)]
            /// Connection update error.
            pub enum Error {
                InvalidUser {
                    display("invalid user")
                }
                NoConnection {
                    display("not connection exists to user")
                }
                InvalidTransition {
                    display("invalid connection status transition")
                }
                TooManyConnections {
                    display("too many connections in state sent or accepted")
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
                    (400, "invalid-user")     => Error::InvalidUser,
                    (403, "no-connection")    => Error::NoConnection,
                    (403, "bad-conn-update")  => Error::InvalidTransition,
                    (403, "connection-limit") => Error::TooManyConnections,
                    _                         => Error::Other(e)
                }
            }
        }
    }

    pub mod get {

        use json::{FromJson, Decoder, DecodeResult, Utf8Buffer};
        use super::super::Connection;
        use types::*;

        pub struct Connections<'a>(pub Page<Vec<Connection<'a>>>);

        impl<'a> FromJson for Connections<'a> {
            fn decode<I: Iterator<Item=char>>(d: &mut Decoder<I>) -> DecodeResult<Self> {
                let mut b = [0; 24];
                let mut u = Utf8Buffer::new(&mut b);
                let data = extract! {
                    let decoder = d;
                    let buffer  = &mut u;
                    elements: req. Vec<Connection<'static>> = "connections" => array!(d, d.from_json()),
                    has_more: req. bool                     = "has_more"    => d.from_json()
                }?;
                Ok(Connections(Page::new(data.elements, data.has_more)))
            }
        }

    }
}
