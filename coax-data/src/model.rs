use chrono::{DateTime, UTC};
use coax_api::client::{self, Model, Client as ApiClient};
use coax_api::conv::{ConvType, Conversation as ApiConv};
use coax_api::types::{ClientId, ConvId, UserId, Name, Handle, Email, Phone, Label};
use coax_api::user::{ConnectStatus, User as ApiUser, Connection as ApiConn};
use coax_api::user::{AssetSize, AssetKey, AssetToken};
use error::Error;
use util::{from_timestamp, as_id};

use schema::assets;
use schema::clients;
use schema::users;
use schema::conversations;
use schema::connections;
use schema::members;
use schema::messages;
use schema::variables;
use schema::inbox;
use schema::outbox;

// User /////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, AsChangeset, Associations, Identifiable, Insertable, Queryable)]
#[has_many(conversations, foreign_key = "creator")]
#[has_many(clients, foreign_key = "user")]
#[has_many(connections, foreign_key = "id")]
#[has_many(members, foreign_key = "id")]
#[has_many(messages, foreign_key = "from_usr")]
#[table_name = "users"]
pub struct RawUser {
    pub id:     Vec<u8>,
    pub name:   String,
    pub state:  i16,
    pub handle: Option<String>,
    pub email:  Option<String>,
    pub phone:  Option<String>,
    pub icon:   Option<String>
}

impl RawUser {
    pub fn to_user<'a>(self) -> Result<User<'a>, Error> {
        Ok(User {
            id:      as_id(&self.id, "user id")?,
            name:    Name::new(self.name),
            email:   self.email.map(Email::new),
            handle:  self.handle.map(Handle::new),
            phone:   self.phone.map(Phone::new),
            deleted: self.state == 1,
            icon:    self.icon.map(AssetKey::new)
        })
    }
}

#[derive(Debug, Clone)]
pub struct User<'a> {
    pub id:      UserId,
    pub name:    Name<'a>,
    pub email:   Option<Email<'a>>,
    pub handle:  Option<Handle<'a>>,
    pub phone:   Option<Phone<'a>>,
    pub deleted: bool,
    pub icon:    Option<AssetKey<'a>>
}

