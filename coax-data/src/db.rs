use std;
use std::fmt;
use std::path::Path;
use std::marker::PhantomData;

use chrono::{DateTime, UTC};
use coax_api as api;
use coax_api::types::{ConvId, UserId, ClientId, NotifId};
use coax_api::user::{AssetKey, ConnectStatus};
use diesel::{self, insert, delete, update};
use diesel::connection::SimpleConnection;
use diesel::expression::sql_literal::sql;
use diesel::prelude::*;
use diesel::result;
use diesel::sqlite::SqliteConnection;
use diesel::sqlite::query_builder::functions::insert_or_replace;
use diesel::types::BigInt;
use error::Error;
use migrations;
use model::{self, NewMember, NewVar, MessageStatus, ConvStatus};
use model::{NewUser, NewClient, NewConnection, NewConversation, NewNotification};
use model::{RawUser, RawClient, RawConnection, RawConversation, RawMessage};
use model::{NewQueueItem, QueueItem, QueueItemType, RawQueueItem};
use model::{RawAsset, AssetStatus, UserUpdate};
use schema;
use slog::Logger;
use util::as_id;

pub struct Database {
    logger: Logger,
    conn:   SqliteConnection
}

const PRAGMAS: &'static str =
    r#"PRAGMA foreign_keys = ON;
       PRAGMA journal_mode = WAL;
       PRAGMA busy_timeout = 5000;"#;

impl Database {
    pub fn open(g: &Logger, path: &Path) -> Result<Database, Error> {
        debug!(g, "opening database"; "path" => ?path);
        let p = path.to_str().ok_or(Error::InvalidPath)?;
        let c = SqliteConnection::establish(p)?;
        let db = Database {
            logger: g.new(o!("context" => "Database")),
            conn:   c
        };
        db.conn.batch_execute(PRAGMAS)?;
        Ok(db)
    }

    pub fn run_migrations(g: &Logger, path: &Path) -> Result<(), Error> {
        debug!(g, "running pending database migrations"; "path" => ?path);
        let p = path.to_str().ok_or(Error::InvalidPath)?;
        let c = SqliteConnection::establish(p)?;
        c.batch_execute(schema::SCHEMA)?;
        let mut w = std::io::stderr();
        diesel::migrations::run_migrations(&c, migrations::all(), &mut w)?;
        Ok(())
    }

    pub fn has_notification(&self, nid: &NotifId) -> Result<bool, Error> {
        use schema::inbox::dsl::*;
        debug!(self.logger, "has notification?"; "id" => %nid);
        match inbox.find(nid.as_slice()).first::<model::Notification>(&self.conn) {
            Err(result::Error::NotFound) => Ok(false),
            Err(e) => Err(Error::Result(e)),
            Ok(n)  => Ok(n.id.as_slice() == nid.as_slice())
        }
    }

    pub fn insert_notification(&self, nid: &NotifId) -> Result<(), Error> {
        debug!(self.logger, "insert notification"; "id" => %nid);
        let n = NewNotification { id: nid.as_slice() };
        insert_or_replace(&n).into(schema::inbox::table).execute(&self.conn)?;
        Ok(())
    }

    pub fn last_notification(&self) -> Result<Option<NotifId>, Error> {
        trace!(self.logger, "last notification");
        if let Some(blob) = self.var("last-notif")? {
            Ok(NotifId::from_bytes(&blob))
        } else {
            Ok(None)
        }
    }

    pub fn set_last_notification(&self, i: &NotifId) -> Result<(), Error> {
        debug!(self.logger, "set last notification"; "id" => %i);
        self.set_var("last-notif", i.as_bytes())?;
        Ok(())
    }

    pub fn user<'a>(&self, uid: &UserId) -> Result<Option<model::User<'a>>, Error> {
        use schema::users::dsl::*;
        debug!(self.logger, "select"; "user" => %uid);
        match users.find(uid.as_slice()).first::<RawUser>(&self.conn) {
            Err(result::Error::NotFound) => Ok(None),
            Err(e) => Err(Error::Result(e)),
            Ok(u)  => Ok(Some(u.to_user()?))
        }
    }

