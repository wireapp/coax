use std::borrow::Cow;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::fs::{self, DirBuilder, File};
use std::io::{self, Cursor, Read};
use std::path::PathBuf;
use std::str;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use std::thread::{self, JoinHandle};
use std::time::Duration;

use chrono::{DateTime, UTC};
use coax_api as api;
use coax_api::client::{self, Client as ApiClient, ClientMismatch, SignalingKeys, Model};
use coax_api::conv::ConvType;
use coax_api::events::{self, Notification, Event, EventType, UserEvent, ConvEvent, ConvEventData};
use coax_api::message::send;
use coax_api::prekeys::{PreKey, LastPreKey};
use coax_api::token::{AccessToken, Credentials};
use coax_api::types::{Label, Password, ClientId, UserId, ConvId, Name, random_uuid};
use coax_api::user::{self, Connection as UserConnection, ConnectStatus, User as ApiUser, AssetKey, AssetToken};
use coax_api_proto::{Builder, GenericMessage};
use coax_api_proto::builder::Confirm;
use coax_client;
use coax_client::error::{Error as ClientError, Void};
use coax_client::client::Client;
use coax_client::listen::Listener;
use coax_data::{self as data, Database, Connection, Conversation, User, ConvStatus};
use coax_data::{MessageStatus, MessageData, NewMessage, QueueItem, QueueItemData};
use coax_data::{NewAsset, AssetStatus, AssetType};
use coax_data::db::{self, PagingState};
use coax_net::http::tls::{Tls, TlsStream};
use coax_ws::io::{Error as WsError};
use config;
use cookie::Cookie;
use error::{self, Error, React};
use cryptobox::{CBox, CBoxSession};
use cryptobox::store::file::FileStore;
use json::{ToJson, Encoder, Decoder};
use json::decoder::ReadIter;
use openssl::hash::{Hasher, MessageDigest};
use openssl::symm;
use pkg::Pkg;
use proteus::keys::MAX_PREKEY_ID;
use proteus::keys::PreKeyId;
use protobuf::{self, Message};
use slog::Logger;
use tempdir::TempDir;
use url::Url;

pub struct Actor<S> {
    logger: Logger,
    config: config::Main,
    tls:    Arc<Tls>,
    state:  S,
}

// Actor states

pub struct Init(());

pub struct Connected {
    client: Client<'static>
}

pub struct Offline {
    user:  UserData,
    bcast: Sender<Pkg>
}

pub struct Online {
    user:   UserData,
    bcast:  Sender<Pkg>,
    client: Client<'static>
}

// Additional data types

pub struct UserData {
    user:   User<'static>,
    dbase:  Database,
    creds:  Arc<Mutex<Credentials<'static, Cookie<'static>>>>,
    device: Device,
    assets: PathBuf
}

#[derive(Clone)]
struct Device {
    fresh:  bool,
    client: data::Client<'static>,
    cbox:   CBox<FileStore>
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Delivery {
    OneShot,
    Persistent
}

// Init state operations ////////////////////////////////////////////////////

impl Actor<Init> {
    /// Create a new `Actor` value.
    pub fn new(g: &Logger, cfg: config::Main, tls: Arc<Tls>) -> Result<Actor<Init>, Error> {
        Ok(Actor {
            logger: g.new(o!("context" => "Actor")),
            config: cfg,
            tls:    tls,
            state:  Init(())
        })
    }

    /// Transition to `Connected` state.
    pub fn connected(self, c: Client<'static>) -> Actor<Connected> {
        debug!(self.logger, "Init -> Connected");
        Actor {
            logger: self.logger,
            config: self.config,
            tls:    self.tls,
            state:  Connected { client: c }
        }
    }

    /// Transition to `Offline` state.
    pub fn offline(self, u: UserData, tx: Sender<Pkg>) -> Actor<Offline> {
        debug!(self.logger, "Init -> Offline");
        Actor {
            logger: self.logger,
            config: self.config,
            tls:    self.tls,
            state:  Offline {
                user:  u,
                bcast: tx
            }
        }
    }

    /// Transition to `Online` state
    pub fn online(self, c: Client<'static>, u: UserData, tx: Sender<Pkg>) -> Actor<Online> {
        debug!(self.logger, "Init -> Online");
        Actor {
            logger: self.logger,
            config: self.config,
            tls:    self.tls,
            state:  Online {
                user:   u,
                client: c,
                bcast:  tx
            }
        }
    }

    /// Load an existing user profile from local storage.
    pub fn profile(&mut self, user_id: &UserId) -> Result<UserData, Error> {
        let assets = self.mk_assets_dir(user_id)?;

        let db_path = self.database_path(user_id)?;
        Database::run_migrations(&self.logger, &db_path)?;

        let dbase = self.open_database(user_id)?;

        let user = dbase.user(user_id)?
            .ok_or(Error::Profile(user_id.clone(), "user not found"))?;

        let client_id = my_client_id(&dbase)?
            .ok_or(Error::Profile(user_id.clone(), "client id not found"))?;

        let client = dbase.client(user_id, &client_id)?
            .ok_or(Error::Profile(user_id.clone(), "client not found"))?;

        let cookie = cookie(&dbase)?
            .ok_or(Error::Profile(user_id.clone(), "cookie not found"))?;

        let token = access_token(&dbase)?.map(|s| AccessToken::new(s, Duration::from_millis(0)))
            .ok_or(Error::Profile(user_id.clone(), "access token not found"))?;

        let creds = Credentials::new(token, cookie);
        let cbox  = self.open_cryptobox(user_id)?;

        Ok(UserData {
            user:   user,
            dbase:  dbase,
            creds:  Arc::new(Mutex::new(creds)),
            device: Device {
                fresh:  false,
                client: client,
                cbox:   cbox
            },
            assets: assets
        })
    }
}

// Connected state operations ///////////////////////////////////////////////

impl Actor<Connected> {
    /// Transition to `Online` state
    pub fn online(self, u: UserData, tx: Sender<Pkg>) -> Actor<Online> {
        debug!(self.logger, "Connected -> Online");
        Actor {
            logger: self.logger,
            config: self.config,
            tls:    self.tls,
            state:  Online {
                user:   u,
                client: self.state.client,
                bcast:  tx
            }
        }
    }

    /// Transition to `Offline` state
    pub fn offline(self, u: UserData, tx: Sender<Pkg>) -> Actor<Offline> {
        debug!(self.logger, "Connected -> Offline");
        Actor {
            logger: self.logger,
            config: self.config,
            tls:    self.tls,
            state:  Offline {
                user:  u,
                bcast: tx
            }
        }
    }

    /// Register a new user with back-end.
    pub fn register_user(&mut self, p: &user::register::Params) -> Result<(), Error> {
        error::retry3x(|r: Option<React<()>>| {
            if r.is_some() {
                self.state.client.reconnect()?
            }
            self.state.client.user_register(p)?;
            Ok(())
        })
    }

