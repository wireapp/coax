use std::io::Cursor;
use std::str::FromStr;

use bytes::{BufMut, BytesMut};
use coax_api::conv::{self, Conversation};
use coax_api::client::{self, Client as ApiClient, User2Clients, ClientMismatch};
use coax_api::events::{self, ConvEvent, EventType, Notification};
use coax_api::message;
use coax_api::prekeys::{ClientPreKey, ClientPreKeys, PreKeyMap};
use coax_api::token::{self, AccessToken};
use coax_api::types::{UserId, ClientId, ConvId, NotifId, ApiError, Page};
use coax_api::user::{self, User, AssetKey, AssetToken};
use error::{Error, Void};
use futures::{Future, Stream};
use futures::future::{self, Either, FutureResult};
use hyper::{self, Request, Response, Body, Uri, Method, StatusCode};
use hyper::client::HttpConnector;
use hyper::error::UriError;
use hyper::header::{Authorization, Bearer, ContentType, Cookie, Accept, Location};
use hyper_tls::HttpsConnector;
use json::{ToJson, FromJson};
use json::decoder::{Decoder, DecodeError, ReadIter};
use json::encoder::{Encoder, EncodeError};
use native_tls::TlsConnector;
use slog::Logger;
use tokio_core::reactor::Handle;
use url::Url;

mod header {
    header! {
        (AssetToken, "Asset-Token") => [String]
    }
}

macro_rules! with {
    ($($name:ident),+ => $f:expr) => {{
        $(let $name = $name.clone();)+
        $f
    }}
}

macro_rules! attempt {
    ($e: expr) => {
        match $e {
            Err(e) => return future::err(e.into()),
            Ok(x)  => x
        }
    }
}

macro_rules! handle_error {
    ($logger: ident, $ctx: expr, $res: expr, $bytes: expr) => {{
        let status  = $res.status();
        let is_json = $res.headers().get() == Some(&ContentType::json());
        if is_json {
            let f = json($res.body(), $bytes)
                .and_then(move |e: ApiError<'static>| {
                    error!($logger, $ctx; "status" => %status, "error" => ?e);
                    future::err(e.into())
                });
            Either::A(f)
        } else {
            let f = $res.body()
                .for_each(|_| future::ok(())) // drain body
                .then(move |_| {
                    error!($logger, $ctx; "status" => %status);
                    future::err(Error::Status(status))
                });
            Either::B(f)
        }
    }}
}

#[derive(Debug, Clone)]
pub struct Client {
    logger: Logger,
    base:   Url,
    bytes:  BytesMut,
    client: hyper::Client<HttpsConnector<HttpConnector>, Body>
}

impl Client {
    pub fn new(g: &Logger, base: Url, tls: TlsConnector, hdl: &Handle) -> Result<Client, Error<Void>> {
        debug!(g, "new client"; "url" => %base);
        Ok(Client {
            logger: g.new(o!("context" => "Client")),
            base:   base,
            bytes:  BytesMut::with_capacity(0x80000),
            client: hyper::Client::configure()
                .connector(HttpsConnector::from((HttpConnector::new(1, hdl), tls)))
                .build(hdl)
        })
    }

    /// Renew an `AccessToken` [`POST /access`].
    pub fn access_renew<'a>(&self, c: &Cookie, t: Option<&AccessToken>) -> impl Future<Item=token::Credentials<'static, Option<Cookie>>, Error=Error<token::renew::Error>> + 'a {
        info!(self.logger, "renewing access token");

        let cookie = c.clone();
        let logger = self.logger.clone();
        let client = self.client.clone();
        let bytes  = self.bytes.clone();

        let mut url = self.base.clone();
        url.set_path("/access");