    pub fn insert_user(&self, u: &api::user::User) -> Result<(), Error> {
        use schema::users::dsl::*;
        debug!(self.logger, "insert"; "user" => %u.id);
        let nu = NewUser::from_api(u);
        self.conn.transaction(|| {
            match users.find(u.id.as_slice()).first::<RawUser>(&self.conn) {
                Err(result::Error::NotFound) => {
                    insert(&nu).into(schema::users::table).execute(&self.conn)?;
                }
                Err(e) => return Err(e),
                Ok(_)  => {
                    update(users.find(u.id.as_slice())).set(&nu).execute(&self.conn)?;
                }
            }
            Ok(())
        }).map_err(From::from)
    }

    pub fn update_user(&self, u: &api::user::UserUpdate) -> Result<(), Error> {
        use schema::users::dsl::*;
        debug!(self.logger, "update"; "user" => %u.id);
        let chg = UserUpdate::from_api(u);
        update(users.find(u.id.as_slice())).set(&chg).execute(&self.conn)?;
        Ok(())
    }

    pub fn client<'a>(&self, uid: &UserId, cid: &ClientId) -> Result<Option<model::Client<'a>>, Error> {
        use schema::clients::dsl::*;
        debug!(self.logger, "select"; "user" => %uid, "client" => %cid);
        let source = clients
            .filter(user.eq(uid.as_slice()))
            .filter(id.eq(cid.as_str()));
        match source.first::<RawClient>(&self.conn) {
            Err(result::Error::NotFound) => Ok(None),
            Err(e) => Err(Error::Result(e)),
            Ok(c)  => Ok(Some(c.to_client()?)),
        }
    }

    pub fn clients<'a>(&self, uid: &UserId) -> Result<Vec<model::Client<'a>>, Error> {
        use schema::clients::dsl::*;
        debug!(self.logger, "select clients"; "user" => %uid);
        let source = clients
            .filter(user.eq(uid.as_slice()));
        match source.load::<RawClient>(&self.conn) {
            Err(result::Error::NotFound) => Ok(Vec::new()),
            Err(e) => Err(Error::Result(e)),
            Ok(v)  => {
                let mut cc = Vec::with_capacity(v.len());
                for r in v {
                    cc.push(r.to_client()?)
                }
                Ok(cc)
            }
        }
    }

    pub fn insert_client(&self, u: &UserId, c: &api::client::Client) -> Result<(), Error> {
        use schema::clients::dsl::*;
        debug!(self.logger, "insert"; "user" => %u, "client" => %c.id);
        let nc = NewClient::from_api(u, c, false);
        self.conn.transaction(|| {
            // TODO: upsert
            match clients.filter(user.eq(u.as_slice())).filter(id.eq(c.id.as_str())).first::<RawClient>(&self.conn) {
                Err(result::Error::NotFound) => {
                    insert(&nc).into(schema::clients::table).execute(&self.conn)?;
                }
                Err(e) => return Err(e),
                _      => ()
            }
            Ok(())
        }).map_err(From::from)
    }

    pub fn insert_clients(&self, u: &UserId, cs: &[api::client::Client]) -> Result<(), Error> {
        debug!(self.logger, "insert clients"; "user" => %u);
        // TODO: upsert
        for c in cs {
            self.insert_client(u, c)?
        }
//        let mut ncs = Vec::with_capacity(cs.len());
//        for c in cs {
//            ncs.push(NewClient::from_api(u, c, false))
//        }
//        insert(&ncs).into(schema::clients::table).execute(&self.conn)?;
        Ok(())
    }

    pub fn remove_client(&self, u: &UserId, c: &ClientId) -> Result<(), Error> {
        use schema::clients::dsl::*;
        info!(self.logger, "remove"; "user" => %u, "client" => %c);
        delete(clients.find((u.as_slice(), c.as_str()))).execute(&self.conn)?;
        Ok(())
    }