    /// Login an existing user.
    ///
    /// If `persist` is `true`, we will store the access credentials in
    /// our local database.
    pub fn login(&mut self, p: &user::login::Params) -> Result<UserData, Error> {
        let credentials = error::retry3x(|r: Option<React<()>>| {
            if r.is_some() {
                self.state.client.reconnect()?
            }
            self.state.client.user_login(p).map_err(From::from)
        })?;

        let user = self.lookup_self(&credentials.token)?;

        let db_path = self.database_path(&user.id)?;
        Database::run_migrations(&self.logger, &db_path)?;

        let dbase  = self.open_database(&user.id)?;
        let cbox   = self.open_cryptobox(&user.id)?;
        let assets = self.mk_assets_dir(&user.id)?;

        set_access_token(&dbase, &credentials.token)?;
        set_cookie(&dbase, &credentials.cookie)?;
        dbase.insert_user(&user)?;

        let (client, fresh) =
            if let Some(cid) = my_client_id(&dbase)? {
                if let Some(client) = dbase.client(&user.id, &cid)? {
                    (client, false)
                } else {
                    let client = self.lookup_self_client(&cid, &credentials.token)?;
                    dbase.insert_client(&user.id, &client)?;
                    (data::Client::from_api(user.id.clone(), client, false), false)
                }
            } else {
                let client = self.register_client(&cbox, &credentials.token, p.pass.replicate())?;
                set_my_client_id(&dbase, &client.id)?;
                dbase.insert_client(&user.id, &client)?;
                (data::Client::from_api(user.id.clone(), client, false), true)
            };

        Ok(UserData {
            user:   data::User::from_api(user),
            dbase:  dbase,
            creds:  Arc::new(Mutex::new(credentials)),
            assets: assets,
            device: Device {
                fresh:  fresh,
                client: client,
                cbox:   cbox
            }
        })
    }

    // Lookup our own user profile.
    fn lookup_self(&mut self, t: &AccessToken) -> Result<ApiUser<'static>, Error> {
        error::retry3x(|r: Option<React<()>>| {
            if r.is_some() {
                self.state.client.reconnect()?
            }
            self.state.client.self_user(t).map_err(From::from)
        })
    }

    // Lookup our own client information by ID.
    fn lookup_self_client(&mut self, cid: &ClientId, t: &AccessToken) -> Result<ApiClient<'static>, Error> {
        error::retry3x(|r: Option<React<()>>| {
            if r.is_some() {
                self.state.client.reconnect()?
            }
            if let Some(c) = self.state.client.self_client(cid, t)? {
                Ok(c)
            } else {
                Err(Error::Message("client not found"))
            }
        })
    }

    // Register a new client with back-end.
    fn register_client(&mut self, cbox: &CBox<FileStore>, t: &AccessToken, pw: Password) -> Result<ApiClient<'static>, Error> {
        let mut pkeys = Vec::new();
        for i in 0 .. 200 {
            pkeys.push(PreKey {
                key: cbox.new_prekey(PreKeyId::new(i))?
            })
        }
        let lkey = LastPreKey::new(PreKey {
            key: cbox.new_prekey(MAX_PREKEY_ID)?
        }).unwrap();
        let params = client::register::Params {
            prekeys:      Cow::Owned(pkeys),
            last_prekey:  Cow::Owned(lkey),
            sig_keys:     SignalingKeys::new(),
            ctype:        client::Type::Permanent,
            class:        client::Class::Desktop,
            label:        None,
            cookie_label: Label::new(format!("Coax-{}", cbox.fingerprint())),
            password:     Some(pw),
            model:        Some(Model::new("Coax"))
        };
        error::retry3x(|r: Option<React<()>>| {
            if r.is_some() {
                self.state.client.reconnect()?
            }
            self.state.client.client_register(&params, t).map_err(From::from)
        })
    }
}

// Offline state operations /////////////////////////////////////////////////

impl Actor<Offline> {
    /// Transition to `Init` state.
    pub fn init(self) -> Actor<Init> {
        debug!(self.logger, "Offline -> Init");
        Actor {
            logger: self.logger,
            config: self.config,
            tls:    self.tls,
            state:  Init(())
        }
    }

    /// Transition to `Online` state.
    pub fn online(self, c: Client<'static>) -> Actor<Online> {
        debug!(self.logger, "Offline -> Online");
        Actor {
            logger: self.logger,
            config: self.config,
            tls:    self.tls,
            state:  Online {
                user:   self.state.user,
                client: c,
                bcast:  self.state.bcast
            }
        }
    }

    /// Transition to `Connected` state.
    pub fn connected(self, c: Client<'static>) -> Actor<Connected> {
        debug!(self.logger, "Offline -> Connected");
        Actor {
            logger: self.logger,
            config: self.config,
            tls:    self.tls,
            state:  Connected { client: c }
        }
    }

    /// Our own user information.
    pub fn me(&self) -> &User<'static> {
        &self.state.user.user
    }

    /// Our own client information.
    pub fn client(&self) -> &data::Client<'static> {
        &self.state.user.device.client
    }

    pub fn load_conversations<'a>(&mut self, from: Option<PagingState<db::C>>, num: usize) -> Result<db::Page<Vec<Conversation<'a>>, db::C>, Error> {
        debug!(self.logger, "loading conversations from database");
        self.state.user.dbase.conversations(from, num).map_err(From::from)
    }

    pub fn load_messages<'a>(&mut self, cid: &ConvId, from: Option<PagingState<db::M>>, num: usize) -> Result<db::Page<Vec<data::Message<'a>>, db::M>, Error> {
        debug!(self.logger, "loading conversation messages"; "id" => cid.to_string());
        self.state.user.dbase.messages(cid, from, num).map_err(From::from)
    }

    pub fn load_contacts<'a>(&mut self) -> Result<Vec<(User<'a>, Connection)>, Error> {
        debug!(self.logger, "loading contacts");
        self.state.user.dbase.connections().map_err(From::from)
    }

    pub fn load_user<'a>(&mut self, id: &UserId) -> Result<Option<User<'a>>, Error> {
        debug!(self.logger, "loading user"; "id" => id.to_string());
        self.state.user.dbase.user(id).map_err(Error::Database)
    }

    pub fn load_user_icon(&mut self, u: &User) -> Result<Vec<u8>, Error> {
        debug!(self.logger, "loading user icon"; "id" => u.id.to_string());
        if let Some(ref i) = u.icon {
            self.state.user.assets.push(i.as_str());
            let file: Result<Option<File>, Error> = {
                debug!(self.logger, "reading asset (locally)"; "key" => i.as_str());
                if self.state.user.assets.exists() {
                    File::open(&self.state.user.assets).map(Some).map_err(From::from)
                } else {
                    Ok(None)
                }
            };
            self.state.user.assets.pop();
            let mut data = Vec::new();
            if let Some(mut f) = file? {
                f.read_to_end(&mut data)?;
            }
            Ok(data)
        } else {
            Ok(Vec::new())
        }
    }

    /// Store as new message in database.
    pub fn store_message(&mut self, cid: &ConvId, msg: &GenericMessage) -> Result<(), Error> {
        debug!(self.logger, "store message"; "conv" => cid.to_string(), "id" => msg.get_message_id());
        save_message(&self.state.user.dbase, cid, &self.me().id, &self.state.user.device.client.id, msg)
    }

    pub fn enqueue(&mut self, id: &[u8], p: &send::Params, msg: &GenericMessage) -> Result<(), Error> {
        debug!(self.logger, "enqueue"; "conv" => p.conv.to_string(), "msg" => str::from_utf8(id).unwrap_or("N/A"));
        enqueue_message(&self.state.user.dbase, id, p, msg)
    }

    /// Encrypt message for conversation members.
    pub fn prepare_message(&mut self, cid: &ConvId, msg: &GenericMessage) -> Result<send::Params, Error> {
        debug!(self.logger, "preparing message"; "conv" => cid.to_string(), "id" => msg.get_message_id());

        let mut params = send::Params::new(cid.clone(), self.state.user.device.client.id.acquire());
        let msg_bytes  = msg.write_to_bytes()?;

        let members = self.state.user.dbase.conversation(cid)?
            .map(|c| c.members)
            .unwrap_or(Vec::new());

        for m in &members {
            let clients = self.state.user.dbase.clients(m)?;
            for c in &clients {
                let sid = api::new_session_id(m, &c.id);
                if let Some(session) = self.state.user.device.cbox.session(&sid)? {
                    params.add(&session, m.clone(), c.id.acquire(), &msg_bytes)?
                }
            }
        }

        Ok(params)
    }
}

// Online state operations //////////////////////////////////////////////////

