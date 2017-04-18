use std::borrow::Cow;
use std::error::Error;
use std::fmt;

use coax_api_proto::messages::GenericMessage;
use cryptobox::store::Store;
use cryptobox::{CBox, CBoxSession, CBoxError};
use json::ast::{Json, Ref};
use json::{FromJson, Decoder, DecodeError, DecodeResult, Utf8Buffer};
use openssl::error::ErrorStack;
use openssl::hash::{hash2, MessageDigest};
use openssl::symm::{self, Cipher};
use protobuf::{self, ProtobufError};
use rustc_serialize::base64::{FromBase64, FromBase64Error};
use types::{UserId, ClientId, ConvId, Name, ServiceRef};
use util;

// Conversation type ////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ConvType {
    Group,
    SelfConv,
    OneToOne,
    Connect
}

impl ConvType {
    pub fn from_u8(b: u8) -> Option<ConvType> {
        match b {
            0 => Some(ConvType::Group),
            1 => Some(ConvType::SelfConv),
            2 => Some(ConvType::OneToOne),
            3 => Some(ConvType::Connect),
            _ => None
        }
    }
}

impl Into<u8> for ConvType {
    fn into(self) -> u8 {
        match self {
            ConvType::Group    => 0,
            ConvType::SelfConv => 1,
            ConvType::OneToOne => 2,
            ConvType::Connect  => 3
        }
    }
}

// Conversation /////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub struct Conversation<'a> {
    pub id:      ConvId,
    pub typ:     ConvType,
    pub creator: UserId,
    pub name:    Option<Name<'a>>,
    pub members: Members
}

impl<'a> Conversation<'a> {
    pub fn new(id: ConvId, u: UserId, m: Members) -> Conversation<'a> {
        Conversation {
            id:      id,
            typ:     ConvType::Group,
            creator: u,
            name:    None,
            members: m
        }
    }

    pub fn set_name(&mut self, n: Name<'a>) {
        self.name = Some(n)
    }

    pub fn from_json(j: &Json) -> DecodeResult<Conversation<'a>> {
        let r = Ref::new(j);
        let i = r.get("id").string().and_then(ConvId::from_str);
        let t = r.get("type").number().and_then(|t| ConvType::from_u8(t as u8));
        let c = r.get("creator").string().and_then(UserId::from_str);
        let n = r.get("name").string().map(|s| Name::new(s).acquire());
        let m = r.get("members").value().map(Members::from_json);
        Ok(Conversation {
            id:      from_some!(i, DecodeError::Expected("id")),
            typ:     from_some!(t, DecodeError::Expected("type")),
            creator: from_some!(c, DecodeError::Expected("creator")),
            name:    n,
            members: from_some_ok!(m, DecodeError::Expected("members"))
        })
    }
}

impl<'a> FromJson for Conversation<'a> {
    fn decode<I: Iterator<Item=char>>(d: &mut Decoder<I>) -> DecodeResult<Self> {
        let mut b = [0; 16];
        let mut u = Utf8Buffer::new(&mut b);
        object! {
            let decoder = d;
            let buffer  = &mut u;
            Conversation {
                id:  req. "id"   => d.from_json(),
                typ: req. "type" =>
                    ConvType::from_u8(d.u8()?)
                        .ok_or(DecodeError::Message("invalid conversation type")),
                creator: req. "creator" => d.from_json(),
                name:    opt. "name"    => d.from_json(),
                members: req. "members" => d.from_json()
            }
        }
    }
}

// Converation members //////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub struct Members {
    pub me:     SelfMember,
    pub others: Vec<Member>
}

impl Members {
    pub fn new(m: SelfMember) -> Members {
        Members {
            me: m,
            others: Vec::new()
        }
    }

    pub fn add_member(&mut self, m: Member) {
        self.others.push(m)
    }

    pub fn from_json(j: &Json) -> DecodeResult<Members> {
        let r = Ref::new(j);
        let s = r.get("self").value().map(SelfMember::from_json);
        let o = util::map_json_slice(r.get("others"), Member::from_json)?;
        Ok(Members {
            me:     from_some_ok!(s, DecodeError::Expected("self")),
            others: o
        })
    }
}

impl FromJson for Members {
    fn decode<I: Iterator<Item=char>>(d: &mut Decoder<I>) -> DecodeResult<Self> {
        let mut b = [0; 16];
        let mut u = Utf8Buffer::new(&mut b);
        object! {
            let decoder = d;
            let buffer  = &mut u;
            Members {
                me:     req. "self"   => d.from_json(),
                others: req. "others" => d.from_json()
            }
        }
    }
}

// Self member //////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub struct SelfMember {
    pub id:      UserId,
    pub muted:   Option<bool>,
    pub current: bool
}

impl SelfMember {
    pub fn new(id: UserId) -> SelfMember {
        SelfMember {
            id:      id,
            muted:   None,
            current: true
        }
    }

