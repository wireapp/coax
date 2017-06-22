use std::borrow::Cow;

use coax_api_proto::messages::GenericMessage;
use chrono::{DateTime, Utc};
use client::Client;
use conv::{Conversation, Message, MemberUpdate};
use json::ast::{Json, Ref};
use json::{FromJson, Decoder, DecodeError, DecodeResult, Utf8Buffer};
use types::{UserId, ClientId, ConvId, Name, NotifId};
use user::{Connection, Identity, User, UserUpdate};
use util;

// Notification type ////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct Notification<'a> {
    pub id:     NotifId,
    pub events: Vec<DecodeResult<Event<'a>>>
}

impl<'a> FromJson for Notification<'a> {
    fn decode<I: Iterator<Item=char>>(d: &mut Decoder<I>) -> DecodeResult<Self> {
        let mut b = [0; 16];
        let mut u = Utf8Buffer::new(&mut b);
        object! {
            let decoder = d;
            let buffer  = &mut u;
            Notification {
                id:     req. "id"      => d.from_json(),
                events: req. "payload" => {
                    let mut v = Vec::new();
                    for e in d.array_iter()? {
                        v.push(e)
                    }
                    Ok(v)
                }
            }
        }
    }
}

// Event type ///////////////////////////////////////////////////////////////

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EventType {
    ConvCreate,
    ConvMemberJoin,
    ConvMemberLeave,
    ConvMemberUpdate,
    ConvMessageAdd,
    ConvRename,
    UserActivate,
    UserClientAdd,
    UserClientRemove,
    UserConnection,
    UserContactJoin,
    UserDelete,
    UserIdentityRemove,
    UserPropertiesClear,
    UserPropertiesDelete,
    UserPropertiesSet,
    UserUpdate
}

impl EventType {
    pub fn from_str(s: &str) -> Option<EventType> {
        match s {
            "conversation.create"          => Some(EventType::ConvCreate),
            "conversation.member-join"     => Some(EventType::ConvMemberJoin),
            "conversation.member-leave"    => Some(EventType::ConvMemberLeave),
            "conversation.member-update"   => Some(EventType::ConvMemberUpdate),
            "conversation.otr-message-add" => Some(EventType::ConvMessageAdd),
            "conversation.rename"          => Some(EventType::ConvRename),
            "user.activate"                => Some(EventType::UserActivate),
            "user.client-add"              => Some(EventType::UserClientAdd),
            "user.client-remove"           => Some(EventType::UserClientRemove),
            "user.connection"              => Some(EventType::UserConnection),
            "user.contact-join"            => Some(EventType::UserContactJoin),
            "user.delete"                  => Some(EventType::UserDelete),
            "user.identity-remove"         => Some(EventType::UserIdentityRemove),
            "user.properties-clear"        => Some(EventType::UserPropertiesClear),
            "user.properties-delete"       => Some(EventType::UserPropertiesDelete),
            "user.properties-set"          => Some(EventType::UserPropertiesSet),
            "user.update"                  => Some(EventType::UserUpdate),
            _                              => None
        }
    }

    pub fn is_conv_event(&self) -> bool {
        use self::EventType::*;
        match *self {
            ConvMessageAdd
            | ConvCreate
            | ConvRename
            | ConvMemberJoin
            | ConvMemberLeave
            | ConvMemberUpdate => true,
            _                  => false
        }
    }

    pub fn is_user_event(&self) -> bool {
        use self::EventType::*;
        match *self {
            UserActivate
            | UserClientAdd
            | UserClientRemove
            | UserConnection
            | UserContactJoin
            | UserDelete
            | UserIdentityRemove
            | UserPropertiesClear
            | UserPropertiesDelete
            | UserPropertiesSet
            | UserUpdate => true,
            _            => false
        }
    }
}