impl Actor<Online> {
    /// Transition to `Init` state.
    pub fn init(self) -> Actor<Init> {
        debug!(self.logger, "Online -> Init");
        Actor {
            logger: self.logger,
            config: self.config,
            tls:    self.tls,
            state:  Init(())
        }
    }

    /// Transition to `Connected` state.
    pub fn connected(self) -> Actor<Connected> {
        debug!(self.logger, "Online -> Connected");
        Actor {
            logger: self.logger,
            config: self.config,
            tls:    self.tls,
            state:  Connected { client: self.state.client }
        }
    }

    /// Transition to `Offline` state.
    pub fn offline(self) -> Actor<Offline> {
        debug!(self.logger, "Online -> Offline");
        Actor {
            logger: self.logger,
            config: self.config,
            tls:    self.tls,
            state:  Offline {
                user:  self.state.user,
                bcast: self.state.bcast
            }
        }
    }

    /// Our own user information.
    pub fn me(&self) -> &User<'static> {
        &self.state.user.user
    }

    /// Our own client information.
    pub fn client(&self) -> &data::Client<'static> {
        &self.state.user.device.client
    }

    /// Is the current client newly created?
    pub fn is_new_client(&self) -> bool {
        self.state.user.device.fresh
    }

    /// Create a new "inbox".
    ///
    /// I.e. a websocket which (when established) listens for
    /// new notifications from back-end.
    pub fn new_inbox(&mut self) -> Result<Inbox, Error> {
        let actor = self.clone()?;
        Ok(Inbox::new(&self.logger, actor))
    }

    pub fn asset_path(&mut self, k: &AssetKey) -> PathBuf {
        self.state.user.assets.join(k.as_str())
    }

    pub fn download_asset(&mut self, k: &AssetKey, t: Option<&AssetToken>) -> Result<(), Error> {
        debug!(self.logger, "downloading asset"; "key" => k.as_str());
        self.state.user.assets.push(k.as_str());
        if self.state.user.assets.exists() {
            debug!(self.logger, "asset already downloaded"; "key" => k.as_str());
            self.state.user.assets.pop();
            return Ok(())
        }
        let result = error::retry3x(|r: Option<React<()>>| {
            self.react(r)?;
            let creds = self.state.user.creds.lock().unwrap();
            let url = self.state.client.asset_url(k, t, &creds.token)?;
            debug!(self.logger, "fetching asset"; "url" => format!("{}", url));
            let dom = url.host_str().ok_or(Error::Message("missing host in asset url"))?;
            let (mut rpc, tkn) = self.state.client.prepare_download(&url, dom)?;
            if rpc.response().status() != 200 {
                return Err(io::Error::new(io::ErrorKind::NotFound, "asset not found").into())
            }
            let mut r = rpc.reader(tkn).map_err(|e| ClientError::Rpc(e) : ClientError<Void>)?;
            let temp  = TempDir::new_in(&self.config.data.root, "coax")?;
            let path  = temp.path().join(k.as_str());
            {
                let mut f = File::create(&path)?;
                io::copy(&mut r, &mut f)?;
            }
            fs::rename(&path, &self.state.user.assets)?;
            Ok(())
        });
        self.state.user.assets.pop();
        result
    }

    pub fn decrypt_asset(&mut self, k: &AssetKey, cksum: &[u8], key: &[u8]) -> Result<(), Error> {
        let input = {
            let mut c = io::Cursor::new(Vec::new());
            self.state.user.assets.push(k.as_str());
            let f = File::open(&self.state.user.assets);
            self.state.user.assets.pop();
            io::copy(&mut f?, &mut c)?;
            c.into_inner()
        };
        let mut h = Hasher::new(MessageDigest::sha256())?;
        h.update(&input)?;
        let sha256 = h.finish()?;
        if sha256 != cksum {
            return Err(Error::Message("asset checksum check failed"))
        }
        if input.len() < 16 || input.len() % 8 != 0 {
            return Err(Error::Message("encrypted asset data length invalid"))
        }
        let iv    = &input[0 .. 16];
        let data  = &input[16 .. input.len()];
        let plain = symm::decrypt(symm::Cipher::aes_256_cbc(), key, Some(iv), data)?;
        let temp  = TempDir::new_in(&self.config.data.root, "coax")?;
        let path  = temp.path().join(k.as_str());
        {
            let mut f = File::create(&path)?;
            io::copy(&mut plain.as_slice(), &mut f)?;
        }
        self.state.user.assets.push(k.as_str());
        let result = fs::rename(&path, &self.state.user.assets);
        self.state.user.assets.pop();
        result?;
        self.state.user.dbase.update_asset_status(k, AssetStatus::Local)?;
        Ok(())
    }

    pub fn load_user_icon(&mut self, u: &User) -> Result<Vec<u8>, Error> {
        debug!(&self.logger, "loading user icon"; "id" => u.id.to_string());
        if let Some(ref i) = u.icon {
            self.download_asset(i, None)?;
            let mut file = {
                self.state.user.assets.push(i.as_str());
                let f = File::open(&self.state.user.assets);
                self.state.user.assets.pop();
                f?
            };
            let mut data = Vec::new();
            file.read_to_end(&mut data)?;
            Ok(data)
        } else {
            Ok(Vec::new())
        }
    }

    /// Given some `UserId` return the corresponding user data.
    ///
    /// If the user is found in local storage and `allow_local` is `true`
    /// it is returned right away, otherwise we try to get the information
    /// from back-end and save it locally.
    pub fn resolve_user<'a>(&mut self, id: &UserId, allow_local: bool) -> Result<Option<User<'a>>, Error> {
        if allow_local {
            if let Some(usr) = self.state.user.dbase.user(id)? {
                if usr.deleted {
                    return Ok(None)
                } else {
                    return Ok(Some(usr))
                }
            }
        }
        let usr = error::retry3x(|r: Option<React<()>>| {
            self.react(r)?;
            let creds = self.state.user.creds.lock().unwrap();
            let usr = self.state.client.user(id, &creds.token)?;
            Ok(usr)
        })?;
        if let Some(u) = usr {
            self.state.user.dbase.insert_user(&u)?;
            if u.deleted == Some(true) {
                Ok(None)
            } else {
                Ok(Some(User::from_api(u)))
            }
        } else {
            warn!(self.logger, "user not found"; "id" => id.to_string());
            Ok(None)
        }
    }

    /// Given some `UserId` and `ClientId` return the corresponding client data.
    ///
    /// If the client is found in local storage it is returned right away,
    /// otherwise we try to get the information from back-end and save it locally.
    pub fn resolve_client<'a>(&mut self, uid: &UserId, cid: &ClientId) -> Result<Option<data::Client<'a>>, Error> {
        if let Some(clt) = self.state.user.dbase.client(uid, cid)? {
            return Ok(Some(clt))
        }
        let client = error::retry3x(|r: Option<React<()>>| {
            self.react(r)?;
            let creds = self.state.user.creds.lock().unwrap();
            let client = self.state.client.user_client(uid, cid, &creds.token)?;
            Ok(client)
        })?;
        if let Some(c) = client {
            self.state.user.dbase.insert_client(uid, &c)?;
            Ok(Some(data::Client::from_api(uid.clone(), c, false)))
        } else {
            warn!(self.logger, "client not found"; "user" => uid.to_string(), "id" => cid.as_str());
            Ok(None)
        }
    }

    /// Given some `UserId` return the corresponding clients.
    ///
    /// If the clients are found in local storage they are returned right away,
    /// otherwise we try to get the information from back-end and save it locally.
    pub fn resolve_clients<'a>(&mut self, uid: &UserId) -> Result<Vec<data::Client<'a>>, Error> {
        let clients = self.state.user.dbase.clients(uid)?;
        if !clients.is_empty() {
            return Ok(clients)
        }
        let clients = error::retry3x(|r: Option<React<()>>| {
            self.react(r)?;
            let creds = self.state.user.creds.lock().unwrap();
            let clients = self.state.client.user_clients(uid, &creds.token)?;
            Ok(clients)
        })?;
        self.state.user.dbase.insert_clients(uid, &clients)?;
        Ok(clients.into_iter().map(|c| data::Client::from_api(uid.clone(), c, false)).collect())
    }

    /// Given some conversation ID return the corresponding conversation data.
    ///
    /// If the conversation is found in local storage it is retured right away,
    /// otherwise we try to get the information from back-end and save it locally.
    pub fn resolve_conversation<'a>(&mut self, id: &ConvId) -> Result<Option<Conversation<'a>>, Error> {
        if let Some(c) = self.state.user.dbase.conversation(id)? {
            return Ok(Some(c))
        }

        enum LookupResult<'a> {
            Found(api::conv::Conversation<'a>),
            NotFound,
            PastMember
        }

        let conv = error::retry3x(|r: Option<React<()>>| {
            self.react(r)?;
            let creds = self.state.user.creds.lock().unwrap();
            if let Some(c) = self.state.client.conversation(id, &creds.token)? {
                if c.members.me.current {
                    Ok(LookupResult::Found(c))
                } else {
                    Ok(LookupResult::PastMember)
                }
            } else {
                Ok(LookupResult::NotFound)
            }
        })?;

        match conv {
            LookupResult::Found(mut c) => {
                self.resolve_user(&c.creator, true)?;
                c.members.others.retain(|m| m.current);
                let mut nobody_left = true;
                for m in &c.members.others {
                    if m.id == self.me().id {
                        continue
                    }
                    nobody_left &= self.resolve_user(&m.id, true)?.is_none()
                }
                if c.typ == ConvType::OneToOne && nobody_left {
                    info!(self.logger, "ignoring 1:1 conversation without peer"; "conv" => c.id.to_string());
                    return Ok(None)
                }
                let t = UTC::now();
                self.state.user.dbase.insert_conversation(&t, &c)?;
                Ok(Some(Conversation::from_api(t, c)))
            }
            LookupResult::NotFound => {
                info!(self.logger, "conversation not found"; "id" => id.to_string());
                Ok(None)
            }
            LookupResult::PastMember => {
                debug!(self.logger, "past member of conversation"; "id" => id.to_string());
                Ok(None)
            }
        }
    }

    /// Resolve all conversations (up to 1000).
    pub fn resolve_conversations(&mut self) -> Result<(), Error> {
        debug!(self.logger, "resolving conversations");
        let mut page  = self.conversation_ids(256, None)?;
        let mut total = page.value.len();
        loop {
            debug!(self.logger, "page of conversation ids"; "len" => page.value.len());
            for id in &page.value {
                self.resolve_conversation(id)?;
            }
            if !page.has_more || page.value.is_empty() || total > 1000 { // TODO
                break
            }
            page   = self.conversation_ids(256, page.value.last())?;
            total += page.value.len()
        }
        Ok(())
    }

    /// Resolve all user connections.
    pub fn resolve_user_connections(&mut self) -> Result<(), Error> {
        debug!(self.logger, "resolving user connections");
        let mut page = self.user_connections(256, None)?;
        loop {
            debug!(self.logger, "page of user connections"; "len" => page.value.len());
            for c in &page.value {
                if self.resolve_user(&c.to, false)?.is_some() {
                    self.state.user.dbase.insert_connection(c)?
                }
            }
            if !page.has_more || page.value.is_empty() {
                break
            }
            page = self.user_connections(256, page.value.last().map(|c| &c.to))?
        }
        Ok(())
    }

    /// Given some `UserId` return connection information.
    ///
    /// If the connection is found in local storage it is retured right away,
    /// otherwise we try to get the information from back-end and save it locally.
    pub fn resolve_connection(&mut self, to: &UserId) -> Result<Option<Connection>, Error> {
        if let Some(cu) = self.state.user.dbase.connection(to)? {
            return Ok(Some(cu.0))
        }
        let conn = error::retry3x(|r: Option<React<()>>| {
            self.react(r)?;
            let creds = self.state.user.creds.lock().unwrap();
            let conn = self.state.client.user_connection(to, &creds.token)?;
            Ok(conn)
        })?;
        if let Some(c) = conn {
            self.state.user.dbase.insert_connection(&c)?;
            Ok(Some(Connection::from_api(c)))
        } else {
            Ok(None)
        }
    }

    /// Create a new connection to the given user.
    pub fn new_connection<'a>(&mut self, to: &User, n: Name, msg: &str) -> Result<Connection, Error> {
        let params = user::connect::Params::new(to.id.clone(), n, msg).unwrap(); // TODO
        let conn = error::retry3x(|r: Option<React<()>>| {
            self.react(r)?;
            let creds = self.state.user.creds.lock().unwrap();
            let conn = self.state.client.user_connect(&params, &creds.token)?;
            Ok(conn)
        })?;
        self.state.user.dbase.insert_connection(&conn)?;
        Ok(self.state.user.dbase.connection(&conn.to)?.map(|cu| cu.0).unwrap())
    }

    /// Update the connection status to the given user.
    pub fn update_connection(&mut self, to: &UserId, s: ConnectStatus) -> Result<bool, Error> {
        let params  = user::connect::update::Params::new(to.clone(), s);
        let updated = error::retry3x(|r: Option<React<()>>| {
            self.react(r)?;
            let creds = self.state.user.creds.lock().unwrap();
            let updated = self.state.client.set_connect_status(&params, &creds.token)?;
            Ok(updated)
        })?;
        if updated {
            self.state.user.dbase.update_connection(to, s)?;
        }
        Ok(updated)
    }

    /// Create a new conversation with some users.
    pub fn new_conversation<'a>(&mut self, name: Name, add: &[UserId]) -> Result<Conversation<'a>, Error> {
        let mut p = api::conv::create::Params::new(add);
        p.set_name(name.replicate());
        let conv_id = error::retry3x(|r: Option<React<()>>| {
            self.react(r)?;
            let creds = self.state.user.creds.lock().unwrap();
            self.state.client.conversation_create(&p, &creds.token).map_err(From::from)
        })?;
        let me = api::conv::SelfMember::new(self.me().id.clone());
        let mut mm = api::conv::Members::new(me);
        for u in add {
            mm.add_member(api::conv::Member::new(u.clone(), None))
        }
        let mut c = api::conv::Conversation::new(conv_id, self.me().id.clone(), mm);
        c.set_name(name.acquire());
        let t = UTC::now();
        self.state.user.dbase.insert_conversation(&t, &c)?;
        Ok(Conversation::from_api(t, c))
    }

    /// Get all conversation IDs.
    pub fn conversation_ids(&mut self, n: usize, c: Option<&ConvId>) -> Result<api::Page<Vec<ConvId>>, Error> {
        debug!(self.logger, "lookup conversation ids");
        error::retry3x(|r: Option<React<()>>| {
            self.react(r)?;
            let creds = self.state.user.creds.lock().unwrap();
            self.state.client.conversations(n, c, &creds.token).map_err(From::from)
        })
    }

    /// Get all user connections.
    pub fn user_connections<'a>(&mut self, n: usize, u: Option<&UserId>) -> Result<api::Page<Vec<UserConnection<'a>>>, Error> {
        debug!(self.logger, "lookup user connections");
        error::retry3x(|r: Option<React<()>>| {
            self.react(r)?;
            let creds = self.state.user.creds.lock().unwrap();
            self.state.client.user_connections(n, u, &creds.token).map_err(From::from)
        })
    }

    /// Encrypt message for conversation members.
    pub fn prepare_message(&mut self, cid: &ConvId, msg: &GenericMessage) -> Result<send::Params, Error> {
        debug!(self.logger, "preparing message"; "conv" => cid.to_string(), "id" => msg.get_message_id());
        let conv =
            if let Some(c) = self.resolve_conversation(cid)? {
                c
            } else {
                warn!(self.logger, "conversation does not exist"; "id" => cid.to_string());
                return Err(Error::Message("conversation does not exist"))
            };

        let mut params = send::Params::new(cid.clone(), self.state.user.device.client.id.acquire());
        let msg_bytes  = msg.write_to_bytes()?;

        for m in &conv.members {
            let clients = self.state.user.dbase.clients(m)?;
            for c in &clients {
                let sid = api::new_session_id(m, &c.id);
                if let Some(session) = self.state.user.device.cbox.session(&sid)? {
                    params.add(&session, m.clone(), c.id.acquire(), &msg_bytes)?
                } else {
                    // init session
                }
            }
        }

        Ok(params)
    }

    /// Store as new message in database.
    pub fn store_message(&mut self, cid: &ConvId, msg: &GenericMessage) -> Result<(), Error> {
        debug!(self.logger, "store message"; "conv" => cid.to_string(), "id" => msg.get_message_id());
        save_message(&self.state.user.dbase, cid, &self.me().id, &self.state.user.device.client.id, msg)
    }

    pub fn enqueue(&mut self, id: &[u8], p: &send::Params, msg: &GenericMessage) -> Result<(), Error> {
        debug!(self.logger, "enqueue"; "conv" => p.conv.to_string(), "msg" => str::from_utf8(id).unwrap_or("N/A"));
        enqueue_message(&self.state.user.dbase, id, p, msg)
    }

    pub fn dequeue(&mut self, id: &[u8], conv: &ConvId) -> Result<(), Error> {
        debug!(self.logger, "dequeue"; "conv" => conv.to_string(), "msg" => str::from_utf8(id).unwrap_or("N/A"));
        self.state.user.dbase.dequeue(id, conv)?;
        Ok(())
    }

    pub fn queue(&mut self, from: Option<PagingState<db::Q>>, num: usize) -> Result<db::Page<Vec<QueueItem>, db::Q>, Error> {
        self.state.user.dbase.queue_items(from, num).map_err(From::from)
    }

    pub fn resend(&mut self) -> Result<(), Error> {
        debug!(self.logger, "re-send queued items");
        let page_size = 10;
        let mut p = self.queue(None, page_size)?;
        loop {
            let n = p.data.len();
            debug!(self.logger, "{} item(s) read from queue (page size = {})", n, page_size);
            for q in p.data {
                match q.data {
                    QueueItemData::Msg { data, mesg } => {
                        if let Ok(g) = protobuf::parse_from_bytes(&mesg) {
                            let mut d = Decoder::default(ReadIter::new(Cursor::new(data)));
                            if let Ok(nm) = d.from_json() {
                                let mut p = send::Params { conv: q.conv, message: nm };
                                let dtime = self.send_message(&mut p, &g, Delivery::Persistent)?;
                                self.dequeue(&q.id, &p.conv)?;
                                if let Ok(msgid) = String::from_utf8(q.id) {
                                    let pkg = Pkg::MessageUpdate(p.conv.clone(), msgid, dtime, MessageStatus::Sent);
                                    self.state.bcast.send(pkg).unwrap()
                                }
                            } else {
                                error!(self.logger, "failed to parse queued message json";
                                    "conv" => q.conv.to_string(),
                                    "id"   => str::from_utf8(&q.id).unwrap_or("N/A"))
                            }
                        } else {
                            error!(self.logger, "failed to parse queued message protobuf";
                                "conv" => q.conv.to_string(),
                                "id"   => str::from_utf8(&q.id).unwrap_or("N/A"))
                        }
                    }
                }
            }
            if n != page_size {
                break
            }
            p = self.queue(Some(p.state), page_size)?
        }
        Ok(())
    }

    /// Send a message to some conversation.
    pub fn send_message(&mut self, params: &mut send::Params, msg: &GenericMessage, del: Delivery) -> Result<DateTime<UTC>, Error> {
        debug!(self.logger, "sending message"; "conv" => params.conv.to_string(), "id" => msg.get_message_id());
        let on_error = |_, e| {
            if error::is_unauthorised(&e) {
                return React::Renew
            }
            if error::can_retry(&e) {
                return React::Retry
            }
            if let Error::MsgSend(ClientError::Error(send::Error::Mismatch(cm))) = e {
                return React::Other(cm)
            }
            React::Abort(e)
        };

        let msg_bytes = msg.write_to_bytes()?;
        let mismatch  = error::retry(3, Duration::from_secs(3), on_error, |r| {
            if let Some(React::Other(cm)) = r {
                let pkm = self.on_mismatch(cm)?;
                for (u, cc) in pkm {
                    for (c, s) in cc {
                        params.add(&s, u.clone(), c.acquire(), &msg_bytes)?
                    }
                }
            } else {
                self.react(r)?
            }
            let creds = self.state.user.creds.lock().unwrap();
            self.state.client.send(&params, &creds.token).map_err(From::from)
        })?;

        if del == Delivery::Persistent {
            self.state.user.dbase.update_message_status(&params.conv, msg.get_message_id(), MessageStatus::Sent)?;
            self.state.user.dbase.update_message_time(&params.conv, msg.get_message_id(), &mismatch.time)?;
            self.state.user.dbase.update_conv_time(&params.conv, mismatch.time.timestamp())?;
        }

        Ok(mismatch.time)
    }

    fn send_message_confirmations(&mut self, to_confirm: HashMap<ConvId, Builder<Confirm>>) {
        debug!(self.logger, "sending message confirmations"; "conversations" => to_confirm.len());
        for (c, b) in to_confirm {
            let msg = b.finish();
            let res = self.prepare_message(&c, &msg)
                .and_then(|mut p| self.send_message(&mut p, &msg, Delivery::OneShot));
            if let Err(e) = res {
                error!(self.logger, "error sending confirmation message"; "error" => format!("{:?}", e))
            }
        }
    }

    pub fn load_conversations<'a>(&mut self, from: Option<PagingState<db::C>>, num: usize) -> Result<db::Page<Vec<Conversation<'a>>, db::C>, Error> {
        debug!(self.logger, "loading conversations from database");
        self.state.user.dbase.conversations(from, num).map_err(From::from)
    }

    pub fn load_messages<'a>(&mut self, cid: &ConvId, from: Option<PagingState<db::M>>, num: usize) -> Result<db::Page<Vec<data::Message<'a>>, db::M>, Error> {
        debug!(self.logger, "loading conversation messages"; "id" => cid.to_string());
        self.state.user.dbase.messages(cid, from, num).map_err(From::from)
    }

    pub fn load_contacts<'a>(&mut self) -> Result<Vec<(User<'a>, Connection)>, Error> {
        debug!(self.logger, "loading contacts");
        self.state.user.dbase.connections().map_err(From::from)
    }

    /// Check for new notifications at back-end.
    pub fn notifications(&mut self, always: bool) -> Result<bool, Error> {
        let mut last_id = self.state.user.dbase.last_notification()?;
        debug!(self.logger, "last notification"; "id" => last_id.as_ref().map(|i| i.to_string()));
        let mut client = error::retry3x(|r: Option<React<()>>| {
            self.react(r)?;
            self.connect()
        })?;

        if self.is_new_client() && !always {
            let last_id = error::retry3x(|r: Option<React<()>>| {
                self.react(r)?;
                let creds = self.state.user.creds.lock().unwrap();
                self.state.client.notifications_last(&creds.token).map_err(From::from)
            })?;
            if let Some(ref id) = last_id {
                self.state.user.dbase.set_last_notification(id)?
            }
            return Ok(false)
        }

        let mut to_confirm = HashMap::new(); // TODO: Limit size

        let has_more = error::retry3x(|r: Option<React<()>>| {
            self.react(r)?;
            let more;
            let token;
            {
                let mut reader = {
                    let mut p = events::get::Params::new(self.state.user.device.client.id.replicate());
                    last_id.as_ref().map(|x| {
                        p.set_start(x.clone())
                    });
                    let creds = self.state.user.creds.lock().unwrap();
                    client.notifications_reader(&p, &creds.token)?
                };
                {
                    let mut iter = events::get::Iter::from_read(&self.logger, &mut reader)?;
                    while let Some(item) = iter.next() {
                        match item {
                            Ok(n) => {
                                last_id = Some(n.id.clone());
                                self.on_notification(n, Some(&mut to_confirm))?
                            }
                            Err(e) => error!(self.logger, "failed to parse notification";
                                "prev"  => last_id.as_ref().map(|id| id.to_string()).unwrap_or("N/A".into()),
                                "error" => format!("{:?}", e))
                        }
                    }
                    more = iter.has_more()?;
                }
                token = reader.into()?;
            }
            client.reset(token);
            Ok(more)
        })?;
        if let Some(ref id) = last_id {
            self.state.user.dbase.set_last_notification(id)?
        }
        self.send_message_confirmations(to_confirm);
        Ok(has_more)
    }

    fn on_notification(&mut self, n: Notification<'static>, mut to_confirm: Option<&mut HashMap<ConvId, Builder<Confirm>>>) -> Result<(), Error> {
        debug!(self.logger, "notification"; "id" => n.id.to_string());
        if self.state.user.dbase.has_notification(&n.id)? {
            debug!(self.logger, "notification already seen"; "id" => n.id.to_string());
            return Ok(())
        }
        for e in n.events {
            match e {
                Ok(Event::User(ety, e)) => {
                    debug!(self.logger, "event"; "type" => format!("{:?}", ety));
                    match ety {
                        EventType::UserClientAdd  => self.on_client_add(e)?,
                        EventType::UserConnection => self.on_user_connection(e)?,
                        _                         => {}
                    }
                }
                Ok(Event::Conv(ety, e)) => {
                    debug!(self.logger, "event"; "type" => format!("{:?}", ety));
                    match ety {
                        EventType::ConvCreate      => self.on_conv_create(e)?,
                        EventType::ConvMemberJoin  => self.on_members_change(e)?,
                        EventType::ConvMemberLeave => self.on_members_change(e)?,
                        EventType::ConvMessageAdd  => self.on_message_add(e, &mut to_confirm)?,
                        _                          => {}
                    }
                }
                Ok(Event::Unknown(e)) => {
                    warn!(self.logger, "unknown event: {:?}", e)
                }
                Err(e) => {
                    error!(self.logger, "could not parse notification event";
                           "notification" => n.id.to_string(),
                           "error"        => format!("{:?}", e))
                }
            }
        }
        self.state.user.dbase.insert_notification(&n.id)?;
        Ok(())
    }

    fn on_mismatch(&mut self, cm: ClientMismatch) -> Result<Vec<(UserId, Vec<(ClientId<'static>, CBoxSession<FileStore>)>)>, Error> {
        debug!(self.logger, "client mismatch"; "clients" => format!("{:?}", cm));
        let prekeys = error::retry3x(|r: Option<React<()>>| {
            self.react(r)?;
            let creds = self.state.user.creds.lock().unwrap();
            let pkeys = self.state.client.prekeys(&cm.missing, &creds.token)?;
            Ok(pkeys)
        })?;
        let mut uvec = Vec::with_capacity(prekeys.value.len());
        for (u, cc) in prekeys.value {
            let mut cvec = Vec::with_capacity(cc.len());
            for (c, k) in cc {
                self.resolve_client(&u, &c)?;
                if let Some(pk) = k {
                    debug!(self.logger, "new cbox session from prekey";
                        "user"   => u.to_string(),
                        "client" => c.as_str());
                    let sid = api::new_session_id(&u, &c);
                    let s = self.state.user.device.cbox.session_from_prekey(sid, pk.key)?;
                    cvec.push((c, s))
                }
            }
            uvec.push((u, cvec))
        }
        Ok(uvec)
    }

    fn on_client_add(&self, e: UserEvent) -> Result<(), Error> {
        if let UserEvent::AddClient(c) = e {
            debug!(self.logger, "adding client"; "id" => c.id.as_str());
            if c.id != self.state.user.device.client.id {
                self.state.user.dbase.insert_client(&self.me().id, &c)?;
            }
        }
        Ok(())
    }

    fn on_user_connection(&mut self, e: UserEvent<'static>) -> Result<(), Error> {
        if let UserEvent::Connect(_, c) = e {
            debug!(self.logger, "user connection"; "to" => c.to.to_string(), "status" => c.status.as_str());
            if let Some(usr) = self.resolve_user(&c.to, true)? {
                self.state.user.dbase.insert_connection(&c)?;
                self.state.bcast.send(Pkg::Contact(usr, Connection::from_api(c))).unwrap()
            } else {
                warn!(self.logger, "user connection peer not found"; "user" => c.to.to_string())
            }
        }
        Ok(())
    }

    fn on_conv_create(&mut self, e: ConvEvent<'static>) -> Result<(), Error> {
        let mut conv =
            if let ConvEventData::Create(conv) = e.data {
                conv
            } else {
                return Ok(())
            };

        debug!(self.logger, "create conversation";
               "id"      => conv.id.to_string(),
               "creator" => conv.creator.to_string(),
               "type"    => format!("{:?}", conv.typ));

        if self.resolve_user(&conv.creator, true)?.is_none() {
            warn!(self.logger, "conversation creator not found"; "user" => conv.creator.to_string());
            return Ok(())
        }

        let mut v = Vec::with_capacity(conv.members.others.len());
        for m in conv.members.others {
            if self.resolve_user(&m.id, true)?.is_some() {
                v.push(m)
            } else {
                warn!(self.logger, "conversation member not found"; "user" => m.id.to_string())
            }
        }
        conv.members.others = v;

        self.state.user.dbase.insert_conversation(&e.time, &conv)?;
        let pkg = Pkg::Conversation(Conversation::from_api(e.time, conv));
        self.state.bcast.send(pkg).unwrap();

        Ok(())
    }

    fn on_members_change(&mut self, e: ConvEvent<'static>) -> Result<(), Error> {
        let (users, status) = match e.data {
            ConvEventData::Join(users)  => (users, ConvStatus::Current),
            ConvEventData::Leave(users) => (users, ConvStatus::Previous),
            _                           => return Ok(())
        };

        debug!(self.logger, "conversation members change";
               "type"  => if status == ConvStatus::Current { "join" } else { "leave" },
               "id"    => e.id.to_string(),
               "from"  => e.from.to_string(),
               "users" => format!("{:?}", users.iter().map(UserId::as_uuid).collect::<Vec<_>>()));

        let sender =
            if let Some(usr) = self.resolve_user(&e.from, true)? {
                usr
            } else {
                warn!(self.logger, "sending user not found"; "id" => e.from.to_string());
                return Ok(())
            };

        if self.resolve_conversation(&e.id)?.is_none() {
            warn!(self.logger, "conversation not found"; "id" => e.id.to_string());
            return Ok(())
        }

        let mut members = Vec::new();
        for u in users.as_ref() {
            if *u == self.me().id {
                members.push(self.me().clone());
                self.state.user.dbase.update_conv_status(&e.id, status)?;
                continue
            }
            if let Some(usr) = self.resolve_user(&u, true)? {
                members.push(usr)
            } else {
                warn!(self.logger, "unknown member"; "conv" => e.id.to_string(), "user" => u.to_string())
            }
        }

        {
            let mids: Vec<&UserId> = members.iter().map(|m| &m.id).collect();
            match status {
                ConvStatus::Current  => self.state.user.dbase.insert_members(&e.id, &mids)?,
                ConvStatus::Previous => self.state.user.dbase.remove_members(&e.id, &mids)?
            }
        }

        for m in &members {
            let id  = random_uuid().to_string();
            let msg = match status {
                ConvStatus::Current  => NewMessage::joined(&id, &e.id, &e.time, &e.from, &m.id),
                ConvStatus::Previous => NewMessage::left(&id, &e.id, &e.time, &e.from, &m.id)
            };
            self.state.user.dbase.insert_message(&msg)?
        }

        self.state.bcast.send(Pkg::MembersChange(status, e.time, e.id, members, sender)).unwrap();

        Ok(())
    }

    fn on_message_add(&mut self, e: ConvEvent<'static>, to_confirm: &mut Option<&mut HashMap<ConvId, Builder<Confirm>>>) -> Result<(), Error> {
        let msg =
            if let ConvEventData::Encrypted(msg) = e.data {
                msg
            } else {
                return Ok(())
            };

        debug!(self.logger, "new message"; "conversation" => e.id.to_string());

        let usr =
            if let Some(usr) = self.resolve_user(&e.from, true)? {
                usr
            } else {
                warn!(self.logger, "unknown sender"; "user" => e.from.to_string());
                return Ok(())
            };

        let conv =
            if let Some(conv) = self.resolve_conversation(&e.id)? {
                conv
            } else {
                warn!(self.logger, "message for unknown conversation";
                      "id"   => e.id.to_string(),
                      "user" => e.from.to_string());
                return Ok(())
            };

        if self.resolve_client(&usr.id, &msg.sender)?.is_none() {
            warn!(self.logger, "unknown sender client";
                  "user"   => e.from.to_string(),
                  "client" => msg.sender.as_str())
        }


        match msg.decrypt(&e.from, &self.state.user.device.cbox) {
            Ok((session, mut plain)) => {
                let mid = plain.text.take_message_id();
                debug!(self.logger, "message"; "id" => mid);
                if plain.text.has_text() {
                    debug!(self.logger, "text message");
                    let text = plain.text.take_text().take_content();
                    {
                        let mut nmsg = NewMessage::text(&mid, &e.id, &e.time, &e.from, &msg.sender, &text);
                        nmsg.set_status(MessageStatus::Received);
                        self.state.user.dbase.insert_message(&nmsg)?;
                    }
                    self.state.user.dbase.update_conv_time(&e.id, e.time.timestamp())?;
                    if conv.ctype == ConvType::OneToOne {
                        if let Some(ref mut confirm) = *to_confirm {
                            match confirm.entry(e.id.clone()) {
                                Entry::Occupied(mut e) => {
                                    e.get_mut().add_delivered(mid.as_ref());
                                }
                                Entry::Vacant(e) => {
                                    e.insert(Builder::new().delivered(mid.as_ref()));
                                }
                            }
                        }
                    }
                    let message = data::Message {
                        id:     mid,
                        conv:   e.id,
                        time:   e.time,
                        user:   usr,
                        client: Some(msg.sender),
                        status: MessageStatus::Received,
                        data:   MessageData::Text(text)
                    };
                    self.state.bcast.send(Pkg::Message(message)).unwrap()
                } else if plain.text.has_confirmation() {
                    debug!(self.logger, "confirmation message");
                    let mut con = plain.text.take_confirmation();
                    let mut ids = con.take_more_message_ids();
                    ids.push(con.take_first_message_id());
                    for id in ids.into_iter() {
                        if let Some(cid) = self.state.user.dbase.message_conversation_id(&id)? {
                            self.state.user.dbase.update_message_status(&cid, &id, MessageStatus::Delivered)?;
                            self.state.bcast.send(Pkg::MessageUpdate(cid, id, e.time, MessageStatus::Delivered)).unwrap()
                        }
                    }
                } else if plain.text.has_asset() {
                    debug!(self.logger, "asset");
                    let mut asset = plain.text.take_asset();
                    if !asset.has_uploaded() || !asset.has_original() {
                        debug!(self.logger, "asset not uploaded or not original"; "msg" => mid);
                        return Ok(())
                    }
                    if !asset.get_uploaded().has_asset_id() {
                        debug!(self.logger, "asset without key"; "msg" => mid);
                        return Ok(())
                    }
                    let     orig  = asset.take_original();
                    let mut data  = asset.take_uploaded();
                    let asset_key = AssetKey::new(data.take_asset_id());
                    let asset_tkn =
                        if data.has_asset_token() {
                            Some(AssetToken::new(data.take_asset_token()))
                        } else {
                            None
                        };
                    debug!(self.logger, "asset data";
                           "id"        => asset_key.as_str(),
                           "mime-type" => orig.get_mime_type(),
                           "size"      => orig.get_size());
                    if orig.has_image() {
                        {
                            let mut nast = NewAsset::new(&asset_key, AssetType::Image, AssetStatus::Remote, data.get_otr_key(), data.get_sha256());
                            if let Some(ref at) = asset_tkn {
                                nast.set_token(at)
                            }
                            self.state.user.dbase.insert_asset(&nast)?;

                            let mut nmsg = NewMessage::asset(&mid, &e.id, &e.time, &e.from, &msg.sender, &asset_key);
                            nmsg.set_status(MessageStatus::Received);
                            self.state.user.dbase.insert_message(&nmsg)?;
                        }
                        let ast = data::Asset {
                            id:     asset_key,
                            atype:  AssetType::Image,
                            status: AssetStatus::Remote,
                            token:  asset_tkn,
                            key:    data.take_otr_key(),
                            cksum:  data.take_sha256()
                        };
                        let msg = data::Message {
                            id:     mid,
                            conv:   e.id,
                            time:   e.time,
                            user:   usr,
                            client: Some(msg.sender),
                            status: MessageStatus::Received,
                            data:   MessageData::Asset(ast)
                        };
                        self.state.bcast.send(Pkg::Message(msg)).unwrap()
                    }
                } else {
                    error!(self.logger, "unsupported message type"); // TODO
                }
                session.save()?
            }
            Err(err) => {
                error!(self.logger, "failed to decrypt";
                    "conv"   => e.id.to_string(),
                    "time"   => format!("{}", e.time),
                    "from"   => e.from.to_string(),
                    "sender" => msg.sender.as_str(),
                    "error"  => format!("{}", err))
            }
        }
        Ok(())
    }

    pub fn renew_access(&mut self) -> Result<(), Error> {
        let mut creds = self.state.user.creds.lock().unwrap();
        let newcreds = self.state.client.access_renew(&creds.cookie, Some(&creds.token))?; // TODO: Retry
        if let Some(c) = newcreds.cookie {
            set_cookie(&self.state.user.dbase, &c)?;
            creds.cookie = c;
        }
        creds.token = newcreds.token;
        Ok(())
    }

    fn react<R>(&mut self, r: Option<React<R>>) -> Result<(), Error> {
        match r {
            Some(React::Renew) => self.renew_access(),
            Some(React::Retry) => {
                self.state.client.reconnect()?;
                if self.state.user.creds.lock().unwrap().token.is_expired() {
                    self.renew_access()?
                }
                Ok(())
            }
            _ => Ok(())
        }
    }

    /// A cloned `Actor` has its own database connection and API client
    /// but shares almost everything else with its original.
    pub fn clone(&mut self) -> Result<Actor<Online>, Error> {
        let self_id = self.me().id.clone();
        let dbase   = self.open_database(&self_id)?;
        let client  = error::retry3x(|r: Option<React<()>>| {
            self.react(r)?;
            self.connect()
        })?;
        Ok(Actor {
            logger: self.logger.clone(),
            config: self.config.clone(),
            tls:    self.tls.clone(),
            state:  Online {
                user: UserData {
                    user:   self.state.user.user.clone(),
                    dbase:  dbase,
                    creds:  self.state.user.creds.clone(),
                    device: self.state.user.device.clone(),
                    assets: self.state.user.assets.clone()
                },
                client: client,
                bcast:  self.state.bcast.clone()
            }
        })
    }
}