        request(Method::Post, &url, t)
            .and_then(move |mut req| {
                req.headers_mut().set(cookie);
                client.request(req).from_err()
            })
            .and_then(move |res| {
                if res.status() == StatusCode::Ok && is_json(&res) {
                    let cookie = res.headers().get().cloned();
                    Either::A(json(res.body(), bytes).map(|token| token::Credentials::new(token, cookie)))
                } else {
                    Either::B(handle_error!(logger, "renew access token", res, bytes))
                }
            })
    }

    /// Login some user [`POST /login`].
    pub fn user_login<'a>(&self, p: user::login::Params<'a>) -> impl Future<Item=token::Credentials<'static, Cookie>, Error=Error<user::login::Error>> + 'a {
        info!(self.logger, "login"; "email" => ?p.email, "phone" => ?p.phone);

        let logger = self.logger.clone();
        let client = self.client.clone();
        let bytes  = self.bytes.clone();

        let mut url = self.base.clone();
        url.set_path("/login");
        url.set_query(Some("persist=true"));

        request(Method::Post, &url, None)
            .and_then(with!(bytes => move |req| request_body(bytes, req, p)))
            .and_then(move |req| client.request(req).from_err())
            .and_then(move |res| {
                if res.status() == StatusCode::Ok && is_json(&res) {
                    let future =
                        if let Some(cookie) = res.headers().get().cloned() {
                            Either::A(json(res.body(), bytes).map(|token| token::Credentials::new(token, cookie)))
                        } else {
                            Either::B(future::err(Error::Error(user::login::Error::MissingCookie)))
                        };
                    Either::A(future)
                } else {
                    Either::B(handle_error!(logger, "user login", res, bytes))
                }
            })
    }

    /// Logout some user [`POST /access/logout`].
    pub fn user_logout<'a>(&self, c: &Cookie, t: &AccessToken) -> impl Future<Item=(), Error=Error<Void>> + 'a {
        info!(self.logger, "logout");

        let cookie = c.clone();
        let logger = self.logger.clone();
        let client = self.client.clone();
        let bytes  = self.bytes.clone();

        let mut url = self.base.clone();
        url.set_path("/access/logout");

        request(Method::Post, &url, Some(t))
            .and_then(move |mut req| {
                req.headers_mut().set(cookie);
                client.request(req).from_err()
            })
            .and_then(move |res| {
                if res.status() == StatusCode::Ok {
                    Either::A(drain(res.body()))
                } else {
                    Either::B(handle_error!(logger, "user logout", res, bytes))
                }
            })
    }

    /// Register a new user [`POST /register`].
    pub fn user_register<'a>(&self, p: user::register::Params<'a>) -> impl Future<Item=User<'static>, Error=Error<user::register::Error>> + 'a {
        info!(self.logger, "registering user"; "email" => ?p.email, "phone" => ?p.phone);

        let logger = self.logger.clone();
        let client = self.client.clone();
        let bytes  = self.bytes.clone();

        let mut url = self.base.clone();
        url.set_path("/register");

        request(Method::Post, &url, None)
            .and_then(with!(bytes => move |req| request_body(bytes, req, p)))
            .and_then(move |req| client.request(req).from_err())
            .and_then(move |res| {
                if res.status() == StatusCode::Created && is_json(&res) {
                    Either::A(json(res.body(), bytes))
                } else {
                    Either::B(handle_error!(logger, "user registration", res, bytes))
                }
            })
    }

    pub fn set_connect_status<'a>(&self, p: user::connect::update::Params, t: &AccessToken) -> impl Future<Item=bool, Error=Error<user::connect::update::Error>> + 'a {
        info!(self.logger, "updating connect status to user"; "user" => %p.user);

        let logger = self.logger.clone();
        let client = self.client.clone();
        let bytes  = self.bytes.clone();

        let mut url = self.base.clone();
        url.set_path(&format!("/connections/{}", p.user));

        request(Method::Put, &url, Some(t))
            .and_then(with!(bytes => move |req| request_body(bytes, req, p)))
            .and_then(move |req| client.request(req).from_err())
            .and_then(move |res| {
                let status = res.status();
                match status {
                    StatusCode::Ok | StatusCode::NoContent => {
                        Either::A(drain(res.body()).map(move |()| status == StatusCode::Ok))
                    }
                    _ => Either::B(handle_error!(logger, "connection status update", res, bytes))
                }
            })
    }

    /// Send a connection request to some user [`POST /connections`].
    pub fn user_connect<'a>(&self, p: user::connect::Params<'a>, t: &AccessToken) -> impl Future<Item=user::Connection<'static>, Error=Error<user::connect::Error>> + 'a {
        info!(self.logger, "connecting to user"; "user" => %p.user);

        let logger = self.logger.clone();
        let client = self.client.clone();
        let bytes  = self.bytes.clone();

        let mut url = self.base.clone();
        url.set_path("/connections");

        request(Method::Post, &url, Some(t))
            .and_then(with!(bytes => move |req| request_body(bytes, req, p)))
            .and_then(move |req| client.request(req).from_err())
            .and_then(move |res| {
                match res.status() {
                    StatusCode::Ok | StatusCode::Created if is_json(&res) => {
                        Either::A(json(res.body(), bytes))
                    }
                    _ => Either::B(handle_error!(logger, "connecting user", res, bytes))
                }
            })
    }

    /// Lookup connection to user [`GET /connections/{user}`]
    pub fn user_connection<'a>(&self, to: &UserId, t: &AccessToken) -> impl Future<Item=Option<user::Connection<'static>>, Error=Error<Void>> + 'a {
        info!(self.logger, "lookup connection"; "id" => %to);

        let logger = self.logger.clone();
        let client = self.client.clone();
        let bytes  = self.bytes.clone();

        let mut url = self.base.clone();
        url.set_path(&format!("/connections/{}", to.to_string()));

        request(Method::Get, &url, Some(t))
            .and_then(move |req| client.request(req).from_err())
            .and_then(move |res| {
                match res.status() {
                    StatusCode::Ok if is_json(&res) => {
                        Either::A(Either::A(json(res.body(), bytes)))
                    }
                    StatusCode::NotFound => {
                        Either::A(Either::B(drain(res.body()).map(|()| None)))
                    }
                    _ => Either::B(handle_error!(logger, "lookup connection", res, bytes))
                }
            })
    }

    /// Lookup connections to user [`GET /connections`]
    pub fn user_connections<'a>(&self, len: usize, start: Option<&UserId>, t: &AccessToken) -> impl Future<Item=Page<Vec<user::Connection<'static>>>, Error=Error<Void>> + 'a {
        info!(self.logger, "listing user connections");

        let logger = self.logger.clone();
        let client = self.client.clone();
        let bytes  = self.bytes.clone();

        let mut url = self.base.clone();
        url.set_path("/connections");
        url.query_pairs_mut().append_pair("size", len.to_string().as_str());
        if let Some(s) = start {
            url.query_pairs_mut().append_pair("start", s.to_string().as_str());
        }

        request(Method::Get, &url, Some(t))
            .and_then(move |req| client.request(req).from_err())
            .and_then(move |res| {
                if res.status() == StatusCode::Ok && is_json(&res) {
                    Either::A(json(res.body(), bytes).map(|x: user::connect::get::Connections<'static>| x.0))
                } else {
                    Either::B(handle_error!(logger, "listing user connections", res, bytes))
                }
            })
    }

    /// Get own user profile [`GET /self`].
    pub fn self_user<'a>(&self, t: &AccessToken) -> impl Future<Item=User<'static>, Error=Error<Void>> + 'a {
        debug!(self.logger, "looking up self");

        let logger = self.logger.clone();
        let client = self.client.clone();
        let bytes  = self.bytes.clone();

        let mut url = self.base.clone();
        url.set_path("/self");

        request(Method::Get, &url, Some(t))
            .and_then(move |req| client.request(req).from_err())
            .and_then(move |res| {
                if res.status() == StatusCode::Ok && is_json(&res) {
                    Either::A(json(res.body(), bytes))
                } else {
                    Either::B(handle_error!(logger, "looking up self", res, bytes))
                }
            })
    }

    /// Get some user profile [`GET /users/{id}`].
    pub fn user<'a>(&self, u: &UserId, t: &AccessToken) -> impl Future<Item=Option<User<'static>>, Error=Error<Void>> + 'a {
        debug!(self.logger, "looking up user"; "id" => %u);

        let logger = self.logger.clone();
        let client = self.client.clone();
        let bytes  = self.bytes.clone();

        let mut url = self.base.clone();
        url.set_path(&format!("/users/{}", u.to_string()));

        request(Method::Get, &url, Some(t))
            .and_then(move |req| client.request(req).from_err())
            .and_then(move |res| {
                match res.status() {
                    StatusCode::Ok if is_json(&res) => {
                        Either::A(Either::A(json(res.body(), bytes)))
                    }
                    StatusCode::NotFound => {
                        Either::A(Either::B(drain(res.body()).map(|()| None)))
                    }
                    _ => Either::B(handle_error!(logger, "looking up user", res, bytes))
                }
            })
    }

    pub fn asset_url<'a>(&self, k: &AssetKey, a: Option<&AssetToken>, t: &AccessToken) -> impl Future<Item=Url, Error=Error<Void>> + 'a {
        debug!(self.logger, "getting asset"; "key" => %k);

        let logger = self.logger.clone();
        let client = self.client.clone();
        let atoken = a.map(|a| header::AssetToken(a.as_str().to_string()));
        let bytes  = self.bytes.clone();

        let mut url = self.base.clone();
        url.set_path(&format!("/assets/v3/{}", k.as_str()));

        request(Method::Get, &url, Some(t))
            .and_then(move |mut req| {
                atoken.map(|t| req.headers_mut().set(t));
                client.request(req).from_err()
            })
            .and_then(move |res| {
                if res.status() != StatusCode::Found {
                    return Either::A(handle_error!(logger, "getting asset", res, bytes))
                }

                let location = res.headers().get::<Location>()
                    .ok_or(Error::Message("missing location header"))
                    .and_then(|loc| Url::parse(loc.as_ref()).map_err(From::from));

                Either::B(drain(res.body()).and_then(|()| future::result(location)))
            })
    }

    pub fn client_prekey<'a>(&self, u: &UserId, c: &ClientId, t: &AccessToken) -> impl Future<Item=Option<ClientPreKey<'static>>, Error=Error<Void>> + 'a {
        debug!(self.logger, "getting prekey"; "user" => %u, "client" => %c);

        let logger = self.logger.clone();
        let client = self.client.clone();
        let bytes  = self.bytes.clone();

        let mut url = self.base.clone();
        url.set_path(&format!("/users/{}/prekeys/{}", u.to_string(), c.as_str()));

        request(Method::Get, &url, Some(t))
            .and_then(move |req| client.request(req).from_err())
            .and_then(move |res| {
                match res.status() {
                    StatusCode::Ok if is_json(&res) => {
                        Either::A(Either::A(json(res.body(), bytes)))
                    }
                    StatusCode::NotFound => {
                        Either::A(Either::B(drain(res.body()).map(|()| None)))
                    }
                    _ => Either::B(handle_error!(logger, "getting prekey", res, bytes))
                }
            })
    }

    /// Get prekeys for all given user clients [`POST /users/prekeys`].
    pub fn prekeys<'a>(&self, u2c: User2Clients<'a>, t: &AccessToken) -> impl Future<Item=PreKeyMap, Error=Error<Void>> + 'a {
        debug!(self.logger, "looking up prekeys");

        let logger = self.logger.clone();
        let client = self.client.clone();
        let bytes  = self.bytes.clone();

        let mut url = self.base.clone();
        url.set_path("/users/prekeys");

        request(Method::Post, &url, Some(t))
            .and_then(with!(bytes => move |req| request_body(bytes, req, u2c)))
            .and_then(move |req| client.request(req).from_err())
            .and_then(move |res| {
                if res.status() == StatusCode::Ok && is_json(&res) {
                    Either::A(json(res.body(), bytes))
                } else {
                    Either::B(handle_error!(logger, "prekeys lookup", res, bytes))
                }
            })
    }

    /// Get prekeys for all clients of the given user [`GET /users/{id}/prekeys`].
    pub fn user_prekeys<'a>(&self, u: &UserId, t: &AccessToken) -> impl Future<Item=ClientPreKeys, Error=Error<Void>> + 'a {
        debug!(self.logger, "looking up user prekeys");

        let logger = self.logger.clone();
        let client = self.client.clone();
        let bytes  = self.bytes.clone();

        let mut url = self.base.clone();
        url.set_path(&format!("/users/{}/prekeys", u.to_string()));

        request(Method::Get, &url, Some(t))
            .and_then(move |req| client.request(req).from_err())
            .and_then(move |res| {
                if res.status() == StatusCode::Ok && is_json(&res) {
                    Either::A(json(res.body(), bytes))
                } else {
                    Either::B(handle_error!(logger, "user prekeys lookup", res, bytes))
                }
            })
    }

    /// Register a new client [`POST /clients`].
    pub fn client_register<'a>(&self, p: client::register::Params<'a>, t: &AccessToken) -> impl Future<Item=ApiClient<'static>, Error=Error<client::register::Error>> + 'a {
        info!(self.logger, "registering client");

        let logger = self.logger.clone();
        let client = self.client.clone();
        let bytes  = self.bytes.clone();

        let mut url = self.base.clone();
        url.set_path("/clients");

        request(Method::Post, &url, Some(t))
            .and_then(with!(bytes => move |req| request_body(bytes, req, p)))
            .and_then(move |req| client.request(req).from_err())
            .and_then(move |res| {
                if res.status() == StatusCode::Created && is_json(&res) {
                    Either::A(json(res.body(), bytes))
                } else {
                    Either::B(handle_error!(logger, "client registration", res, bytes))
                }
            })
    }

    /// Remove the given client [`DELETE /clients/{id}`].
    pub fn client_delete<'a>(&self, c: &ClientId, p: Option<client::delete::Params<'a>>, t: &AccessToken) -> impl Future<Item=(), Error=Error<client::delete::Error>> + 'a {
        info!(self.logger, "deleting client"; "id" => %c);

        let logger = self.logger.clone();
        let client = self.client.clone();
        let bytes  = self.bytes.clone();

        let mut url = self.base.clone();
        url.set_path(&format!("/clients/{}", c.as_str()));

        request(Method::Delete, &url, Some(t))
            .and_then(with!(bytes => move |req| {
                if let Some(params) = p {
                    Either::A(request_body(bytes, req, params))
                } else {
                    Either::B(future::ok(req))
                }
            }))
            .and_then(move |req| client.request(req).from_err())
            .and_then(move |res| {
                if res.status() == StatusCode::Ok {
                    Either::A(drain(res.body()))
                } else {
                    Either::B(handle_error!(logger, "deleting client", res, bytes))
                }
            })
    }

    /// Get information about some client belonging to the requestor [`GET /clients/{id}`].
    pub fn self_client<'a>(&self, c: &ClientId, t: &AccessToken) -> impl Future<Item=Option<ApiClient<'static>>, Error=Error<Void>> + 'a {
        debug!(self.logger, "looking up client"; "id" => %c);

        let logger = self.logger.clone();
        let client = self.client.clone();
        let bytes  = self.bytes.clone();

        let mut url = self.base.clone();
        url.set_path(&format!("/clients/{}", c.as_str()));

        request(Method::Get, &url, Some(t))
            .and_then(move |req| client.request(req).from_err())
            .and_then(move |res| {
                match res.status() {
                    StatusCode::Ok if is_json(&res) => {
                        Either::A(Either::A(json(res.body(), bytes)))
                    }
                    StatusCode::NotFound => {
                        Either::A(Either::B(drain(res.body()).map(|()| None)))
                    }
                    _ => Either::B(handle_error!(logger, "client lookup", res, bytes))
                }
            })
    }

    /// Get information about some client belonging to the given user [`GET /users/{uid}/clients/{cid}`].
    pub fn user_client<'a>(&self, u: &UserId, c: &ClientId, t: &AccessToken) -> impl Future<Item=Option<ApiClient<'static>>, Error=Error<Void>> + 'a {
        debug!(self.logger, "looking up client"; "user" => %u, "id" => %c);

        let logger = self.logger.clone();
        let client = self.client.clone();
        let bytes  = self.bytes.clone();

        let mut url = self.base.clone();
        url.set_path(&format!("/users/{}/clients/{}", u.to_string(), c.as_str()));

        request(Method::Get, &url, Some(t))
            .and_then(move |req| client.request(req).from_err())
            .and_then(move |res| {
                match res.status() {
                    StatusCode::Ok if is_json(&res) => {
                        Either::A(Either::A(json(res.body(), bytes)))
                    }
                    StatusCode::NotFound => {
                        Either::A(Either::B(drain(res.body()).map(|()| None)))
                    }
                    _ => Either::B(handle_error!(logger, "user client lookup", res, bytes))
                }
            })
    }

    /// Get information about all clients belonging to the given user [`GET /users/{id}/clients`].
    pub fn user_clients<'a>(&self, u: &UserId, t: &AccessToken) -> impl Future<Item=Vec<ApiClient<'static>>, Error=Error<Void>> + 'a {
        debug!(self.logger, "looking up clients"; "user" => %u);

        let logger = self.logger.clone();
        let client = self.client.clone();
        let bytes  = self.bytes.clone();

        let mut url = self.base.clone();
        url.set_path(&format!("/users/{}/clients", u.to_string()));

        request(Method::Get, &url, Some(t))
            .and_then(move |req| client.request(req).from_err())
            .and_then(move |res| {
                match res.status() {
                    StatusCode::Ok if is_json(&res) => {
                        Either::A(Either::A(json(res.body(), bytes)))
                    }
                    StatusCode::NotFound => {
                        Either::A(Either::B(drain(res.body()).map(|()| Vec::new())))
                    }
                    _ => Either::B(handle_error!(logger, "user clients lookup", res, bytes))
                }
            })
    }

    /// Lookup conversation IDs [`GET /conversations/ids`]
    pub fn conversations<'a>(&self, len: usize, start: Option<&ConvId>, t: &AccessToken) -> impl Future<Item=Page<Vec<ConvId>>, Error=Error<Void>> + 'a {
        debug!(self.logger, "looking up conversation ids");

        let logger = self.logger.clone();
        let client = self.client.clone();
        let bytes  = self.bytes.clone();

        let mut url = self.base.clone();
        url.set_path("/conversations/ids");
        url.query_pairs_mut().append_pair("size", len.to_string().as_str());
        if let Some(ref s) = start {
            url.query_pairs_mut().append_pair("start", s.to_string().as_str());
        }

        request(Method::Get, &url, Some(t))
            .and_then(move |req| client.request(req).from_err())
            .and_then(move |res| {
                if res.status() == StatusCode::Ok && is_json(&res) {
                    Either::A(json(res.body(), bytes).map(|xs: conv::get::ConvIds| xs.0))
                } else {
                    Either::B(handle_error!(logger, "renew access token", res, bytes))
                }
            })
    }

    /// Lookup conversation by id [`GET /conversations/{id}`]
    pub fn conversation<'a>(&self, id: &ConvId, t: &AccessToken) -> impl Future<Item=Option<Conversation<'static>>, Error=Error<Void>> + 'a {
        debug!(self.logger, "looking up conversation"; "id" => %id);

        let logger = self.logger.clone();
        let client = self.client.clone();
        let bytes  = self.bytes.clone();

        let mut url = self.base.clone();
        url.set_path(&format!("/conversations/{}", id.to_string()));

        request(Method::Get, &url, Some(t))
            .and_then(move |req| client.request(req).from_err())
            .and_then(move |res| {
                match res.status() {
                    StatusCode::Ok if is_json(&res) => {
                        Either::A(Either::A(json(res.body(), bytes)))
                    }
                    StatusCode::NotFound => {
                        Either::A(Either::B(drain(res.body()).map(|()| None)))
                    }
                    _ => Either::B(handle_error!(logger, "conversation lookup", res, bytes))
                }
            })
    }

    /// Create new conversation [`POST /conversations`].
    pub fn conversation_create<'a>(&self, p: conv::create::Params<'a>, t: &AccessToken) -> impl Future<Item=ConvId, Error=Error<Void>> + 'a {
        debug!(self.logger, "creating new conversation");

        let logger = self.logger.clone();
        let client = self.client.clone();
        let bytes  = self.bytes.clone();

        let mut url = self.base.clone();
        url.set_path("/conversations");

        request(Method::Post, &url, Some(t))
            .and_then(with!(bytes => move |req| request_body(bytes, req, p)))
            .and_then(move |req| client.request(req).from_err())
            .and_then(move |res| {
                if res.status() != StatusCode::Created {
                    return Either::A(handle_error!(logger, "creating conversation", res, bytes))
                }

                let cid = res.headers().get::<Location>()
                    .ok_or(Error::Message("missing location header"))
                    .and_then(|id| ConvId::from_str(id.as_ref()).ok_or(Error::Message("invalid conversation id")));

                Either::B(drain(res.body()).and_then(|()| future::result(cid)))
            })
    }

    pub fn conversation_join<'a>(&self, id: &ConvId, t: &AccessToken) -> impl Future<Item=ConvEvent<'static>, Error=Error<conv::join::Error>> + 'a {
        debug!(self.logger, "joining an existing conversation");

        let logger = self.logger.clone();
        let client = self.client.clone();
        let bytes  = self.bytes.clone();

        let mut url = self.base.clone();
        url.set_path(&format!("/conversations/{}/join", id));

        request(Method::Post, &url, Some(t))
            .and_then(move |req| client.request(req).from_err())
            .and_then(move |res| {
                if res.status() == StatusCode::Ok && is_json(&res) {
                    Either::A(json(res.body(), bytes).and_then(|j| {
                        let ce = ConvEvent::from_json(EventType::ConvMemberJoin, &j);
                        future::result(ce).from_err()
                    }))
                } else {
                    Either::B(handle_error!(logger, "error joining conversation", res, bytes))
                }
            })
    }

    pub fn notifications_last<'a>(&self, t: &AccessToken) -> impl Future<Item=Option<NotifId>, Error=Error<Void>> + 'a {
        debug!(self.logger, "get last notification id");

        let logger = self.logger.clone();
        let client = self.client.clone();
        let bytes  = self.bytes.clone();

        let mut url = self.base.clone();
        url.set_path("/notifications/last");

        request(Method::Get, &url, Some(t))
            .and_then(move |req| client.request(req).from_err())
            .and_then(move |res| {
                match res.status() {
                    StatusCode::Ok if is_json(&res) => {
                        Either::A(Either::A(json(res.body(), bytes).map(|n: Notification<'static>| Some(n.id))))
                    }
                    StatusCode::NotFound => {
                        Either::A(Either::B(drain(res.body()).map(|()| None)))
                    }
                    _ => Either::B(handle_error!(logger, "getting last notification id", res, bytes))
                }
            })
    }

    pub fn notifications<'a>(&self, p: &events::get::Params, t: &AccessToken) -> impl Future<Item=Vec<Notification<'static>>, Error=Error<Void>> + 'a {
        info!(self.logger, "notifications");

        let logger = self.logger.clone();
        let client = self.client.clone();
        let bytes  = self.bytes.clone();

        let mut url = self.base.clone();
        url.set_path("/notifications");
        url.query_pairs_mut().append_pair("client", p.client.as_str());
        if let Some(ref s) = p.start {
            url.query_pairs_mut().append_pair("since", s.to_string().as_str());
        }
        if let Some(ref s) = p.size {
            url.query_pairs_mut().append_pair("size", s.to_string().as_str());
        }

        request(Method::Get, &url, Some(t))
            .and_then(move |req| client.request(req).from_err())
            .and_then(move |res| {
                match res.status() {
                    StatusCode::Ok | StatusCode::NotFound if is_json(&res) => Either::A(json(res.body(), bytes)),
                    _ => Either::B(handle_error!(logger, "notifications", res, bytes))
                }
            })
    }

    pub fn send<'a>(&self, p: message::send::Params, t: &AccessToken) -> impl Future<Item=ClientMismatch<'static>, Error=Error<message::send::Error>> + 'a {
        info!(self.logger, "sending message");

        let logger = self.logger.clone();
        let client = self.client.clone();
        let bytes  = self.bytes.clone();

        let mut url = self.base.clone();
        url.set_path(&format!("/conversations/{}/otr/messages", p.conv.to_string()));

        request(Method::Post, &url, Some(t))
            .and_then(with!(bytes => move |req| request_body(bytes, req, p.message)))
            .and_then(move |req| client.request(req).from_err())
            .and_then(move |res| {
                match res.status() {
                    StatusCode::Created if is_json(&res) => {
                        Either::A(Either::A(json(res.body(), bytes)))
                    }
                    StatusCode::PreconditionFailed if is_json(&res) => {
                        Either::A(Either::B(json(res.body(), bytes).and_then(|e| future::err(Error::Error(message::send::Error::Mismatch(e))))))
                    }
                    _ => Either::B(handle_error!(logger, "error sending message", res, bytes))
                }
            })
    }
}

