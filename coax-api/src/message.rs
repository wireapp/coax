use std::borrow::Cow;
use std::collections::HashMap;
use std::io::Write;

use json::{ToJson, Encoder, EncodeResult};
use json::{FromJson, Decoder, DecodeError, DecodeResult, Utf8Buffer};
use types::{UserId, ClientId};

pub use coax_api_proto::builder::Builder;

#[derive(Clone, Debug)]
pub enum Priority {
    Low,
    High
}

#[derive(Clone, Debug)]
pub struct NewMessage<'a> {
    pub native_push: bool,
    pub native_prio: Priority,
    pub transient:   bool,
    pub sender:      ClientId<'a>,
    pub data:        Option<Cow<'a, str>>,
    pub recipients:  HashMap<UserId, HashMap<ClientId<'a>, String>>
}

impl<'a> NewMessage<'a> {
    pub fn new(from: ClientId<'a>) -> NewMessage<'a> {
        NewMessage {
            native_push: true,
            native_prio: Priority::High,
            transient:   false,
            sender:      from,
            data:        None,
            recipients:  HashMap::new()
        }
    }

    pub fn set_data(&mut self, d: &'a str) {
        self.data = Some(Cow::Borrowed(d))
    }
}

impl<'a> FromJson for NewMessage<'a> {
    fn decode<I: Iterator<Item=char>>(d: &mut Decoder<I>) -> DecodeResult<Self> {
        let mut b = [0; 32];
        let mut u = Utf8Buffer::new(&mut b);
        object! {
            let decoder = d;
            let buffer  = &mut u;
            NewMessage {
                native_push: req. "native_push"     => d.bool(),
                native_prio: req. "native_priority" => match d.string()?.as_ref() {
                    "low"  => Ok(Priority::Low),
                    "high" => Ok(Priority::High),
                    _      => return Err(DecodeError::Message("unknown priority"))
                },
                transient:  req. "transient"  => d.bool(),
                sender:     req. "sender"     => d.from_json(),
                data:       opt. "data"       => d.optional(|d| d.string().map(Cow::Owned)),
                recipients: req. "recipients" => {
                    d.object()?;
                    let mut users = HashMap::new();
                    while d.has_more()? {
                        let uid = UserId::from_str(&d.key()?).ok_or(DecodeError::Message("invalid user id"))?;
                        d.object()?;
                        let mut clients = HashMap::new();
                        while d.has_more()? {
                            let cid = d.key().map(ClientId::new)?;
                            let msg = d.string()?;
                            clients.insert(cid, msg);
                        }
                        users.insert(uid, clients);
                    }
                    Ok(users)
                }
            }
        }
    }
}

impl<'a> ToJson for NewMessage<'a> {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> EncodeResult<()> {
        e.object()?;
            e.key("native_push")?; e.bool(self.native_push)?;
            e.key("native_priority")?;
            match self.native_prio {
                Priority::Low  => e.string("low")?,
                Priority::High => e.string("high")?
            }
            e.key("transient")?; e.bool(self.transient)?;
            e.key("sender")?;    e.to_json(&self.sender)?;
            if let Some(ref d) = self.data {
                e.key("data")?;
                d.encode(e)?;
            }
            e.key("recipients")?;
            e.object()?;
                for (usr, map) in &self.recipients {
                    e.key(usr.to_string())?;
                    e.object()?;
                    for (dev, txt) in map {
                        e.key(dev.as_str())?; txt.encode(e)?;
                    }
                    e.end()?;
                }
            e.end()?;
        e.end()
    }
}


pub mod send {

    use std;
    use std::collections::HashMap;
    use std::collections::hash_map::Entry;
    use std::fmt;

    use client::ClientMismatch;
    use cryptobox::store::Store;
    use cryptobox::{CBoxError, CBoxSession};
    use protobuf::ProtobufError;
    use base64;
    use super::NewMessage;
    use types::{UserId, ClientId, ConvId, ApiError};

    #[derive(Clone, Debug)]
    /// Send message parameters.
    pub struct Params {
        pub conv:    ConvId,
        pub message: NewMessage<'static>
    }

    impl Params {
        pub fn new(c: ConvId, from: ClientId<'static>) -> Params {
            Params {
                conv:    c,
                message: NewMessage::new(from)
            }
        }

        pub fn add<S: Store>(&mut self,
                             s: &CBoxSession<S>,
                             u: UserId,
                             c: ClientId<'static>,
                             m: &[u8]) -> Result<(), EncryptError<S>>
        {
            let enc = s.encrypt(m)?;
            let b64 = base64::encode(&enc);
            match self.message.recipients.entry(u) {
                Entry::Occupied(mut e) => {
                    e.get_mut().insert(c, b64);
                }
                Entry::Vacant(e) => {
                    let mut m = HashMap::new();
                    m.insert(c, b64);
                    e.insert(m);
                }
            }
            s.save()?;
            Ok(())
        }
    }

    quick_error! {
        #[derive(Debug)]
        /// Post message error.
        pub enum Error {
            Mismatch(m: ClientMismatch<'static>) {
                display("missing clients")
            }
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

    #[derive(Debug)]
    pub enum EncryptError<T: Store> {
        CBox(CBoxError<T>),
        Pbuf(ProtobufError)
    }

    impl<T: Store> From<CBoxError<T>> for EncryptError<T> {
        fn from(e: CBoxError<T>) -> EncryptError<T> {
            EncryptError::CBox(e)
        }
    }

    impl<T: Store> From<ProtobufError> for EncryptError<T> {
        fn from(e: ProtobufError) -> EncryptError<T> {
            EncryptError::Pbuf(e)
        }
    }

    impl<S: Store> fmt::Display for EncryptError<S> {
        fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
            match *self {
                EncryptError::CBox(ref e) => write!(f, "cbox error: {}", e),
                EncryptError::Pbuf(ref e) => write!(f, "protobuf error: {}", e)
            }
        }
    }

    impl<S: Store + fmt::Debug> std::error::Error for EncryptError<S> {
        fn description(&self) -> &str {
            "EncryptError"
        }

        fn cause(&self) -> Option<&std::error::Error> {
            match *self {
                EncryptError::CBox(ref e) => Some(e),
                EncryptError::Pbuf(ref e) => Some(e)
            }
        }
    }
}