// Generic Actor operations /////////////////////////////////////////////////

impl<S> Actor<S> {
    /// View configuration.
    pub fn config(&self) -> &config::Main {
        &self.config
    }

    /// Connect to configured `/host/url`.
    pub fn connect(&self) -> Result<Client<'static>, Error> {
        let url = Url::parse(&self.config.host.url)?;
        let mut client =
            if let Some(dom) = url.domain() {
                Client::connect(&self.logger, url.clone(), dom, self.tls.clone())?
            } else {
                return Err(Error::Message("/host/url has no domain"))
            };
        client.set_read_timeout(Some(Duration::from_secs(10)))?;
        client.set_write_timeout(Some(Duration::from_secs(10)))?;
        Ok(client)
    }

    fn open_cryptobox(&self, uid: &UserId) -> Result<CBox<FileStore>, Error> {
        let mut root = PathBuf::from(&self.config.data.root);
        root.push(uid.to_string());
        root.push("cryptobox");
        if !root.exists() {
            DirBuilder::new().create(&root)?;
        }
        let store = FileStore::new(&root)?;
        CBox::open(store).map_err(From::from)
    }

    fn database_path(&self, uid: &UserId) -> Result<PathBuf, Error> {
        let mut p = PathBuf::from(&self.config.data.root);
        p.push(uid.to_string());
        if !p.exists() {
            DirBuilder::new().create(&p)?;
        }
        p.push("main.db");
        Ok(p)
    }

    fn open_database(&self, uid: &UserId) -> Result<Database, Error> {
        let path  = self.database_path(uid)?;
        let dbase = Database::open(&self.logger, &path)?;
        Ok(dbase)
    }

    fn mk_assets_dir(&self, uid: &UserId) -> Result<PathBuf, Error> {
        let mut root = PathBuf::from(&self.config.data.root);
        root.push(uid.to_string());
        root.push("assets");
        if !root.exists() {
            DirBuilder::new().create(&root)?;
        }
        Ok(root)
    }
}