impl<'a> User<'a> {
    pub fn new(id: UserId, n: Name<'a>) -> User<'a> {
        User {
            id:      id,
            name:    n,
            handle:  None,
            email:   None,
            phone:   None,
            deleted: false,
            icon:    None
        }
    }

    pub fn from_api(u: ApiUser<'a>) -> User<'a> {
        User {
            id:      u.id,
            name:    u.name,
            handle:  u.handle,
            email:   u.email,
            phone:   u.phone,
            deleted: u.deleted.unwrap_or(false),
            icon:    u.assets.iter()
                      .filter(|a| a.size == AssetSize::Preview)
                      .next()
                      .map(|a| a.key.clone())
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, AsChangeset, Insertable, Queryable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub id:     &'a [u8],
    pub name:   &'a str,
    pub state:  i16,
    pub handle: Option<&'a str>,
    pub email:  Option<&'a str>,
    pub phone:  Option<&'a str>,
    pub icon:   Option<&'a str>
}

impl<'a> NewUser<'a> {
    pub fn from_api(u: &'a ApiUser) -> NewUser<'a> {
        NewUser {
            id:     u.id.as_slice(),
            name:   u.name.as_str(),
            state:  if u.deleted == Some(true) { 1 } else { 0 },
            handle: u.handle.as_ref().map(|h| h.as_str()),
            email:  u.email.as_ref().map(|e| e.as_str()),
            phone:  u.phone.as_ref().map(|p| p.as_str()),
            icon:   u.assets.iter()
                     .filter(|a| a.size == AssetSize::Preview)
                     .next()
                     .map(|a| a.key.as_str())
        }
    }
}

// Client ///////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, AsChangeset, Associations, Identifiable, Insertable, Queryable)]
#[belongs_to(RawUser, foreign_key = "user")]
#[table_name = "clients"]
pub struct RawClient {
    pub id:       String,
    pub user:     Vec<u8>,
    pub class:    Option<i16>,
    pub verified: bool,
    pub time:     Option<i64>,
    pub ctype:    Option<i16>,
    pub label:    Option<String>,
    pub model:    Option<String>
}

impl RawClient {
    pub fn to_client<'a>(self) -> Result<Client<'a>, Error> {
        Ok(Client {
            id:       ClientId::new(self.id),
            user:     as_id(&self.user, "user id")?,
            class:    self.class.and_then(|c| client::Class::from_u8(c as u8)),
            verified: self.verified,
            time:     self.time.map(from_timestamp),
            ctype:    self.ctype.and_then(|t| client::Type::from_u8(t as u8)),
            label:    self.label.map(Label::new),
            model:    self.model.map(Model::new)
        })
    }
}

#[derive(Debug, Clone)]
pub struct Client<'a> {
    pub id:       ClientId<'a>,
    pub user:     UserId,
    pub class:    Option<client::Class>,
    pub verified: bool,
    pub time:     Option<DateTime<UTC>>,
    pub ctype:    Option<client::Type>,
    pub label:    Option<Label<'a>>,
    pub model:    Option<Model<'a>>,
}

impl<'a> Client<'a> {
    pub fn from_api(u: UserId, c: ApiClient<'a>, verified: bool) -> Client<'a> {
        Client {
            id:       c.id,
            user:     u.clone(),
            class:    c.class,
            verified: verified,
            time:     c.time,
            ctype:    c.ctype,
            label:    c.label,
            model:    c.model
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, AsChangeset, Insertable, Queryable)]
#[table_name = "clients"]
pub struct NewClient<'a> {
    pub id:       &'a str,
    pub user:     &'a [u8],
    pub class:    Option<i16>,
    pub verified: bool,
    pub time:     Option<i64>,
    pub ctype:    Option<i16>,
    pub label:    Option<&'a str>,
    pub model:    Option<&'a str>
}

impl<'a> NewClient<'a> {
    pub fn from_api(u: &'a UserId, c: &'a ApiClient, v: bool) -> NewClient<'a> {
        NewClient {
            id:       c.id.as_str(),
            user:     u.as_slice(),
            class:    c.class.map(|c| c.into(): u8 as i16),
            verified: v,
            time:     c.time.as_ref().map(DateTime::timestamp),
            ctype:    c.ctype.map(|t| t.into(): u8 as i16),
            label:    c.label.as_ref().map(Label::as_str),
            model:    c.model.as_ref().map(Model::as_str)
        }
    }
}

// Conversation /////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConvStatus {
    Current  = 0,
    Previous = 1
}

impl ConvStatus {
    pub fn from_i16(i: i16) -> Option<ConvStatus> {
        match i {
            0 => Some(ConvStatus::Current),
            1 => Some(ConvStatus::Previous),
            _ => None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, AsChangeset, Associations, Identifiable, Insertable, Queryable)]
#[belongs_to(RawUser, foreign_key = "creator")]
#[has_many(members, foreign_key = "conv")]
#[has_many(messages, foreign_key = "conv")]
#[table_name = "conversations"]
pub struct RawConversation {
    pub id:      Vec<u8>,
    pub name:    Option<String>,
    pub ctype:   i16,
    pub creator: Vec<u8>,
    pub muted:   bool,
    pub time:    i64,
    pub status:  i16
}

impl RawConversation {
    pub fn to_conversation<'a>(self, mm: Vec<UserId>) -> Result<Conversation<'a>, Error> {
        let status =
            if let Some(cs) = ConvStatus::from_i16(self.status) {
                cs
            } else {
                return Err(Error::InvalidData("unknown conversation status"))
            };
        Ok(Conversation {
            id:       as_id(&self.id, "conversation id")?,
            name:     self.name.map(Name::new),
            ctype:    ConvType::from_u8(self.ctype as u8).ok_or(Error::InvalidData("conversation type"))?,
            creator:  as_id(&self.creator, "user id")?,
            muted:    self.muted,
            time:     from_timestamp(self.time),
            status:   status,
            members:  mm
        })
    }
}

#[derive(Debug, Clone)]
pub struct Conversation<'a> {
    pub id:      ConvId,
    pub name:    Option<Name<'a>>,
    pub ctype:   ConvType,
    pub creator: UserId,
    pub muted:   bool,
    pub time:    DateTime<UTC>,
    pub status:  ConvStatus,
    pub members: Vec<UserId>
}

impl<'a> Conversation<'a> {
    pub fn from_api(t: DateTime<UTC>, c: ApiConv<'a>) -> Conversation<'a> {
        let mut mm = c.members.others.into_iter().map(|m| m.id).collect() : Vec<UserId>;
        mm.push(c.members.me.id);
        Conversation {
            id:      c.id,
            name:    c.name,
            ctype:   c.typ,
            creator: c.creator,
            muted:   c.members.me.muted.unwrap_or(false),
            time:    t,
            status:  if c.members.me.current {
                         ConvStatus::Current
                     } else {
                         ConvStatus::Previous
                     },
            members: mm
        }
    }

    pub fn set_name(&mut self, n: Name<'a>) {
        self.name = Some(n)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, AsChangeset, Insertable, Queryable)]
#[table_name = "conversations"]
pub struct NewConversation<'a> {
    pub id:      &'a [u8],
    pub name:    Option<&'a str>,
    pub ctype:   i16,
    pub creator: &'a [u8],
    pub muted:   bool,
    pub time:    i64,
    pub status:  i16
}

impl<'a> NewConversation<'a> {
    pub fn from_api(t: &DateTime<UTC>, c: &'a ApiConv) -> NewConversation<'a> {
        NewConversation {
            id:       c.id.as_slice(),
            name:     c.name.as_ref().map(|n| n.as_str()),
            ctype:    c.typ.into(): u8 as i16,
            creator:  c.creator.as_slice(),
            muted:    c.members.me.muted.unwrap_or(false),
            time:     t.timestamp(),
            status:   if c.members.me.current {
                          ConvStatus::Current as i16
                      } else {
                          ConvStatus::Previous as i16
                      }
        }
    }
}

// Member ///////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, AsChangeset, Associations, Identifiable, Insertable, Queryable)]
#[belongs_to(RawConversation, foreign_key = "conv")]
#[belongs_to(RawUser, foreign_key = "id")]
#[table_name = "members"]
pub struct Member {
    pub id:   Vec<u8>,
    pub conv: Vec<u8>
}

#[derive(Debug, Clone, PartialEq, Eq, AsChangeset, Insertable, Queryable)]
#[table_name = "members"]
pub struct NewMember<'a> {
    pub id:   &'a [u8],
    pub conv: &'a [u8]
}

// Variables ////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, AsChangeset, Insertable, Queryable)]
#[table_name = "variables"]
pub struct NewVar<'a> {
    pub name:  &'a str,
    pub value: &'a [u8]
}

// Connections //////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, AsChangeset, Associations, Identifiable, Insertable, Queryable)]
#[belongs_to(RawUser, foreign_key = "id")]
#[table_name = "connections"]
pub struct RawConnection {
    pub id:      Vec<u8>,
    pub conv:    Vec<u8>,
    pub status:  i16,
    pub message: Option<String>
}

impl RawConnection {
    pub fn to_connection(self) -> Result<Connection, Error> {
        Ok(Connection {
            user:    as_id(&self.id, "user id")?,
            conv:    as_id(&self.conv, "conversation id")?,
            status:  ConnectStatus::from_u8(self.status as u8).ok_or(Error::InvalidData("connect status"))?,
            message: self.message
        })
    }
}

#[derive(Debug, Clone)]
pub struct Connection {
    pub user:    UserId,
    pub conv:    ConvId,
    pub status:  ConnectStatus,
    pub message: Option<String>
}

impl Connection {
    pub fn from_api(c: ApiConn) -> Connection {
        Connection {
            user:    c.to,
            conv:    c.conv,
            status:  c.status,
            message: c.message.map(|cow| cow.into_owned())
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, AsChangeset, Insertable, Queryable)]
#[table_name = "connections"]
pub struct NewConnection<'a> {
    pub id:      &'a [u8],
    pub conv:    &'a [u8],
    pub status:  i16,
    pub message: Option<&'a str>
}

impl<'a> NewConnection<'a> {
    pub fn from_api(c: &'a ApiConn) -> NewConnection<'a> {
        NewConnection {
            id:      c.to.as_slice(),
            conv:    c.conv.as_slice(),
            status:  c.status.into(): u8 as i16,
            message: c.message.as_ref().map(|cow| cow.as_ref()),
        }
    }
}

// Messages /////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, AsChangeset, Associations, Identifiable, Insertable, Queryable)]
#[belongs_to(RawConversation, foreign_key = "conv")]
#[belongs_to(RawUser, foreign_key = "from_usr")]
#[belongs_to(RawAsset, foreign_key = "asset")]
#[table_name = "messages"]
pub struct RawMessage {
    pub id:       String,
    pub conv:     Vec<u8>,
    pub time:     i64,
    pub from_usr: Vec<u8>,
    pub from_clt: Option<String>,
    pub mtype:    i16,
    pub status:   i16,
    pub text:     Option<String>,
    pub user_id:  Option<Vec<u8>>,
    pub asset:    Option<String>
}

impl RawMessage {
    pub fn to_message<'a>(self, s: User<'a>, u: Option<User<'a>>, a: Option<Asset<'a>>) -> Result<Message<'a>, Error> {
        let data = match self.mtype {
            0 => {
                let txt = self.text.ok_or(Error::InvalidData("missing text in message"))?;
                MessageData::Text(txt)
            }
            1 => MessageData::MemberJoined(u),
            2 => MessageData::MemberLeft(u),
            3 => {
                if let Some(asset) = a {
                    MessageData::Asset(asset)
                } else {
                    return Err(Error::InvalidData("missing asset in message"))
                }
            }
            _ => return Err(Error::InvalidData("unknown message type"))
        };
        let status =
            if let Some(ms) = MessageStatus::from_i16(self.status) {
                ms
            } else {
                return Err(Error::InvalidData("unknown message status"))
            };
        Ok(Message {
            id:     self.id,
            conv:   as_id(&self.conv, "conversation id")?,
            time:   from_timestamp(self.time),
            user:   s,
            client: self.from_clt.map(ClientId::new),
            status: status,
            data:   data
        })
    }
}

#[derive(Debug, Clone)]
pub enum MessageData<'a> {
    Text(String),
    Asset(Asset<'a>),
    MemberJoined(Option<User<'a>>), // None == message sender
    MemberLeft(Option<User<'a>>) // None == message sender
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageStatus {
    Created   = 0,
    Received  = 1,
    Sent      = 2,
    Delivered = 3
}

impl MessageStatus {
    pub fn from_i16(i: i16) -> Option<MessageStatus> {
        match i {
            0 => Some(MessageStatus::Created),
            1 => Some(MessageStatus::Received),
            2 => Some(MessageStatus::Sent),
            3 => Some(MessageStatus::Delivered),
            _ => None
        }
    }
}

#[derive(Debug, Clone)]
pub struct Message<'a> {
    pub id:     String,
    pub conv:   ConvId,
    pub time:   DateTime<UTC>,
    pub user:   User<'a>,
    pub client: Option<ClientId<'a>>,
    pub status: MessageStatus,
    pub data:   MessageData<'a>
}

#[derive(Debug, Clone, PartialEq, Eq, AsChangeset, Insertable, Queryable)]
#[table_name = "messages"]
pub struct NewMessage<'a> {
    pub id:       &'a str,
    pub conv:     &'a [u8],
    pub time:     i64,
    pub from_usr: &'a [u8],
    pub from_clt: Option<&'a str>,
    pub mtype:    i16,
    pub status:   i16,
    pub text:     Option<&'a str>,
    pub user_id:  Option<&'a [u8]>,
    pub asset:    Option<&'a str>
}

impl<'a> NewMessage<'a> {
    pub fn text(mid: &'a str, cid: &'a ConvId, t: &DateTime<UTC>, from: &'a UserId, client: &'a ClientId, txt: &'a str) -> NewMessage<'a> {
        NewMessage {
            id:       mid,
            conv:     cid.as_slice(),
            time:     t.timestamp(),
            from_usr: from.as_slice(),
            from_clt: Some(client.as_str()),
            mtype:    0,
            status:   MessageStatus::Created as i16,
            text:     Some(txt),
            user_id:  None,
            asset:    None
        }
    }

    pub fn joined(mid: &'a str, cid: &'a ConvId, t: &DateTime<UTC>, from: &'a UserId, user: &'a UserId) -> NewMessage<'a> {
        NewMessage {
            id:       mid,
            conv:     cid.as_slice(),
            time:     t.timestamp(),
            from_usr: from.as_slice(),
            from_clt: None,
            mtype:    1,
            status:   MessageStatus::Received as i16,
            text:     None,
            user_id:  if from != user { Some(user.as_slice()) } else { None },
            asset:    None
        }
    }

    pub fn left(mid: &'a str, cid: &'a ConvId, t: &DateTime<UTC>, from: &'a UserId, user: &'a UserId) -> NewMessage<'a> {
        NewMessage {
            id:       mid,
            conv:     cid.as_slice(),
            time:     t.timestamp(),
            from_usr: from.as_slice(),
            from_clt: None,
            mtype:    2,
            status:   MessageStatus::Received as i16,
            text:     None,
            user_id:  if from != user { Some(user.as_slice()) } else { None },
            asset:    None
        }
    }

    pub fn asset(mid: &'a str, cid: &'a ConvId, t: &DateTime<UTC>, from: &'a UserId, client: &'a ClientId, asset: &'a AssetKey) -> NewMessage<'a> {
        NewMessage {
            id:       mid,
            conv:     cid.as_slice(),
            time:     t.timestamp(),
            from_usr: from.as_slice(),
            from_clt: Some(client.as_str()),
            mtype:    3,
            status:   MessageStatus::Created as i16,
            text:     None,
            user_id:  None,
            asset:    Some(asset.as_str())
        }
    }

    pub fn set_status(&mut self, s: MessageStatus) {
        self.status = s as i16
    }
}


// Assets ///////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetType {
    Image = 0,
    Audio = 1,
    Video = 2
}

impl AssetType {
    pub fn from_i16(i: i16) -> Option<AssetType> {
        match i {
            0 => Some(AssetType::Image),
            1 => Some(AssetType::Audio),
            2 => Some(AssetType::Video),
            _ => None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetStatus {
    Remote = 0,
    Local  = 1
}

impl AssetStatus {
    pub fn from_i16(i: i16) -> Option<AssetStatus> {
        match i {
            0 => Some(AssetStatus::Remote),
            1 => Some(AssetStatus::Local),
            _ => None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, AsChangeset, Associations, Identifiable, Insertable, Queryable)]
#[table_name = "assets"]
pub struct RawAsset {
    pub id:     String,
    pub atype:  i16,
    pub status: i16,
    pub token:  Option<String>,
    pub key:    Vec<u8>,
    pub cksum:  Vec<u8>
}

impl RawAsset {
    pub fn to_asset<'a>(self) -> Result<Asset<'a>, Error> {
        let ty =
            if let Some(ty) = AssetType::from_i16(self.atype) {
                ty
            } else {
                return Err(Error::InvalidData("unknown asset type"))
            };
        let st =
            if let Some(st) = AssetStatus::from_i16(self.status) {
                st
            } else {
                return Err(Error::InvalidData("unknown asset status"))
            };
        Ok(Asset {
            id:     AssetKey::new(self.id),
            atype:  ty,
            status: st,
            token:  self.token.map(AssetToken::new),
            key:    self.key,
            cksum:  self.cksum
        })
    }
}

#[derive(Debug, Clone)]
pub struct Asset<'a> {
    pub id:     AssetKey<'a>,
    pub atype:  AssetType,
    pub status: AssetStatus,
    pub token:  Option<AssetToken<'a>>,
    pub key:    Vec<u8>,
    pub cksum:  Vec<u8>
}

#[derive(Debug, Clone, PartialEq, Eq, AsChangeset, Insertable, Queryable)]
#[table_name = "assets"]
pub struct NewAsset<'a> {
    pub id:       &'a str,
    pub atype:    i16,
    pub status:   i16,
    pub token:    Option<&'a str>,
    pub key:      &'a [u8],
    pub cksum:    &'a [u8]
}

impl<'a> NewAsset<'a> {
    pub fn new(id: &'a AssetKey, ty: AssetType, st: AssetStatus, ky: &'a[u8], ck: &'a [u8]) -> NewAsset<'a> {
        NewAsset {
            id:     id.as_str(),
            atype:  ty as i16,
            status: st as i16,
            token:  None,
            key:    ky,
            cksum:  ck
        }
    }

    pub fn set_token(&mut self, t: &'a AssetToken) {
        self.token = Some(t.as_str())
    }
}

// Inbox ////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Associations, Identifiable, Insertable, Queryable)]
#[table_name = "inbox"]
pub struct Notification {
    pub id: Vec<u8>
}

#[derive(Debug, Clone, PartialEq, Eq, Associations, Identifiable, Insertable, Queryable)]
#[table_name = "inbox"]
pub struct NewNotification<'a> {
    pub id: &'a [u8]
}

// Outbox ///////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueueItemType { Message }

#[derive(Debug, Clone, PartialEq, Eq, AsChangeset, Associations, Identifiable, Insertable, Queryable)]
#[table_name = "outbox"]
pub struct RawQueueItem {
    pub id:   Vec<u8>,
    pub conv: Vec<u8>,
    pub kind: i16,
    pub data: Vec<u8>,
    pub mesg: Option<Vec<u8>>
}

impl RawQueueItem {
    pub fn to_item(self) -> Result<QueueItem, Error> {
        let data =
            if self.kind == 0 {
                if let Some(m) = self.mesg {
                    QueueItemData::Msg {
                        data: self.data,
                        mesg: m
                    }
                } else {
                    return Err(Error::InvalidData("missing message in queue item"))
                }
            } else {
                return Err(Error::InvalidData("queue item kind"))
            };

        Ok(QueueItem {
            id:   self.id,
            conv: as_id(&self.conv, "conversation id")?,
            data: data
        })
    }
}

#[derive(Debug, Clone)]
pub struct QueueItem {
    pub id:   Vec<u8>,
    pub conv: ConvId,
    pub data: QueueItemData
}

#[derive(Debug, Clone)]
pub enum QueueItemData {
    Msg {
        data: Vec<u8>,
        mesg: Vec<u8>
    }
}

#[derive(Debug, Clone, PartialEq, Eq, AsChangeset, Insertable, Queryable)]
#[table_name = "outbox"]
pub struct NewQueueItem<'a> {
    pub id:   &'a [u8],
    pub conv: &'a [u8],
    pub kind: i16,
    pub data: &'a [u8],
    pub mesg: Option<&'a [u8]>
}

