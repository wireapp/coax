use std::collections::HashMap;
use std::collections::hash_map;
use std::io::Write;

use proteus::keys as proteus;
use proteus::keys::{IdentityKey, PreKeyBundle};
use rustc_serialize::base64::{self, ToBase64, FromBase64};
use json::ast::{Json, Ref};
use json::{FromJson, Decoder, DecodeError, DecodeResult, Utf8Buffer};
use json::{ToJson, Encoder, EncodeError, EncodeResult};
use types::{UserId, ClientId};

// PreKeyMap ////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub struct PreKeyMap {
    pub value: HashMap<UserId, HashMap<ClientId<'static>, Option<PreKey>>>
}

impl PreKeyMap {
    pub fn new() -> PreKeyMap {
        PreKeyMap {
            value: HashMap::new()
        }
    }

    pub fn get(&self, id: &UserId) -> Option<&HashMap<ClientId, Option<PreKey>>> {
        self.value.get(id)
    }
}

impl<'a> IntoIterator for &'a PreKeyMap {
    type Item     = (&'a UserId, &'a HashMap<ClientId<'static>, Option<PreKey>>);
    type IntoIter = hash_map::Iter<'a, UserId, HashMap<ClientId<'static>, Option<PreKey>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.value.iter()
    }
}

impl FromJson for PreKeyMap {
    fn decode<I: Iterator<Item=char>>(d: &mut Decoder<I>) -> DecodeResult<Self> {
        if let Json::Object(o) = d.decode()? {
            let mut user_map = HashMap::with_capacity(o.len());
            for (k, v) in o.into_iter() {
                if let Some(u) = UserId::from_str(k.as_str()) {
                    if let Json::Object(o) = v {
                        let mut clt_map = HashMap::with_capacity(o.len());
                        for (k, v) in o.into_iter() {
                            let r = Ref::new(&v);
                            if let Some(pk) = r.get("key").string().map(PreKey::from_str) {
                                // TODO: compare prekey ids
                                clt_map.insert(ClientId::new(k), pk);
                            } else {
                                return Err(DecodeError::Message("failed to parse prekey"))
                            }
                        }
                        user_map.insert(u, clt_map);
                    } else {
                        return Err(DecodeError::Message("missing client-prekey map"))
                    }
                } else {
                    return Err(DecodeError::Message("failed to parse user id"))
                }
            }
            Ok(PreKeyMap { value: user_map })
        } else {
            Err(DecodeError::Message("expected json object"))
        }
    }
}

// Prekey ///////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct PreKey {
    pub key: PreKeyBundle
}

impl PreKey {
    pub fn last_resort(k: &IdentityKey) -> LastPreKey {
        let pk = proteus::PreKey::last_resort();
        LastPreKey(PreKey { key: PreKeyBundle::new(k.clone(), &pk) })
    }

    pub fn from_str(s: &str) -> Option<PreKey> {
        s.from_base64().ok().and_then(|xs| {
            PreKeyBundle::deserialise(xs.as_slice()).ok().map(|k| PreKey { key: k })
        })
    }
}

impl From<PreKeyBundle> for PreKey {
    fn from(k: PreKeyBundle) -> PreKey {
        PreKey { key: k }
    }
}

impl FromJson for PreKey {
    fn decode<I: Iterator<Item=char>>(d: &mut Decoder<I>) -> DecodeResult<Self> {
        let mut b = [0; 8];
        let mut u = Utf8Buffer::new(&mut b);
        object! {
            let decoder = d;
            let buffer  = &mut u;
            PreKey {
                key: req. "key" => {
                    let b64 = d.string()?;
                    match b64.as_str().from_base64() {
                        Ok(xs) => match PreKeyBundle::deserialise(xs.as_slice()) {
                            Ok(pk) => Ok(pk),
                            Err(e) => Err(DecodeError::Other(Box::new(e)))
                        },
                        Err(e) => Err(DecodeError::Other(Box::new(e)))
                    }
                }
            }
        }
    }
}

impl ToJson for PreKey {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> EncodeResult<()> {
        let key =
            match self.key.serialise() {
                Ok(xs)   => xs,
                Err(err) => return Err(EncodeError::Other(Box::new(err)))
            };
        e.object()?;
            e.key("id")?;  e.u16(self.key.prekey_id.value())?;
            e.key("key")?; e.string(key.as_slice().to_base64(base64::STANDARD))?;
        e.end()
    }
}

// Last prekey //////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub struct LastPreKey(PreKey);

impl LastPreKey {
    pub fn new(p: PreKey) -> Option<LastPreKey> {
        if p.key.prekey_id == proteus::MAX_PREKEY_ID {
            Some(LastPreKey(p))
        } else {
            None
        }
    }
}

impl FromJson for LastPreKey {
    fn decode<I: Iterator<Item=char>>(d: &mut Decoder<I>) -> DecodeResult<Self> {
        Ok(LastPreKey(d.from_json()?))
    }
}

impl ToJson for LastPreKey {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> EncodeResult<()> {
        self.0.encode(e)
    }
}

// Client prekey ////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub struct ClientPreKey<'a> {
    pub id:  ClientId<'a>,
    pub key: PreKey
}

impl<'a> FromJson for ClientPreKey<'a> {
    fn decode<I: Iterator<Item=char>>(d: &mut Decoder<I>) -> DecodeResult<Self> {
        let mut b = [0; 16];
        let mut u = Utf8Buffer::new(&mut b);
        object! {
            let decoder = d;
            let buffer  = &mut u;
            ClientPreKey {
                id:  req. "client" => d.from_json(),
                key: req. "prekey" => d.from_json()
            }
        }
    }
}

// Client prekeys ///////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub struct ClientPreKeys(pub Vec<ClientPreKey<'static>>);

impl FromJson for ClientPreKeys {
    fn decode<I: Iterator<Item=char>>(d: &mut Decoder<I>) -> DecodeResult<Self> {
        let mut b = [0; 16];
        let mut u = Utf8Buffer::new(&mut b);
        let x = extract! {
            let decoder = d;
            let buffer  = &mut u;
            clients: req. Vec<ClientPreKey<'static>> = "clients" => d.from_json()
        }?;
        Ok(ClientPreKeys(x.clients))
    }
}