    pub fn mute(&mut self) {
        self.muted = Some(true)
    }

    pub fn from_json(j: &Json) -> DecodeResult<SelfMember> {
        let r = Ref::new(j);
        let i = r.get("id").string().and_then(UserId::from_str);
        let c = r.get("status").number().map(|s| s as u8 == 0).unwrap_or(false);
        let m = r.get("otr_muted").bool();
        Ok(SelfMember {
            id:      from_some!(i, DecodeError::Expected("id")),
            muted:   m,
            current: c
        })
    }
}

impl FromJson for SelfMember {
    fn decode<I: Iterator<Item=char>>(d: &mut Decoder<I>) -> DecodeResult<Self> {
        let mut b = [0; 16];
        let mut u = Utf8Buffer::new(&mut b);
        object! {
            let decoder = d;
            let buffer  = &mut u;
            SelfMember {
                id:      req. "id"        => d.from_json(),
                muted:   opt. "otr_muted" => d.from_json(),
                current: req. "status"    => d.u8().map(|s| s == 0)
            }
        }
    }
}

// Member ///////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub struct Member {
    pub id:      UserId,
    pub srv:     Option<ServiceRef>,
    pub current: bool
}

impl Member {
    pub fn new(id: UserId, s: Option<ServiceRef>) -> Member {
        Member {
            id:     id,
            srv:     s,
            current: true
        }
    }

    pub fn from_json(j: &Json) -> DecodeResult<Member> {
        let r = Ref::new(j);
        let i = r.get("id").string().and_then(UserId::from_str);
        let s = r.get("service").value().map(ServiceRef::from_json);
        let c = r.get("status").number().map(|s| s as u8 == 0).unwrap_or(false);
        Ok(Member {
            id:      from_some!(i, DecodeError::Expected("id")),
            srv:     from_some_ok!(s),
            current: c
        })
    }
}

impl FromJson for Member {
    fn decode<I: Iterator<Item=char>>(d: &mut Decoder<I>) -> DecodeResult<Self> {
        let mut b = [0; 16];
        let mut u = Utf8Buffer::new(&mut b);
        object! {
            let decoder = d;
            let buffer  = &mut u;
            Member {
                id:      req. "id"      => d.from_json(),
                srv:     opt. "service" => d.from_json(),
                current: req. "status"  => d.u8().map(|s| s == 0)
            }
        }
    }
}

// Member update ////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub struct MemberUpdate {
    pub muted: Option<bool>
}

impl MemberUpdate {
    pub fn from_json(j: &Json) -> DecodeResult<MemberUpdate> {
        Ok(MemberUpdate {
            muted: Ref::new(j).get("otr_muted").bool()
        })
    }
}

// Message //////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub struct Message<'a, T> {
    pub sender:    ClientId<'a>,
    pub recipient: ClientId<'a>,
    pub text:      T,
    pub data:      Option<Cow<'a, str>>
}

impl<'a> Message<'a, Cow<'a, [u8]>> {
    pub fn from_json(j: &Json) -> DecodeResult<Message<'a, Cow<'a, [u8]>>> {
        let r = Ref::new(j);
        let f = r.get("sender").string().map(|s| ClientId::new(String::from(s)));
        let t = r.get("recipient").string().map(|s| ClientId::new(String::from(s)));
        let d = r.get("data").string().map(|s| Cow::Owned(String::from(s)));
        let m = r.get("text").string().map(|s| {
            match s.from_base64() {
                Ok(b)  => Ok(Cow::Owned(b)),
                Err(e) => Err(DecodeError::Other(Box::new(e)))
            }
        });
        Ok(Message {
            sender:    from_some!(f, DecodeError::Expected("sender")),
            recipient: from_some!(t, DecodeError::Expected("recipient")),
            text:      from_some_ok!(m, DecodeError::Expected("text")),
            data:      d
        })
    }

    pub fn decrypt<S: Store>(&self, from: &UserId, b: &CBox<S>) ->
        Result<(CBoxSession<S>, Message<'a, GenericMessage>), DecryptError<S>>
    {
        let sid = super::new_session_id(from, &self.sender);
        let (session, bytes) =
            if let Some(s) = b.session(&sid)? {
                let data = s.decrypt(&self.text)?;
                (s.clone(), data)
            } else {
                b.session_from_message(&sid, &self.text)?
            };
        let proto = {
            let msg: GenericMessage = protobuf::parse_from_bytes(&bytes)?;
            if let Some(ref d) = self.data {
                if !msg.has_external() {
                    return Err(DecryptError::MessageType("expected `External`"))
                }
                let data = d.from_base64()?;
                if data.len() < 16 {
                    return Err(DecryptError::Integrity("too short"))
                }
                let ext  = msg.get_external();
                let hsh1 = ext.get_sha256();
                let hsh2 = hash2(MessageDigest::sha256(), &data)?;
                if hsh1 != hsh2.as_ref() {
                    return Err(DecryptError::Integrity("hash mismatch"))
                }
                let key = ext.get_otr_key();
                let iv  = &data[0 .. 16];
                let txt = symm::decrypt(Cipher::aes_256_cbc(), key, Some(iv), &data[16 ..])?;
                protobuf::parse_from_bytes(&txt)?
            } else {
                msg
            }
        };
        let msg = Message {
            sender:    self.sender.clone(),
            recipient: self.recipient.clone(),
            text:      proto,
            data:      None
        };
        Ok((session, msg))
    }
}

