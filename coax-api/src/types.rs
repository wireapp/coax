use std::borrow::{Borrow, Cow};
use std::error::Error;
use std::fmt;
use std::io::Write;
use std::marker::PhantomData;

use json::ast::{Json, Ref};
use json::{ToJson, Encoder, EncodeResult};
use json::{FromJson, Decoder, DecodeError, DecodeResult, Utf8Buffer};
use uuid::Uuid;

// IDs //////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, PartialEq, Eq, Hash)] pub struct N;
#[derive(Clone, Debug, PartialEq, Eq, Hash)] pub struct U;
#[derive(Clone, Debug, PartialEq, Eq, Hash)] pub struct C;
#[derive(Clone, Debug, PartialEq, Eq, Hash)] pub struct S;
#[derive(Clone, Debug, PartialEq, Eq, Hash)] pub struct P;

pub type UserId  = Id<U>;
pub type ConvId  = Id<C>;
pub type ServId  = Id<S>;
pub type ProvId  = Id<P>;
pub type NotifId = Id<N>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Id<T>{
    id: Uuid,
    _t: PhantomData<T>
}

impl<T> Id<T> {
    pub fn new(u: Uuid) -> Id<T> {
        Id {
            id: u,
            _t: PhantomData
        }
    }

    pub fn rand() -> Id<T> {
        Id::new(Uuid::new_v4())
    }

    pub fn from_str(s: &str) -> Option<Id<T>> {
        Uuid::parse_str(s).ok().map(Id::new)
    }

    pub fn to_string(&self) -> String {
        self.id.hyphenated().to_string()
    }

    pub fn as_bytes(&self) -> &[u8; 16] {
        self.id.as_bytes()
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.as_bytes()[..]
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.id
    }

    pub fn from_bytes(b: &[u8]) -> Option<Id<T>> {
        Uuid::from_bytes(b).map(Id::new).ok()
    }
}

impl<T> fmt::Display for Id<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.id.fmt(f)
    }
}

impl<T> ToJson for Id<T> {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> EncodeResult<()> {
        e.string(self.to_string())
    }
}

impl<T> FromJson for Id<T> {
    fn decode<I: Iterator<Item=char>>(d: &mut Decoder<I>) -> DecodeResult<Self> {
        let s = d.string()?;
        match Id::from_str(s.as_str()) {
            Some(i) => Ok(i),
            None    => Err(DecodeError::Message("failed to parse UUID"))
        }
    }
}

// API error ////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct ApiError<'a> {
    pub code:    isize,
    pub message: Cow<'a, str>,
    pub label:   Cow<'a, str>
}

impl<'a> fmt::Display for ApiError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "code: {}, label: {}, message: {}", self.code, self.label, self.message)
    }
}

impl<'a> Error for ApiError<'a> {
    fn description(&self) -> &str {
        "api error"
    }
}

impl<'a> ApiError<'a> {
    pub fn new<M, L>(code: isize, msg: M, lbl: L) -> ApiError<'a>
        where M: Into<Cow<'a, str>>,
              L: Into<Cow<'a, str>>
    {
        ApiError {
            code:    code,
            message: msg.into(),
            label:   lbl.into()
        }
    }
}

impl<'a> FromJson for ApiError<'a> {
    fn decode<I: Iterator<Item=char>>(d: &mut Decoder<I>) -> DecodeResult<Self> {
        let mut b = [0; 16];
        let mut u = Utf8Buffer::new(&mut b);
        object! {
            let decoder = d;
            let buffer  = &mut u;
            ApiError {
                code:    req. "code"    => d.from_json(),
                message: req. "message" => d.from_json().map(Cow::Owned),
                label:   req. "label"   => d.from_json().map(Cow::Owned)
            }
        }
    }
}

// ServiceRef ///////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub struct ServiceRef {
    pub serv: ServId,
    pub prov: ProvId
}

impl ServiceRef {
    pub fn new(s: ServId, p: ProvId) -> ServiceRef {
        ServiceRef {
            serv: s,
            prov: p
        }
    }

    pub fn from_json(j: &Json) -> DecodeResult<ServiceRef> {
        let r = Ref::new(j);
        let i = r.get("id").string().and_then(ServId::from_str);
        let p = r.get("provider").string().and_then(ProvId::from_str);
        Ok(ServiceRef {
            serv: from_some!(i, DecodeError::Expected("id")),
            prov: from_some!(p, DecodeError::Expected("provider"))
        })
    }
}

impl FromJson for ServiceRef {
    fn decode<I: Iterator<Item=char>>(d: &mut Decoder<I>) -> DecodeResult<Self> {
        let mut b = [0; 16];
        let mut u = Utf8Buffer::new(&mut b);
        object! {
            let decoder = d;
            let buffer  = &mut u;
            ServiceRef {
                serv: req. "id"       => d.from_json(),
                prov: req. "provider" => d.from_json()
            }
        }
    }
}

impl ToJson for ServiceRef {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> EncodeResult<()> {
        e.object()?;
            e.key("id")?;       e.to_json(&self.serv)?;
            e.key("provider")?; e.to_json(&self.prov)?;
        e.end()
    }
}

// Small types //////////////////////////////////////////////////////////////

json_str_type!(ClientId);
json_str_type!(Email);
json_str_type!(Phone);
json_str_type!(Name);
json_str_type!(Password);
json_str_type!(Label);
json_str_type!(Handle);

pub struct Page<T> {
    pub value:    T,
    pub has_more: bool
}

impl<T> Page<T> {
    pub fn new(v: T, m: bool) -> Page<T> {
        Page {
            value: v,
            has_more: m
        }
    }
}