const MY_CLIENT_ID: &'static str = "my-client-id";
const ACCESS_TOKEN: &'static str = "access-token";
const USER_COOKIE: &'static str = "user-cookie";

fn my_client_id<'a>(db: &Database) -> Result<Option<ClientId<'a>>, Error> {
	if let Some(blob) = db.var(MY_CLIENT_ID)? {
		Ok(String::from_utf8(blob).ok().map(|s| ClientId::new(s)))
	} else {
		Ok(None)
	}
}

fn set_my_client_id(db: &Database, c: &ClientId) -> Result<(), Error> {
    db.set_var(MY_CLIENT_ID, c.as_str().as_bytes())?;
    Ok(())
}

fn cookie(db: &Database) -> Result<Option<Cookie<'static>>, Error> {
    if let Some(blob) = db.var(USER_COOKIE)? {
		Ok(String::from_utf8(blob).ok().and_then(|s| Cookie::parse(s).ok()))
	} else {
		Ok(None)
	}
}

fn set_cookie(db: &Database, c: &Cookie) -> Result<(), Error> {
    db.set_var(USER_COOKIE, format!("{}", c).as_bytes())?;
    Ok(())
}

fn access_token(db: &Database) -> Result<Option<String>, Error> {
	if let Some(blob) = db.var(ACCESS_TOKEN)? {
		Ok(String::from_utf8(blob).ok())
	} else {
		Ok(None)
	}
}