// DecryptError /////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum DecryptError<T: Store> {
    CBox(CBoxError<T>),
    Pbuf(ProtobufError),
    Integrity(&'static str),
    MessageType(&'static str),
    Other(Box<Error + Send + Sync>)
}

impl<T: Store> From<CBoxError<T>> for DecryptError<T> {
    fn from(e: CBoxError<T>) -> DecryptError<T> {
        DecryptError::CBox(e)
    }
}

impl<T: Store> From<ProtobufError> for DecryptError<T> {
    fn from(e: ProtobufError) -> DecryptError<T> {
        DecryptError::Pbuf(e)
    }
}

impl<T: Store> From<FromBase64Error> for DecryptError<T> {
    fn from(e: FromBase64Error) -> DecryptError<T> {
        DecryptError::Other(Box::new(e))
    }
}

impl<T: Store> From<ErrorStack> for DecryptError<T> {
    fn from(e: ErrorStack) -> DecryptError<T> {
        DecryptError::Other(Box::new(e))
    }
}

impl<S: Store> fmt::Display for DecryptError<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            DecryptError::CBox(ref e)        => write!(f, "cbox error: {}", e),
            DecryptError::Pbuf(ref e)        => write!(f, "protobuf error: {}", e),
            DecryptError::Integrity(ref e)   => write!(f, "integrity error: {}", e),
            DecryptError::MessageType(ref e) => write!(f, "message type: {}", e),
            DecryptError::Other(ref e)       => write!(f, "other error: {}", e)
        }
    }
}

impl<S: Store + fmt::Debug> Error for DecryptError<S> {
    fn description(&self) -> &str {
        "DecryptError"
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            DecryptError::CBox(ref e)    => Some(e),
            DecryptError::Pbuf(ref e)    => Some(e),
            DecryptError::Other(ref e)   => Some(&**e),
            DecryptError::Integrity(_)   => None,
            DecryptError::MessageType(_) => None
        }
    }
}


pub mod create {

    use std::borrow::Cow;
    use std::io::Write;
    use json::{Json, ToJson, Encoder, EncodeResult};
    use types::*;

    #[derive(Clone, Debug)]
    /// Conversation creation parameters.
    pub struct Params<'a> {
        pub name:  Option<Name<'a>>,
        pub users: Cow<'a, [UserId]>,
        pub extra: Option<Json>
    }

    impl<'a> Params<'a> {
        pub fn new(u: &'a [UserId]) -> Params<'a> {
            Params {
                name:  None,
                users: Cow::Borrowed(u),
                extra: None
            }
        }

        pub fn set_name(&mut self, n: Name<'a>) {
            self.name = Some(n)
        }
    }

    impl<'a> ToJson for Params<'a> {
        fn encode<W: Write>(&self, e: &mut Encoder<W>) -> EncodeResult<()> {
            e.object()?;
                e.key("name")?;  e.to_json(&self.name)?;
                e.key("users")?; e.to_json(&*self.users)?;
                if let Some(Json::Object(ref m)) = self.extra {
                    for (k, v) in m {
                        e.key(k.as_str())?; e.to_json(v)?
                    }
                }
            e.end()
        }
    }
}

pub mod join {

    use types::*;

    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            NotFound {
                display("conversation not found")
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
                (404, "no-conversation")  => Error::NotFound,
                _                         => Error::Other(e)
            }
        }
    }
}

pub mod get {

    use json::{FromJson, Decoder, DecodeResult, Utf8Buffer};
    use types::*;

    pub struct ConvIds(pub Page<Vec<ConvId>>);

    impl FromJson for ConvIds {
        fn decode<I: Iterator<Item=char>>(d: &mut Decoder<I>) -> DecodeResult<Self> {
            let mut b = [0; 16];
            let mut u = Utf8Buffer::new(&mut b);
            let data = extract! {
                let decoder = d;
                let buffer  = &mut u;
                ids:      req. Vec<ConvId> = "conversations" => d.from_json(),
                has_more: req. bool        = "has_more"      => d.from_json()
            }?;
            Ok(ConvIds(Page::new(data.ids, data.has_more)))
        }
    }
}