// Event ////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub enum Event<'a> {
    Conv(EventType, ConvEvent<'a>),
    User(EventType, UserEvent<'a>),
    Unknown(Cow<'a, str>)
}

impl<'a> FromJson for Event<'a> {
    fn decode<I: Iterator<Item=char>>(d: &mut Decoder<I>) -> DecodeResult<Self> {
        let j = d.from_json()?;
        if let Some(t) = Ref::new(&j).get("type").string().map(String::from) {
            match EventType::from_str(t.as_str()) {
                Some(ref ty) if ty.is_conv_event() => {
                    let ev = ConvEvent::from_json(ty.clone(), &j)?;
                    Ok(Event::Conv(ty.clone(), ev))
                }
                Some(ref ty) if ty.is_user_event() => {
                    let ev = UserEvent::from_json(ty.clone(), j)?;
                    Ok(Event::User(ty.clone(), ev))
                }
                _ => Ok(Event::Unknown(Cow::Owned(t)))
            }
        } else {
            Err(DecodeError::Expected("type"))
        }
    }
}

// Conversation event ///////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub struct ConvEvent<'a> {
    pub id:   ConvId,
    pub from: UserId,
    pub time: DateTime<Utc>,
    pub data: ConvEventData<'a>
}

impl<'a> ConvEvent<'a> {
    pub fn from_json(ty: EventType, j: &Json) -> DecodeResult<ConvEvent<'a>> {
        let r = Ref::new(j);
        let i = r.get("conversation").string().and_then(ConvId::from_str);
        let u = r.get("from").string().and_then(UserId::from_str);
        let t = r.get("time").value().map(util::datetime_from_json);
        let d = r.get("data").value().map(|d| ConvEventData::from_json(ty, d));
        Ok(ConvEvent {
            id:   from_some!(i, DecodeError::Expected("conversation")),
            from: from_some!(u, DecodeError::Expected("from")),
            time: from_some_ok!(t, DecodeError::Expected("time")),
            data: from_some_ok!(d, DecodeError::Expected("data"))
        })
    }
}