    pub fn connection<'a>(&self, uid: &UserId) -> Result<Option<(model::Connection, model::User<'a>)>, Error> {
        use schema::connections::dsl::*;
        debug!(self.logger, "select connection"; "to" => %uid);
        let source = connections
            .inner_join(schema::users::table)
            .filter(id.eq(uid.as_slice()));
        match source.first::<(RawConnection, RawUser)>(&self.conn) {
            Err(result::Error::NotFound) => Ok(None),
            Err(e) => Err(Error::Result(e)),
            Ok(cu) => {
                let c = cu.0.to_connection()?;
                let u = cu.1.to_user()?;
                Ok(Some((c, u)))
            }
        }
    }

    pub fn connections<'a>(&self) -> Result<Vec<(model::User<'a>, model::Connection)>, Error> {
        use schema::users::dsl::*;
        use schema::connections;
        debug!(self.logger, "select all connections");
        let source = users
            .inner_join(connections::table)
            .filter(id.eq(connections::id));
        match source.load::<(RawUser, RawConnection)>(&self.conn) {
            Err(result::Error::NotFound) => Ok(Vec::new()),
            Err(e) => Err(Error::Result(e)),
            Ok(uc) => {
                let mut vec = Vec::with_capacity(uc.len());
                for (u, c) in uc {
                    let usr = u.to_user()?;
                    let con = c.to_connection()?;
                    vec.push((usr, con))
                }
                Ok(vec)
            }
        }
    }

    pub fn insert_connection(&self, c: &api::user::Connection) -> Result<(), Error> {
        debug!(self.logger, "insert"; "connection" => %c.to);
        let nc = NewConnection::from_api(c);
        insert_or_replace(&nc).into(schema::connections::table).execute(&self.conn)?;
        Ok(())
    }

    pub fn update_connection(&self, uid: &UserId, s: ConnectStatus) -> Result<bool, Error> {
        use schema::connections::dsl::*;
        debug!(self.logger, "updating connection"; "to" => %uid, "status" => ?s);
        let val = s.into() : u8 as i16;
        update(connections.find(uid.as_slice()))
            .set(status.eq(val))
            .execute(&self.conn)
            .map(|n| n > 0)
            .map_err(From::from)
    }

    pub fn conversation<'a>(&self, cid: &ConvId) -> Result<Option<model::Conversation<'a>>, Error> {
        use schema::conversations::dsl::*;
        debug!(self.logger, "select"; "conv" => %cid);
        let source = conversations.find(cid.as_slice());
        match source.first::<RawConversation>(&self.conn) {
            Err(result::Error::NotFound) => Ok(None),
            Err(e) => Err(Error::Result(e)),
            Ok(c)  => {
                let mm = self.member_ids(cid)?;
                let cv = c.to_conversation(mm)?;
                Ok(Some(cv))
            }
        }
    }

    pub fn conversations<'a>(&self, from: Option<PagingState<C>>, lim: usize) -> Result<Page<Vec<model::Conversation<'a>>, C>, Error> {
        use schema::conversations::dsl::*;
        debug!(self.logger, "select conversations");
        let mut source = conversations
            .order(time.desc())
            .limit(lim as i64)
            .into_boxed();
        if let Some(ref p) = from {
            source = source.offset(p.state);
        }
        match source.load::<RawConversation>(&self.conn) {
            Err(result::Error::NotFound) => Ok(Page::new(Vec::new(), from.unwrap_or(PagingState::zero()))),
            Err(e) => Err(Error::Result(e)),
            Ok(rr) => {
                let mut vec = Vec::with_capacity(rr.len());
                for r in rr {
                    let mut cv = r.to_conversation(Vec::new())?;
                    cv.members = self.member_ids(&cv.id)?;
                    vec.push(cv)
                }
                let ps = from.map(|p| p.forward(vec.len() as i64)).unwrap_or(PagingState::new(vec.len() as i64));
                Ok(Page::new(vec, ps))
            }
        }
    }

    pub fn update_conv_time(&self, cid: &ConvId, t: i64) -> Result<bool, Error> {
        use schema::conversations::dsl::*;
        debug!(self.logger, "updating conversation time"; "id" => %cid, "value" => t);
        update(conversations.find(cid.as_slice()))
            .set(time.eq(t))
            .execute(&self.conn)
            .map(|n| n > 0)
            .map_err(From::from)
    }

    pub fn update_conv_status(&self, cid: &ConvId, s: ConvStatus) -> Result<bool, Error> {
        use schema::conversations::dsl::*;
        debug!(self.logger, "updating conversation status"; "id" => %cid, "value" => ?s);
        update(conversations.find(cid.as_slice()))
            .set(status.eq(s as i16))
            .execute(&self.conn)
            .map(|n| n > 0)
            .map_err(From::from)
    }

    pub fn update_conv_name(&self, cid: &ConvId, n: &str) -> Result<(), Error> {
        use schema::conversations::dsl::*;
        debug!(self.logger, "update conversation name"; "id" => %cid);
        update(conversations.find(cid.as_slice()))
            .set(name.eq(n))
            .execute(&self.conn)?;
        Ok(())
    }

    pub fn insert_conversation(&self, t: &DateTime<UTC>, c: &api::conv::Conversation) -> Result<(), Error> {
        use schema::conversations::dsl::*;
        debug!(self.logger, "insert"; "conv" => %c.id);
        let ci = c.id.as_slice();
        let mut mm = c.members.others.iter()
            .map(|m| NewMember { id: m.id.as_slice(), conv: ci })
            .collect() : Vec<NewMember>;
        mm.push(NewMember { id: c.members.me.id.as_slice(), conv: ci });
        let nc = NewConversation::from_api(t, c);
        self.conn.transaction(|| {
            // TODO: upsert
            match conversations.find(ci).first::<RawConversation>(&self.conn) {
                Err(result::Error::NotFound) => {
                    insert(&nc).into(schema::conversations::table).execute(&self.conn)?;
                }
                Err(e) => return Err(e),
                _      => ()
            }
            insert_or_replace(&mm).into(schema::members::table).execute(&self.conn)?;
            Ok(())
        })?;
        Ok(())
    }

    pub fn member_ids(&self, cid: &ConvId) -> Result<Vec<UserId>, Error> {
        use schema::members::dsl::*;
        debug!(self.logger, "select member ids"; "conv" => %cid);
        let source = members
            .filter(conv.eq(cid.as_slice()))
            .select(id);
        match source.load::<Vec<u8>>(&self.conn) {
            Err(result::Error::NotFound) => Ok(Vec::new()),
            Err(e) => Err(Error::Result(e)),
            Ok(mm) => {
                let mut ids = Vec::with_capacity(mm.len());
                for m in mm {
                    ids.push(as_id(&m, "user id")?)
                }
                Ok(ids)
            }
        }
    }

    pub fn insert_members(&self, cid: &ConvId, users: &[&UserId]) -> Result<(), Error> {
        debug!(self.logger, "insert members"; "conv" => %cid);
        let mm: Vec<NewMember> = users.iter()
            .map(|u| NewMember { id: u.as_slice(), conv: cid.as_slice() })
            .collect();
        insert_or_replace(&mm).into(schema::members::table).execute(&self.conn)?;
        Ok(())
    }

    pub fn remove_members(&self, cid: &ConvId, users: &[&UserId]) -> Result<(), Error> {
        use schema::members::dsl::*;
        info!(self.logger, "remove members"; "conv" => %cid);
        let condition = conv.eq(cid.as_slice())
            .and(id.eq_any(users.iter().map(|uid| uid.as_slice())));
        delete(members.filter(condition)).execute(&self.conn)?;
        Ok(())
    }

    pub fn members<'a>(&self, cid: &ConvId) -> Result<Vec<model::User<'a>>, Error> {
        use schema::members::dsl::*;
        debug!(self.logger, "select members"; "conv" => %cid);
        // many to many joins are not supported yet (cf. diesel issue #398)
        let source = members
            .filter(conv.eq(cid.as_slice()))
            .select(id);
        match source.load::<Vec<u8>>(&self.conn) {
            Err(result::Error::NotFound) => Ok(Vec::new()),
            Err(e) => Err(Error::Result(e)),
            Ok(mm) => {
                let mut vec = Vec::with_capacity(mm.len());
                for m in mm {
                    if let Some(u) = self.user(&as_id(&m, "user id")?)? {
                        vec.push(u)
                    }
                }
                Ok(vec)
            }
        }
    }

    pub fn messages<'a>(&self, cid: &ConvId, from: Option<PagingState<M>>, lim: usize) -> Result<Page<Vec<model::Message<'a>>, M>, Error> {
        use schema::users;
        use schema::messages::dsl::*;
        debug!(self.logger, "select messages"; "conv" => %cid);
        let mut source = messages
            .inner_join(users::table)
            .filter(conv.eq(cid.as_slice()))
            .filter(from_usr.eq(users::id))
            .limit(lim as i64)
            .order(sql::<BigInt>("messages.rowid").desc())
            .into_boxed();
        if let Some(ref p) = from {
            source = source.offset(p.state);
        }
        match source.load::<(RawMessage, RawUser)>(&self.conn) {
            Err(result::Error::NotFound) => Ok(Page::new(Vec::new(), from.unwrap_or(PagingState::zero()))),
            Err(e) => Err(Error::Result(e)),
            Ok(mm) => {
                let mut vec = Vec::with_capacity(mm.len());
                for (m, s) in mm {
                    let a =
                        if let Some(ref ai) = m.asset {
                            self.asset(&AssetKey::new(ai.as_ref()))?
                        } else {
                            None
                        };
                    match m.user_id.as_ref().map(|xs| UserId::from_bytes(xs)) {
                        None            => vec.push(m.to_message(s.to_user()?, None, a)?),
                        Some(Some(uid)) => vec.push(m.to_message(s.to_user()?, self.user(&uid)?, a)?),
                        Some(None)      => {
                            error!(self.logger, "invalid messages.user_id"; "conv" => %cid);
                            return Err(Error::InvalidData("messages.user_id"))
                        }
                    }
                }
                let ps = from.map(|p| p.forward(vec.len() as i64)).unwrap_or(PagingState::new(vec.len() as i64));
                Ok(Page::new(vec, ps))
            }
        }
    }

    pub fn insert_message(&self, nm: &model::NewMessage) -> Result<(), Error> {
        debug!(self.logger, "insert message";
               "conv"   => ?ConvId::from_bytes(nm.conv),
               "id"     => nm.id,
               "status" => nm.status);
        insert_or_replace(nm).into(schema::messages::table).execute(&self.conn)?;
        Ok(())
    }

    pub fn update_message_status(&self, cid: &ConvId, mid: &str, s: MessageStatus) -> Result<bool, Error> {
        use schema::messages::dsl::*;
        debug!(self.logger, "updating message status"; "id" => mid, "status" => ?s);
        update(messages.find((cid.as_slice(), mid)))
            .set(status.eq(s as i16))
            .execute(&self.conn)
            .map(|n| n > 0)
            .map_err(From::from)
    }

    pub fn update_message_time(&self, cid: &ConvId, mid: &str, t: &DateTime<UTC>) -> Result<bool, Error> {
        use schema::messages::dsl::*;
        debug!(self.logger, "updating message time"; "id" => mid, "time" => t.timestamp());
        update(messages.find((cid.as_slice(), mid)))
            .set(time.eq(t.timestamp()))
            .execute(&self.conn)
            .map(|n| n > 0)
            .map_err(From::from)
    }

    pub fn message_conversation_id(&self, mid: &str) -> Result<Option<ConvId>, Error> {
        use schema::messages::dsl::*;
        debug!(self.logger, "select conversation of message"; "id" => mid);
        let source = messages.filter(id.eq(mid)).select(conv);
        match source.first::<Vec<u8>>(&self.conn) {
            Err(result::Error::NotFound) => Ok(None),
            Err(e)  => Err(Error::Result(e)),
            Ok(cid) => as_id(&cid, "conversation id").map(Some)
        }
    }

    pub fn asset<'a>(&self, aid: &AssetKey) -> Result<Option<model::Asset<'a>>, Error> {
        use schema::assets::dsl::*;
        debug!(self.logger, "select asset"; "id" => %aid);
        match assets.find(aid.as_str()).first::<RawAsset>(&self.conn) {
            Err(result::Error::NotFound) => Ok(None),
            Err(e)                       => Err(Error::Result(e)),
            Ok(a)                        => a.to_asset().map(Some)
        }
    }

    pub fn insert_asset(&self, na: &model::NewAsset) -> Result<(), Error> {
        debug!(self.logger, "insert asset"; "id" => na.id, "status" => na.status);
        insert_or_replace(na).into(schema::assets::table).execute(&self.conn)?;
        Ok(())
    }

    pub fn update_asset_status(&self, k: &AssetKey, s: AssetStatus) -> Result<bool, Error> {
        use schema::assets::dsl::*;
        debug!(self.logger, "updating asset status"; "id" => %k, "status" => ?s);
        update(assets.find(k.as_str()))
            .set(status.eq(s as i16))
            .execute(&self.conn)
            .map(|n| n > 0)
            .map_err(From::from)
    }

    /// Select value by lookup key.
    pub fn var(&self, n: &str) -> Result<Option<Vec<u8>>, Error> {
        use schema::variables::dsl::*;
        debug!(self.logger, "select variable"; "name" => n);
        match variables.find(n).select(value).first(&self.conn) {
            Ok(x)                        => Ok(Some(x)),
            Err(result::Error::NotFound) => Ok(None),
            Err(e)                       => Err(Error::Result(e))
        }
    }

    /// Insert or replace key-value pair.
    pub fn set_var(&self, n: &str, v: &[u8]) -> Result<(), Error> {
        debug!(self.logger, "insert variable"; "name" => n);
        let nv = NewVar { name: n, value: v };
        insert_or_replace(&nv).into(schema::variables::table).execute(&self.conn)?;
        Ok(())
    }

    pub fn enqueue_message(&self, id: &[u8], conv: &ConvId, data: &[u8], msg: &[u8]) -> Result<(), Error> {
        debug!(self.logger, "add message to outbox";
               "id"   => %String::from_utf8_lossy(id),
               "conv" => %conv);
        let item = NewQueueItem {
            id:   id,
            conv: conv.as_slice(),
            kind: QueueItemType::Message as i16,
            data: data,
            mesg: Some(msg)
        };
        insert_or_replace(&item).into(schema::outbox::table).execute(&self.conn)?;
        Ok(())
    }

    pub fn dequeue(&self, id: &[u8], conv: &ConvId) -> Result<(), Error> {
        use schema::outbox::dsl::outbox;
        debug!(self.logger, "remove from outbox";
               "id"   => %String::from_utf8_lossy(id),
               "conv" => %conv);
        delete(outbox.find((conv.as_slice(), id))).execute(&self.conn)?;
        Ok(())
    }

    pub fn queue_items(&self, from: Option<PagingState<Q>>, num: usize) -> Result<Page<Vec<QueueItem>, Q>, Error> {
        use schema::outbox::dsl::*;
        debug!(self.logger, "select outbox items"; "size" => num, "paging-state" => ?from);
        let mut source = outbox
            .limit(num as i64)
            .order(sql::<BigInt>("outbox.rowid").asc())
            .select((sql::<BigInt>("outbox.rowid"), id, conv, kind, data, mesg))
            .into_boxed();
        if let Some(ref p) = from {
            source = source.filter(sql::<BigInt>("outbox.rowid").gt(p.state));
        }
        match source.load::<(i64, Vec<u8>, Vec<u8>, i16, Vec<u8>, Option<Vec<u8>>)>(&self.conn) {
            Err(result::Error::NotFound) => Ok(Page::new(Vec::new(), from.unwrap_or(PagingState::zero()))),
            Err(e) => Err(Error::Result(e)),
            Ok(qi) => {
                let mut items = Vec::new();
                let mut rowid = 0;
                for i in qi {
                    rowid = i.0;
                    let r = RawQueueItem {
                        id:   i.1,
                        conv: i.2,
                        kind: i.3,
                        data: i.4,
                        mesg: i.5
                    };
                    items.push(r.to_item()?)
                }
                Ok(Page::new(items, PagingState::new(rowid)))
            }
        }
    }
}


#[derive(Clone)] pub struct C;
#[derive(Clone)] pub struct M;
#[derive(Clone)] pub struct Q;

#[derive(Clone)]
pub struct PagingState<T>{
    state:    i64,
    _phantom: PhantomData<T>
}

impl<T> PagingState<T> {
    fn zero() -> PagingState<T> {
        PagingState::new(0)
    }

    fn new(s: i64) -> PagingState<T> {
        PagingState {
            state:    s,
            _phantom: PhantomData
        }
    }

    fn forward(self, s: i64) -> PagingState<T> {
        PagingState::new(self.state + s)
    }
}

impl<T> fmt::Debug for PagingState<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PagingState {{ state: {} }}", self.state)
    }
}

pub struct Page<T, P> {
    pub data:  T,
    pub state: PagingState<P>
}

impl<T, P> Page<T, P> {
    fn new(data: T, p: PagingState<P>) -> Page<T, P> {
        Page { data: data, state: p }
    }
}