fn set_access_token(db: &Database, t: &AccessToken) -> Result<(), Error> {
    db.set_var(ACCESS_TOKEN, t.token.as_ref().as_bytes())?;
    Ok(())
}

fn save_message(db: &Database, cid: &ConvId, uid: &UserId, clt: &ClientId, msg: &GenericMessage) -> Result<(), Error> {
    let time = UTC::now();
    let text = msg.get_text().get_content(); // TODO
    let new_msg = NewMessage::text(msg.get_message_id(), cid, &time, uid, clt, text);
    db.insert_message(&new_msg)?;
    Ok(())
}

fn enqueue_message(db: &Database, id: &[u8], p: &send::Params, msg: &GenericMessage) -> Result<(), Error> {
    use protobuf::Message;
    let     m = msg.write_to_bytes()?;
    let mut e = Encoder::new(Cursor::new(Vec::new()));
    p.message.encode(&mut e)?;
    db.enqueue_message(id, &p.conv, e.into_writer().get_ref(), &m)?;
    Ok(())
}

/// An `Inbox` awaits new notifications from back-end.
pub struct Inbox {
    logger: Logger,
    actor:  Actor<Online>
}

impl Inbox {
    fn new(g: &Logger, a: Actor<Online>) -> Inbox {
        Inbox {
            logger: g.new(o!("context" => "Inbox")),
            actor:  a,
        }
    }