fn request<E>(m: Method, url: &Url, token: Option<&AccessToken>) -> impl Future<Item=Request, Error=E>
    where E: From<UriError> + From<EncodeError>
{
    let     uri = attempt!(Uri::from_str(url.as_str()));
    let mut req = Request::new(m, uri);
    req.headers_mut().set(Accept::json());
    token.map(|t| req.headers_mut().set(Authorization(Bearer { token: t.bearer.to_owned() })));
    future::ok(req)
}

fn request_body<T, E>(mut bytes: BytesMut, mut req: Request, body: T) -> impl Future<Item=Request, Error=E>
    where T: ToJson,
          E: From<EncodeError>
{
    bytes.clear();
    let mut enc = Encoder::new(bytes.writer());
    attempt!(enc.to_json(body));
    req.set_body(enc.into_writer().into_inner().freeze());
    future::ok(req)
}

fn json<T, E>(body: Body, mut bytes: BytesMut) -> impl Future<Item=T, Error=E>
    where T: FromJson,
          E: From<DecodeError> + From<hyper::Error>
{
    bytes.clear();
    body.fold(bytes, |mut bytes, chunk| {
        bytes.extend_from_slice(&*chunk);
        future::ok(bytes) : FutureResult<BytesMut, hyper::Error>
    })
    .from_err()
    .and_then(|bytes| {
        let mut d = Decoder::default(ReadIter::new(Cursor::new(bytes)));
        match d.from_json() {
            Ok(t)  => future::ok(t),
            Err(e) => future::err(e.into())
        }
    })
}

#[inline]
fn drain<E: From<hyper::Error>>(b: Body) -> impl Future<Item=(), Error=E> {
    b.for_each(|_| future::ok(())).from_err()
}

#[inline]
fn is_json(res: &Response) -> bool {
    res.headers().get() == Some(&ContentType::json())
}