#[derive(Clone, Debug)]
pub enum ConvEventData<'a> {
    Encrypted(Message<'a, Cow<'a, [u8]>>),
    Decrypted(Message<'a, GenericMessage>),
    Create(Conversation<'a>),
    Rename(Cow<'a, str>),
    Join(Cow<'a, [UserId]>),
    Leave(Cow<'a, [UserId]>),
    Update(MemberUpdate)
}

impl<'a> ConvEventData<'a> {
    pub fn from_json(ty: EventType, j: &Json) -> DecodeResult<ConvEventData<'a>> {
        match ty {
            EventType::ConvMessageAdd => {
                let m = Message::from_json(j)?;
                Ok(ConvEventData::Encrypted(m))
            }
            EventType::ConvCreate => {
                let x = Conversation::from_json(j)?;
                Ok(ConvEventData::Create(x))
            }
            EventType::ConvRename =>
                Ref::new(j).get("name").string()
                    .map(|s| Cow::Owned(String::from(s)))
                    .map(ConvEventData::Rename)
                    .ok_or(DecodeError::Expected("name")),
            EventType::ConvMemberJoin => {
                let ids = util::map_json_slice(Ref::new(j).get("user_ids"), |x| {
                    Ref::new(x).string()
                        .and_then(UserId::from_str)
                        .ok_or(DecodeError::Message("invalid user ID"))
                })?;
                Ok(ConvEventData::Join(Cow::Owned(ids)))
            }
            EventType::ConvMemberLeave => {
                let ids = util::map_json_slice(Ref::new(j).get("user_ids"), |x| {
                    Ref::new(x).string()
                        .and_then(UserId::from_str)
                        .ok_or(DecodeError::Message("invalid user ID"))
                })?;
                Ok(ConvEventData::Leave(Cow::Owned(ids)))
            }
            EventType::ConvMemberUpdate => {
                let x = MemberUpdate::from_json(j)?;
                Ok(ConvEventData::Update(x))
            }
            _ => Err(DecodeError::Expected("conversation event"))
        }
    }
}

// User event ///////////////////////////////////////////////////////////////


#[derive(Clone, Debug)]
pub enum UserEvent<'a> {
    Activate(User<'a>),
    Delete(UserId),
    Update(UserUpdate<'a>),
    RemoveIdent(Identity<'a>),
    RemoveClient(ClientId<'a>),
    AddClient(Client<'a>),
    Connect(Option<Name<'a>>, Connection<'a>),
    ClearProps,
    SetProp(Cow<'a, str>, Json),
    DeleteProp(Cow<'a, str>)
}

impl<'a> UserEvent<'a> {
    pub fn from_json(ty: EventType, j: Json) -> DecodeResult<UserEvent<'a>> {
        match ty {
            EventType::UserActivate => {
                let u = Ref::new(&j).get("user").value().map(User::from_json);
                Ok(UserEvent::Activate(from_some_ok!(u, DecodeError::Expected("user"))))
            }
            EventType::UserDelete => {
                let i = Ref::new(&j).get("id").string().and_then(UserId::from_str);
                Ok(UserEvent::Delete(from_some!(i, DecodeError::Expected("user id"))))
            }
            EventType::UserUpdate => {
                let u = Ref::new(&j).get("user").value().map(UserUpdate::from_json);
                Ok(UserEvent::Update(from_some_ok!(u, DecodeError::Expected("user update"))))
            }
            EventType::UserIdentityRemove => {
                let i = Ref::new(&j).get("user").value().map(Identity::from_json);
                Ok(UserEvent::RemoveIdent(from_some_ok!(i, DecodeError::Expected("user identity"))))
            }
            EventType::UserClientRemove => {
                let i =
                    Ref::new(&j)
                        .get("client")
                        .get("id")
                        .string()
                        .map(|s| ClientId::new(String::from(s)));
                Ok(UserEvent::RemoveClient(from_some!(i, DecodeError::Expected("client id"))))
            }
            EventType::UserClientAdd => {
                let c = Ref::new(&j).get("client").value().map(Client::from_json);
                Ok(UserEvent::AddClient(from_some_ok!(c, DecodeError::Expected("client"))))
            }
            EventType::UserPropertiesClear  => Ok(UserEvent::ClearProps),
            EventType::UserPropertiesDelete => {
                let k = Ref::new(&j).get("key").string().map(|s| Cow::Owned(String::from(s)));
                Ok(UserEvent::DeleteProp(from_some!(k, DecodeError::Expected("key"))))
            }
            EventType::UserPropertiesSet => {
                match j {
                    Json::Object(mut m) => {
                        if let Some(Json::String(k)) = m.remove("key") {
                            if let Some(v) = m.remove("value") {
                                return Ok(UserEvent::SetProp(Cow::Owned(k), v))
                            }
                        }
                        Err(DecodeError::Message("invalid user.properties-set json"))
                    }
                    _ => Err(DecodeError::Message("invalid user.properties-set json"))
                }
            }
            EventType::UserConnection => {
                let r = Ref::new(&j);
                let n = r.get("user").get("name").string().map(|s| Name::new(String::from(s)));
                let c = r.get("connection").value().map(Connection::from_json);
                Ok(UserEvent::Connect(n, from_some_ok!(c, DecodeError::Expected("connection"))))
            }
            _ => Err(DecodeError::Expected("user event type"))
        }
    }
}

pub mod get {

    use std::io::Read;
    use json::decoder::{Decoder, DecodeError, DecodeResult, ReadIter};
    use slog::Logger;
    use super::Notification;
    use types::{ClientId, NotifId};

    #[derive(Clone, Debug)]
    pub struct Params<'a> {
        pub client: ClientId<'a>,
        pub start:  Option<NotifId>,
        pub size:   Option<usize>
    }

    impl<'a> Params<'a> {
        pub fn new(c: ClientId<'a>) -> Params<'a> {
            Params {
                client: c,
                start: None,
                size:  None
            }
        }

        pub fn set_start(&mut self, s: NotifId) {
            self.start = Some(s)
        }

        pub fn set_size(&mut self, s: usize) {
            self.size = Some(s)
        }
    }

    pub struct Iter<I: Iterator<Item=char>> {
        decoder:  Decoder<I>,
        has_more: Option<bool>,
        logger:   Logger
    }

    impl<R: Read> Iter<ReadIter<R>> {
        pub fn from_read(g: &Logger, r: R) -> DecodeResult<Iter<ReadIter<R>>> {
            Iter::new(g, ReadIter::new(r))
        }
    }

    impl<I: Iterator<Item=char>> Iter<I> {
        pub fn new(g: &Logger, i: I) -> DecodeResult<Iter<I>> {
            let mut d = Decoder::default(i);
            let mut has_more = None;
            d.object()?;
            while d.has_more()? {
                match d.key()?.as_str() {
                    "has_more"      => { has_more = Some(d.bool()?) }
                    "notifications" => {
                        d.array()?;
                        let it = Iter {
                            decoder:  d,
                            has_more: has_more,
                            logger:   g.new(o!("context" => "Iter"))
                        };
                        return Ok(it)
                    }
                    _ => d.skip()?
                }
            }
            Err(DecodeError::Expected("notifications"))
        }

        pub fn has_more(&mut self) -> DecodeResult<bool> {
            while self.decoder.has_more()? {
                match self.decoder.key()?.as_str() {
                    "has_more" => self.has_more = Some(self.decoder.bool()?),
                    _          => self.decoder.skip()?
                }
            }
            if let Some(m) = self.has_more {
                Ok(m)
            } else {
                Err(DecodeError::Expected("has_more"))
            }
        }
    }

    impl<I: Iterator<Item=char>> Iterator for Iter<I> {
        type Item = DecodeResult<Notification<'static>>;

        fn next(&mut self) -> Option<DecodeResult<Notification<'static>>> {
            match self.decoder.has_more() {
                Ok(true)  => Some(self.decoder.from_json()),
                Ok(false) => None,
                Err(e)    => {
                    error!(self.logger, "Decoder::has_more failed"; "error" => ?e);
                    None
                }
            }
        }
    }

    impl<I: Iterator<Item=char>> Drop for Iter<I> {
        fn drop(&mut self) {
            for _ in self.decoder.iter_mut() {
                // drain iterator
            }
        }
    }
}