    /// Establish websocket connection.
    pub fn connect(&mut self) -> Result<Listener<'static, TlsStream>, Error> {
        let mut url = Url::parse(&self.actor.config().host.websocket)?;
        url.query_pairs_mut().append_pair("client", self.actor.client().id.as_str());
        if let Some(dom) = url.domain() {
            error::retry3x(|r: Option<React<()>>| {
                self.actor.react(r)?;
                let c = self.actor.state.user.creds.lock().unwrap();
                let w = Listener::open_wss(&self.logger, url.clone(), dom, self.actor.tls.clone(), &c.token)?;
                Ok(w)
            })
        } else {
            Err(Error::Message("/host/websocket has no domain"))
        }
    }

    /// Begin listening for notifications in a separate thread.
    pub fn fork(mut self, wsock: Listener<'static, TlsStream>) -> JoinHandle<()> {
        thread::spawn(move || self.start(wsock))
    }

    /// Begin listening for notifications.
    ///
    /// NB: This method never terminates!
    pub fn start(&mut self, mut wsock: Listener<'static, TlsStream>) -> ! {
        loop {
            match wsock.listen() : Result<Notification, ClientError<coax_client::error::Void>> {
                Ok(n) => {
                    debug!(self.logger, "received"; "id" => n.id.to_string());
                    if let Err(e) = self.actor.on_notification(n, None) {
                        error!(self.logger, "error decrypting notification: {}", e)
                    }
                }
                Err(e) => {
                    error!(self.logger, "error on receive: {}", e);
                    self.actor.state.bcast.send(Pkg::Disconnected).unwrap();
                    let mut d = 1;
                    loop {
                        debug!(self.logger, "reconnecting websocket ...");
                        let res = {
                            let t = self.actor.state.user.creds.lock().unwrap();
                            wsock.reconnect_wss(&t.token)
                        };
                        match res {
                            Ok(()) => {
                                debug!(self.logger, "websocket reconnected");
                                self.actor.state.bcast.send(Pkg::Connected).unwrap();
                                break
                            }
                            Err(ClientError::WebSocket(WsError::Handshake(401, _))) => {
                                debug!(self.logger, "handshake unauthorised, renewing credentials ...");
                                if let Err(e) = self.actor.state.client.reconnect().map_err(From::from).and(self.actor.renew_access()) {
                                    error!(self.logger, "error renewing access: {}", e)
                                } else {
                                    continue
                                }
                            }
                            Err(e) => error!(self.logger, "websocket reconnect error: {}", e)
                        }
                        if d < 30 { d += 1 }
                        thread::sleep(Duration::from_secs(d))
                    }
                }
            }
        }
    }
}

