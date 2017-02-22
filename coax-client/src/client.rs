use std::net::TcpStream;
use std::sync::Arc;
use std::time::Duration;

use coax_api::conv::{self, Conversation};
use coax_api::client::{self, Client as ApiClient, User2Clients, ClientMismatch};
use coax_api::events::{self, ConvEvent, EventType, Notification};
use coax_api::message;
use coax_api::prekeys::{ClientPreKey, ClientPreKeys, PreKeyMap};
use coax_api::token::{self, AccessToken};
use coax_api::types::{UserId, ClientId, ConvId, NotifId, ApiError, Page};
use coax_api::user::{self, User, AssetKey, AssetToken};
use coax_net::http::{Method, HttpStream, AsTcp};
use coax_net::http::header::{self, Response, Value, Cookie};
use coax_net::http::header::{COOKIE, AUTHORIZATION, LOCATION, ASSET_TOKEN};
use coax_net::http::tls::{Tls, TlsStream};
use coax_net::rpc::{self, Rpc, Reader};
use error::{Error, Void};
use json::FromJson;
use slog::Logger;
use url::Url;

// Client ///////////////////////////////////////////////////////////////////

struct Timeouts {
    read:  Option<Duration>,
    write: Option<Duration>
}

/// A client of Wire's HTTP API.
///
/// Provides a typed interface to perform synchronous operations.
pub struct Client<'a> {
    url: Url,
    dom: String,
    tls: Arc<Tls>,
    rpc: Rpc<'a, TlsStream>,
    tkn: Option<rpc::Init>,
    log: Logger,
    tme: Timeouts
}

impl<'a> Client<'a> {
    /// Establish a TLS connection to the given URL host.
    pub fn connect(g: &Logger, u: Url, domain: &str, c: Arc<Tls>) -> Result<Client<'a>, Error<Void>> {
        let log = g.new(o!("context" => "Client"));
        debug!(log, "connect"; "url" => u.as_str());
        let tcp = TcpStream::connect(&u)?;
        let tls = TlsStream::new(c.as_ref(), domain, tcp)?;
        let (rpc, tkn) = Rpc::new(&log, HttpStream::new(&log, tls), true);
        Ok(Client {
            url: u,
            dom: String::from(domain),
            tls: c,
            rpc: rpc,
            tkn: Some(tkn),
            log: log,
            tme: Timeouts { read: None, write: None }
        })
    }

    /// Set TCP read timeout.
    pub fn set_read_timeout(&mut self, d: Option<Duration>) -> Result<(), Error<Void>> {
        self.rpc.stream().as_tcp().set_read_timeout(d.clone())?;
        self.tme.read = d;
        Ok(())
    }

    /// Set TCP write timeout.
    pub fn set_write_timeout(&mut self, d: Option<Duration>) -> Result<(), Error<Void>> {
        self.rpc.stream().as_tcp().set_write_timeout(d.clone())?;
        self.tme.write = d;
        Ok(())
    }

    /// Re-establish the connection.
    pub fn reconnect(&mut self) -> Result<(), Error<Void>> {
        debug!(self.log, "reconnect"; "host" => self.url.host_str().unwrap_or("N/A"));
        let tcp = TcpStream::connect(&self.url)?;
        tcp.set_read_timeout(self.tme.read.clone())?;
        tcp.set_write_timeout(self.tme.write.clone())?;
        let tls = TlsStream::new(self.tls.as_ref(), &self.dom, tcp)?;
        self.tkn = Some(self.rpc.set_stream(tls));
        Ok(())
    }

    /// Current HTTP response.
    pub fn response(&self) -> &Response {
        self.rpc.response()
    }

    /// Renew an `AccessToken` [`POST /access`].
    pub fn access_renew<'b>(&mut self, c: &Cookie, t: Option<&AccessToken>) -> Result<token::Credentials<'b, Option<Cookie<'b>>>, Error<token::renew::Error>> {
        info!(self.log, "renewing access token");
        self.url.set_path("/access");
        let cook = format!("{}={}", c.name(), c.value());
        let tkn = self.tkn.take().ok_or(Error::InvalidState)?;
        let tkn = if let Some(at) = t {
            let hdrs = &[(AUTHORIZATION, Value::new(at.bearer.as_bytes())), (COOKIE, Value::new(cook.as_bytes()))];
            self.rpc.send(tkn, Method::Post, &self.url, hdrs)?
        } else {
            let hdrs = &[(COOKIE, Value::new(cook.as_bytes()))];
            self.rpc.send(tkn, Method::Post, &self.url, hdrs)?
        };
        let tkn = self.rpc.recv_header(tkn)?;
        let json_resp = is_json(self.response());
        match self.response().status() {
            200 if json_resp => {
                let at = self.recv_json(tkn)?;
                let cok = self.response().cookie("zuid").map(Cookie::into_owned);
                Ok(token::Credentials::new(at, cok))
            }
            num if json_resp => {
                let e: ApiError = self.recv_json(tkn)?;
                error!(self.log, "renew access token error"; "status" => num, "error" => format!("{:?}", e));
                Err(Error::Error(e.into()))
            }
            num => {
                let e = self.error_response(tkn, json_resp)?;
                error!(self.log, "renew access token error"; "status" => num, "error" => format!("{:?}", e));
                Err(e)
            }
        }
    }

    /// Login some user [`POST /login`].
    pub fn user_login<'b>(&mut self, p: &user::login::Params) -> Result<token::Credentials<'b, Cookie<'b>>, Error<user::login::Error>> {
        info!(self.log, "login";
                "email" => p.email.as_ref().map(|x| x.as_str()).unwrap_or("n/a"),
                "phone" => p.phone.as_ref().map(|x| x.as_str()).unwrap_or("n/a"));
        let mut url = self.url.clone();
        url.set_path("/login");
        url.set_query(Some("persist=true"));
        let tkn = self.tkn.take().ok_or(Error::InvalidState)?;
        let tkn = self.rpc.send_json(tkn, Method::Post, &url, &[], p)?;
        let tkn = self.rpc.recv_header(tkn)?;
        let json_resp = is_json(self.response());
        match self.response().status() {
            200 if json_resp => {
                let t = self.recv_json(tkn)?;
                if let Some(c) = self.response().cookie("zuid") {
                    Ok(token::Credentials::new(t, c.into_owned()))
                } else {
                    Err(Error::Error(user::login::Error::MissingCookie))
                }
            }
            num if json_resp => {
                let e: ApiError = self.recv_json(tkn)?;
                error!(self.log, "login error"; "status" => num, "error" => format!("{:?}", e));
                Err(Error::Error(e.into()))
            }
            num => {
                let e = self.error_response(tkn, json_resp)?;
                error!(self.log, "login error"; "status" => num, "error" => format!("{:?}", e));
                Err(e)
            }
        }
    }

    /// Logout some user [`POST /access/logout`].
    pub fn user_logout(&mut self, c: &Cookie, t: &AccessToken) -> Result<(), Error<Void>> {
        info!(self.log, "logout");
        self.url.set_path("/access/logout");
        let cook = format!("{}={}", c.name(), c.value());
        let hdrs = &[(AUTHORIZATION, Value::new(t.bearer.as_bytes())), (COOKIE, Value::new(cook.as_bytes()))];
        let tkn  = self.tkn.take().ok_or(Error::InvalidState)?;
        let tkn  = self.rpc.send(tkn, Method::Post, &self.url, hdrs)?;
        let tkn  = self.rpc.recv_header(tkn)?;
        match self.response().status() {
            200 => self.drain(tkn),
            num => {
                let j = is_json(self.response());
                let e = self.error_response(tkn, j)?;
                error!(self.log, "logout error"; "status" => num, "error" => format!("{:?}", e));
                Err(e)
            }
        }
    }

    /// Register a new user [`POST /register`].
    pub fn user_register<'b>(&mut self, p: &user::register::Params) -> Result<User<'b>, Error<user::register::Error>> {
        info!(self.log, "registering user";
                "email" => p.email.as_ref().map(|x| x.as_str()).unwrap_or("n/a"),
                "phone" => p.phone.as_ref().map(|x| x.as_str()).unwrap_or("n/a"));
        self.url.set_path("/register");
        let tkn = self.tkn.take().ok_or(Error::InvalidState)?;
        let tkn = self.rpc.send_json(tkn, Method::Post, &self.url, &[], p)?;
        let tkn = self.rpc.recv_header(tkn)?;
        let json_resp = is_json(self.response());
        match self.response().status() {
            201 if json_resp => self.recv_json(tkn).map_err(From::from),
            num if json_resp => {
                let e = self.recv_json(tkn)?;
                error!(self.log, "user registration error"; "status" => num, "error" => format!("{:?}", e));
                Err(Error::Error((e: ApiError).into()))
            }
            num => {
                let e = self.error_response(tkn, json_resp)?;
                error!(self.log, "user registration error"; "status" => num, "error" => format!("{:?}", e));
                Err(e)
            }
        }
    }

    pub fn set_connect_status(&mut self, p: &user::connect::update::Params, t: &AccessToken) -> Result<bool, Error<user::connect::update::Error>> {
        info!(self.log, "updating connect status to user"; "user" => p.user.to_string());
        self.url.set_path(&format!("/connections/{}", p.user));
        let hdrs = &[(AUTHORIZATION, Value::new(t.bearer.as_bytes()))];
        let tkn = self.tkn.take().ok_or(Error::InvalidState)?;
        let tkn = self.rpc.send_json(tkn, Method::Put, &self.url, hdrs, p)?;
        let tkn = self.rpc.recv_header(tkn)?;
        let json_resp = is_json(self.response());
        match self.response().status() {
            200 if json_resp => {
                debug!(self.log, "connection status updated"; "user" => p.user.to_string());
                self.drain(tkn)?;
                Ok(true)
            }
            204 => {
                debug!(self.log, "connection status unchanged"; "user" => p.user.to_string());
                self.drain(tkn)?;
                Ok(false)
            }
            num if json_resp => {
                let e: ApiError = self.recv_json(tkn)?;
                error!(self.log, "error updating connection status"; "status" => num, "error" => format!("{:?}", e));
                Err(Error::Error(e.into()))
            }
            num => {
                let e = self.error_response(tkn, json_resp)?;
                error!(self.log, "error updaeting connection status"; "status" => num, "error" => format!("{:?}", e));
                Err(e)
            }
        }
    }

    /// Send a connection request to some user [`POST /connections`].
    pub fn user_connect<'b>(&mut self, p: &user::connect::Params, t: &AccessToken) -> Result<user::Connection<'b>, Error<user::connect::Error>> {
        info!(self.log, "connecting to user"; "user" => p.user.to_string());
        self.url.set_path("/connections");
        let hdrs = &[(AUTHORIZATION, Value::new(t.bearer.as_bytes()))];
        let tkn = self.tkn.take().ok_or(Error::InvalidState)?;
        let tkn = self.rpc.send_json(tkn, Method::Post, &self.url, hdrs, p)?;
        let tkn = self.rpc.recv_header(tkn)?;
        let json_resp = is_json(self.response());
        match self.response().status() {
            200 | 201 if json_resp => self.recv_json(tkn).map_err(From::from),
            num if json_resp => {
                let e: ApiError = self.recv_json(tkn)?;
                error!(self.log, "error connecting user"; "status" => num, "error" => format!("{:?}", e));
                Err(Error::Error(e.into()))
            }
            num => {
                let e = self.error_response(tkn, json_resp)?;
                error!(self.log, "error connecting user"; "status" => num, "error" => format!("{:?}", e));
                Err(e)
            }
        }
    }

    /// Lookup connection to user [`GET /connections/{user}`]
    pub fn user_connection<'b>(&mut self, to: &UserId, t: &AccessToken) -> Result<Option<user::Connection<'b>>, Error<Void>> {
        info!(self.log, "lookup connection"; "id" => to.to_string());
        self.url.set_path(&format!("/connections/{}", to.to_string()));
        let hdrs = &[(AUTHORIZATION, Value::new(t.bearer.as_bytes()))];
        let tkn  = self.tkn.take().ok_or(Error::InvalidState)?;
        let tkn  = self.rpc.send(tkn, Method::Get, &self.url, hdrs)?;
        let tkn  = self.rpc.recv_header(tkn)?;
        let json_resp = is_json(self.response());
        match self.response().status() {
            200 if json_resp => self.recv_json(tkn).map_err(From::from),
            404 => {
                debug!(self.log, "connection not found"; "id" => to.to_string());
                self.drain(tkn)?;
                Ok(None)
            }
            num => {
                let e = self.error_response(tkn, json_resp)?;
                error!(self.log, "error getting connection"; "status" => num, "error" => format!("{:?}", e));
                Err(e)
            }
        }
    }

    /// Lookup connections to user [`GET /connections`]
    pub fn user_connections<'b>(&mut self, len: usize, start: Option<&UserId>, t: &AccessToken) -> Result<Page<Vec<user::Connection<'b>>>, Error<Void>> {
        info!(self.log, "listing user connections");
        let mut url = self.url.clone();
        url.set_path("/connections");
        url.query_pairs_mut().append_pair("size", len.to_string().as_str());
        if let Some(s) = start {
            url.query_pairs_mut().append_pair("start", s.to_string().as_str());
        }
        let hdrs = &[(AUTHORIZATION, Value::new(t.bearer.as_bytes()))];
        let tkn  = self.tkn.take().ok_or(Error::InvalidState)?;
        let tkn  = self.rpc.send(tkn, Method::Get, &url, hdrs)?;
        let tkn  = self.rpc.recv_header(tkn)?;
        let json_resp = is_json(self.response());
        match self.response().status() {
            200 if json_resp =>
                self.recv_json(tkn)
                    .map(|x: user::connect::get::Connections<'b>| x.0)
                    .map_err(From::from),
            num => {
                let e = self.error_response(tkn, json_resp)?;
                error!(self.log, "error getting connections"; "status" => num, "error" => format!("{:?}", e));
                Err(e)
            }
        }
    }

    /// Get own user profile [`GET /self`].
    pub fn self_user<'b>(&mut self, t: &AccessToken) -> Result<User<'b>, Error<Void>> {
        debug!(self.log, "looking up self");
        self.url.set_path("/self");
        let hdrs = &[(AUTHORIZATION, Value::new(t.bearer.as_bytes()))];
        let tkn  = self.tkn.take().ok_or(Error::InvalidState)?;
        let tkn  = self.rpc.send(tkn, Method::Get, &self.url, hdrs)?;
        let tkn  = self.rpc.recv_header(tkn)?;
        let json_resp = is_json(self.response());
        match self.response().status() {
            200 if json_resp => self.recv_json(tkn).map_err(From::from),
            num => {
                let e = self.error_response(tkn, json_resp)?;
                error!(self.log, "self lookup error"; "status" => num, "error" => format!("{:?}", e));
                Err(e)
            }
        }
    }

    /// Get some user profile [`GET /users/{id}`].
    pub fn user<'b>(&mut self, u: &UserId, t: &AccessToken) -> Result<Option<User<'b>>, Error<Void>> {
        debug!(self.log, "looking up user"; "id" => u.to_string());
        self.url.set_path(&format!("/users/{}", u.to_string()));
        let hdrs = &[(AUTHORIZATION, Value::new(t.bearer.as_bytes()))];
        let tkn  = self.tkn.take().ok_or(Error::InvalidState)?;
        let tkn  = self.rpc.send(tkn, Method::Get, &self.url, hdrs)?;
        let tkn  = self.rpc.recv_header(tkn)?;
        let json_resp = is_json(self.response());
        match self.response().status() {
            200 if json_resp => self.recv_json(tkn).map_err(From::from),
            404 if json_resp => {
                self.drain(tkn)?;
                warn!(self.log, "user not found"; "id" => u.to_string());
                Ok(None)
            }
            num => {
                let e = self.error_response(tkn, json_resp)?;
                error!(self.log, "user lookup error"; "status" => num, "error" => format!("{:?}", e));
                Err(e)
            }
        }
    }

    pub fn asset_url(&mut self, k: &AssetKey, a: Option<&AssetToken>, t: &AccessToken) -> Result<Url, Error<Void>> {
        debug!(self.log, "getting asset"; "key" => k.as_str());
        self.url.set_path(&format!("/assets/v3/{}", k.as_str()));
        let tkn  = self.tkn.take().ok_or(Error::InvalidState)?;
        let tkn  =
            if let Some(x) = a {
                let hdrs = &[(AUTHORIZATION, Value::new(t.bearer.as_bytes())), (ASSET_TOKEN, Value::new(x.as_str().as_bytes()))];
                self.rpc.send(tkn, Method::Get, &self.url, hdrs)?
            } else {
                let hdrs = &[(AUTHORIZATION, Value::new(t.bearer.as_bytes()))];
                self.rpc.send(tkn, Method::Get, &self.url, hdrs)?
            };
        let tkn  = self.rpc.recv_header(tkn)?;
        match self.response().status() {
            302 => {
                self.drain(tkn)?;
                if let Some(loc) = self.response().header(LOCATION) {
                    loc.as_str()
                       .and_then(|s| Url::parse(s).ok())
                       .ok_or(Error::Message("invalid URL in location header"))
                } else {
                    Err(Error::Message("missing or invalid location header"))
                }
            }
            num => {
                let json_resp = is_json(self.response());
                let e = self.error_response(tkn, json_resp)?;
                error!(self.log, "error getting asset url"; "status" => num, "error" => format!("{:?}", e));
                Err(e)
            }
        }
    }

    pub fn prepare_download(&mut self, u: &Url, dom: &str) -> Result<(Rpc<TlsStream>, rpc::Head), Error<Void>> {
        let tcp = TcpStream::connect(&u)?;
        let tls = TlsStream::new(self.tls.as_ref(), dom, tcp)?;
        let (mut rpc, tkn) = Rpc::new(&self.log, HttpStream::new(&self.log, tls), true);
        let tkn = rpc.send(tkn, Method::Get, u, &[])?;
        let tkn = rpc.recv_header(tkn)?;
        Ok((rpc, tkn))
    }

    pub fn client_prekey<'b>(&mut self, u: &UserId, c: &ClientId, t: &AccessToken) -> Result<Option<ClientPreKey<'b>>, Error<Void>> {
        debug!(self.log, "getting prekey"; "user" => u.to_string(), "client" => c.as_str());
        self.url.set_path(&format!("/users/{}/prekeys/{}", u.to_string(), c.as_str()));
        let hdrs = &[(AUTHORIZATION, Value::new(t.bearer.as_bytes()))];
        let tkn  = self.tkn.take().ok_or(Error::InvalidState)?;
        let tkn  = self.rpc.send(tkn, Method::Get, &self.url, hdrs)?;
        let tkn  = self.rpc.recv_header(tkn)?;
        let json_resp = is_json(self.response());
        match self.response().status() {
            200 if json_resp => self.recv_json(tkn).map_err(From::from),
            404 if json_resp => {
                self.drain(tkn)?;
                warn!(self.log, "user or client not found"; "user" => u.to_string(), "client" => c.as_str());
                Ok(None)
            }
            num => {
                let e = self.error_response(tkn, json_resp)?;
                error!(self.log, "error getting prekey"; "status" => num, "error" => format!("{:?}", e));
                Err(e)
            }
        }
    }

    /// Get prekeys for all given user clients [`POST /users/prekeys`].
    pub fn prekeys(&mut self, u2c: &User2Clients, t: &AccessToken) -> Result<PreKeyMap, Error<Void>> {
        debug!(self.log, "looking up prekeys");
        self.url.set_path("/users/prekeys");
        let hdrs = &[(AUTHORIZATION, Value::new(t.bearer.as_bytes()))];
        let tkn  = self.tkn.take().ok_or(Error::InvalidState)?;
        let tkn  = self.rpc.send_json(tkn, Method::Post, &self.url, hdrs, u2c)?;
        let tkn  = self.rpc.recv_header(tkn)?;
        let json_resp = is_json(self.response());
        match self.response().status() {
            200 if json_resp => self.recv_json(tkn).map_err(From::from),
            num => {
                let e = self.error_response(tkn, json_resp)?;
                error!(self.log, "prekeys lookup error"; "status" => num, "error" => format!("{:?}", e));
                Err(e)
            }
        }
    }

    /// Get prekeys for all clients of the given user [`GET /users/{id}/prekeys`].
    pub fn user_prekeys(&mut self, u: &UserId, t: &AccessToken) -> Result<ClientPreKeys, Error<Void>> {
        debug!(self.log, "looking up user prekeys");
        self.url.set_path(&format!("/users/{}/prekeys", u.to_string()));
        let hdrs = &[(AUTHORIZATION, Value::new(t.bearer.as_bytes()))];
        let tkn  = self.tkn.take().ok_or(Error::InvalidState)?;
        let tkn  = self.rpc.send(tkn, Method::Get, &self.url, hdrs)?;
        let tkn  = self.rpc.recv_header(tkn)?;
        let json_resp = is_json(self.response());
        match self.response().status() {
            200 if json_resp => self.recv_json(tkn).map_err(From::from),
            num => {
                let e = self.error_response(tkn, json_resp)?;
                error!(self.log, "user prekeys lookup error"; "status" => num, "error" => format!("{:?}", e));
                Err(e)
            }
        }
    }

    /// Register a new client [`POST /clients`].
    pub fn client_register<'b>(&mut self, p: &client::register::Params, t: &AccessToken) -> Result<ApiClient<'b>, Error<client::register::Error>> {
        info!(self.log, "registering client");
        self.url.set_path("/clients");
        let hdrs = &[(AUTHORIZATION, Value::new(t.bearer.as_bytes()))];
        let tkn  = self.tkn.take().ok_or(Error::InvalidState)?;
        let tkn  = self.rpc.send_json(tkn, Method::Post, &self.url, hdrs, p)?;
        let tkn  = self.rpc.recv_header(tkn)?;
        let json_resp = is_json(self.response());
        match self.response().status() {
            201 if json_resp => Ok(self.recv_json(tkn)?),
            num if json_resp => {
                let e: ApiError = self.recv_json(tkn)?;
                error!(self.log, "client registration error"; "status" => num, "error" => format!("{:?}", e));
                Err(Error::Error(e.into()))
            }
            num => {
                let e = self.error_response(tkn, json_resp)?;
                error!(self.log, "client registration error"; "status" => num, "error" => format!("{:?}", e));
                Err(e)
            }
        }
    }

    /// Remove the given client [`DELETE /clients/{id}`].
    pub fn client_delete(&mut self, c: &ClientId, p: Option<&client::delete::Params>, t: &AccessToken) -> Result<(), Error<client::delete::Error>> {
        info!(self.log, "deleting client"; "id" => c.as_str());
        self.url.set_path(&format!("/clients/{}", c.as_str()));
        let hdrs = &[(AUTHORIZATION, Value::new(t.bearer.as_bytes()))];
        let tkn  = self.tkn.take().ok_or(Error::InvalidState)?;
        let tkn  = if let Some(ref pp) = p {
            self.rpc.send_json(tkn, Method::Delete, &self.url, hdrs, pp)?
        } else {
            self.rpc.send(tkn, Method::Delete, &self.url, hdrs)?
        };
        let tkn = self.rpc.recv_header(tkn)?;
        let json_resp = is_json(self.response());
        match self.response().status() {
            200 => self.drain(tkn),
            num if json_resp => {
                let e: ApiError = self.recv_json(tkn)?;
                error!(self.log, "client deletion error"; "status" => num, "error" => format!("{:?}", e), "id" => c.as_str());
                Err(Error::Error(e.into()))
            }
            num => {
                let e = self.error_response(tkn, json_resp)?;
                error!(self.log, "client deletion error"; "status" => num, "error" => format!("{:?}", e));
                Err(e)
            }
        }
    }

    /// Get information about some client belonging to the requestor [`GET /clients/{id}`].
    pub fn self_client<'b>(&mut self, c: &ClientId, t: &AccessToken) -> Result<Option<ApiClient<'b>>, Error<Void>> {
        debug!(self.log, "looking up client"; "id" => c.as_str());
        self.url.set_path(&format!("/clients/{}", c.as_str()));
        let hdrs = &[(AUTHORIZATION, Value::new(t.bearer.as_bytes()))];
        let tkn  = self.tkn.take().ok_or(Error::InvalidState)?;
        let tkn  = self.rpc.send(tkn, Method::Get, &self.url, hdrs)?;
        let tkn  = self.rpc.recv_header(tkn)?;
        let json_resp = is_json(self.response());
        match self.response().status() {
            200 if json_resp => self.recv_json(tkn).map_err(From::from),
            404 if json_resp => {
                debug!(self.log, "client not found"; "id" => c.as_str());
                self.drain(tkn)?;
                Ok(None)
            }
            num => {
                let e = self.error_response(tkn, json_resp)?;
                error!(self.log, "client lookup error"; "status" => num, "error" => format!("{:?}", e));
                Err(e)
            }
        }
    }

    /// Get information about some client belonging to the given user [`GET /users/{uid}/clients/{cid}`].
    pub fn user_client<'b>(&mut self, u: &UserId, c: &ClientId, t: &AccessToken) -> Result<Option<ApiClient<'b>>, Error<Void>> {
        debug!(self.log, "looking up client"; "user" => u.to_string(), "id" => c.as_str());
        self.url.set_path(&format!("/users/{}/clients/{}", u.to_string(), c.as_str()));
        let hdrs = &[(AUTHORIZATION, Value::new(t.bearer.as_bytes()))];
        let tkn  = self.tkn.take().ok_or(Error::InvalidState)?;
        let tkn  = self.rpc.send(tkn, Method::Get, &self.url, hdrs)?;
        let tkn  = self.rpc.recv_header(tkn)?;
        let json_resp = is_json(self.response());
        match self.response().status() {
            200 if json_resp => self.recv_json(tkn).map_err(From::from),
            404 if json_resp => {
                debug!(self.log, "client not found"; "user" => u.to_string(), "id" => c.as_str());
                self.drain(tkn)?;
                Ok(None)
            }
            num => {
                let e = self.error_response(tkn, json_resp)?;
                error!(self.log, "client lookup error"; "status" => num, "error" => format!("{:?}", e));
                Err(e)
            }
        }
    }

    /// Get information about all clients belonging to the given user [`GET /users/{id}/clients`].
    pub fn user_clients<'b>(&mut self, u: &UserId, t: &AccessToken) -> Result<Vec<ApiClient<'b>>, Error<Void>> {
        debug!(self.log, "looking up clients"; "user" => u.to_string());
        self.url.set_path(&format!("/users/{}/clients", u.to_string()));
        let hdrs = &[(AUTHORIZATION, Value::new(t.bearer.as_bytes()))];
        let tkn  = self.tkn.take().ok_or(Error::InvalidState)?;
        let tkn  = self.rpc.send(tkn, Method::Get, &self.url, hdrs)?;
        let tkn  = self.rpc.recv_header(tkn)?;
        let json_resp = is_json(self.response());
        match self.response().status() {
            200 if json_resp => self.recv_json(tkn).map_err(From::from),
            404 if json_resp => {
                debug!(self.log, "user not found"; "user" => u.to_string());
                self.drain(tkn)?;
                Ok(Vec::new())
            }
            num => {
                let e = self.error_response(tkn, json_resp)?;
                error!(self.log, "clients lookup error"; "status" => num, "error" => format!("{:?}", e));
                Err(e)
            }
        }
    }

    /// Lookup conversation IDs [`GET /conversations/ids`]
    pub fn conversations<'b>(&mut self, len: usize, start: Option<&ConvId>, t: &AccessToken) -> Result<Page<Vec<ConvId>>, Error<Void>> {
        debug!(self.log, "looking up conversation ids");
        let mut url = self.url.clone();
        url.set_path("/conversations/ids");
        url.query_pairs_mut().append_pair("size", len.to_string().as_str());
        if let Some(ref s) = start {
            url.query_pairs_mut().append_pair("start", s.to_string().as_str());
        }
        let hdrs = &[(AUTHORIZATION, Value::new(t.bearer.as_bytes()))];
        let tkn  = self.tkn.take().ok_or(Error::InvalidState)?;
        let tkn  = self.rpc.send(tkn, Method::Get, &url, hdrs)?;
        let tkn  = self.rpc.recv_header(tkn)?;
        let json_resp = is_json(self.response());
        match self.response().status() {
            200 if json_resp =>
                self.recv_json(tkn)
                    .map(|x: conv::get::ConvIds| x.0)
                    .map_err(From::from),
            num => {
                let e = self.error_response(tkn, json_resp)?;
                error!(self.log, "conversation lookup error"; "status" => num, "error" => format!("{:?}", e));
                Err(e)
            }
        }
    }

    /// Lookup conversation by id [`GET /conversations/{id}`]
    pub fn conversation<'b>(&mut self, id: &ConvId, t: &AccessToken) -> Result<Option<Conversation<'b>>, Error<Void>> {
        debug!(self.log, "looking up conversation"; "id" => id.to_string());
        self.url.set_path(&format!("/conversations/{}", id.to_string()));
        let hdrs = &[(AUTHORIZATION, Value::new(t.bearer.as_bytes()))];
        let tkn  = self.tkn.take().ok_or(Error::InvalidState)?;
        let tkn  = self.rpc.send(tkn, Method::Get, &self.url, hdrs)?;
        let tkn  = self.rpc.recv_header(tkn)?;
        let json_resp = is_json(self.response());
        match self.response().status() {
            200 if json_resp => self.recv_json(tkn).map_err(From::from),
            404 if json_resp => {
                debug!(self.log, "conversation not found"; "id" => id.to_string());
                self.drain(tkn)?;
                Ok(None)
            }
            num => {
                let e = self.error_response(tkn, json_resp)?;
                error!(self.log, "conversation lookup error"; "status" => num, "error" => format!("{:?}", e));
                Err(e)
            }
        }
    }

    /// Create new conversation [`POST /conversations`].
    pub fn conversation_create(&mut self, p: &conv::create::Params, t: &AccessToken) -> Result<ConvId, Error<Void>> {
        debug!(self.log, "creating new conversation");
        self.url.set_path("/conversations");
        let hdrs = &[(AUTHORIZATION, Value::new(t.bearer.as_bytes()))];
        let tkn  = self.tkn.take().ok_or(Error::InvalidState)?;
        let tkn  = self.rpc.send_json(tkn, Method::Post, &self.url, hdrs, p)?;
        let tkn  = self.rpc.recv_header(tkn)?;
        let json_resp = is_json(self.response());
        match self.response().status() {
            201 => {
                self.drain(tkn)?;
                if let Some(loc) = self.response().header(LOCATION) {
                    loc.as_str()
                       .and_then(ConvId::from_str)
                       .ok_or(Error::Message("invalid UUID in location header"))
                } else {
                    Err(Error::Message("missing or invalid location header"))
                }
            }
            num => {
                let e = self.error_response(tkn, json_resp)?;
                error!(self.log, "conversation creation error"; "status" => num, "error" => format!("{:?}", e));
                Err(e)
            }
        }
    }

    pub fn conversation_join<'b>(&mut self, id: &ConvId, t: &AccessToken) -> Result<ConvEvent<'b>, Error<conv::join::Error>> {
        debug!(self.log, "joining an existing conversation");
        self.url.set_path(&format!("/conversations/{}/join", id));
        let hdrs = &[(AUTHORIZATION, Value::new(t.bearer.as_bytes()))];
        let tkn  = self.tkn.take().ok_or(Error::InvalidState)?;
        let tkn  = self.rpc.send(tkn, Method::Post, &self.url, hdrs)?;
        let tkn  = self.rpc.recv_header(tkn)?;
        let json_resp = is_json(self.response());
        match self.response().status() {
            200 if json_resp => {
                let j = self.recv_json(tkn)?;
                let e = ConvEvent::from_json(EventType::ConvMemberJoin, &j)?;
                Ok(e)
            }
            num if json_resp => {
                let e: ApiError = self.recv_json(tkn)?;
                error!(self.log, "error joining conversation"; "statuc" => num, "error" => format!("{:?}", e), "id" => id.to_string());
                Err(Error::Error(e.into()))
            }
            num => {
                let e = self.error_response(tkn, json_resp)?;
                error!(self.log, "error joining conversation"; "status" => num, "error" => format!("{:?}", e));
                Err(e)
            }
        }
    }

    pub fn notifications_last(&mut self, t: &AccessToken) -> Result<Option<NotifId>, Error<Void>> {
        debug!(self.log, "get last notification id");
        self.url.set_path("/notifications/last");
        let hdrs = &[(AUTHORIZATION, Value::new(t.bearer.as_bytes()))];
        let tkn  = self.tkn.take().ok_or(Error::InvalidState)?;
        let tkn  = self.rpc.send(tkn, Method::Get, &self.url, hdrs)?;
        let tkn  = self.rpc.recv_header(tkn)?;
        let json_resp = is_json(self.response());
        match self.response().status() {
            200 if json_resp => {
                let n: Notification = self.recv_json(tkn)?;
                debug!(self.log, "last notification"; "id" => n.id.to_string());
                Ok(Some(n.id))
            }
            404 => {
                self.drain(tkn)?;
                Ok(None)
            }
            num => {
                let e = self.error_response(tkn, json_resp)?;
                error!(self.log, "error getting last notification"; "status" => num, "error" => format!("{:?}", e));
                Err(e)
            }
        }
    }

    pub fn notifications(&mut self, p: &events::get::Params, t: &AccessToken) -> Result<Vec<Notification<'static>>, Error<Void>> {
        debug!(self.log, "get notifications");
        let mut vec = Vec::new();
        let token = {
            let mut rdr = self.notifications_reader(p, t)?;
            {
                let mut itr = events::get::Iter::from_read(&mut rdr)?;
                while let Some(n) = itr.next() {
                    vec.push(n)
                }
                if let Some(e) = itr.take_error() {
                    return Err(Error::Json(e))
                }
            }
            rdr.into()?
        };
        self.reset(token);
        Ok(vec)
    }

    pub fn notifications_reader(&mut self, p: &events::get::Params, t: &AccessToken) -> Result<Reader<TlsStream>, Error<Void>> {
        info!(self.log, "get notifications (reader)");
        let mut url = self.url.clone();
        url.set_path("/notifications");
        url.query_pairs_mut().append_pair("client", p.client.as_str());
        if let Some(ref s) = p.start {
            url.query_pairs_mut().append_pair("since", s.to_string().as_str());
        }
        if let Some(ref s) = p.size {
            url.query_pairs_mut().append_pair("size", s.to_string().as_str());
        }
        let hdrs = &[(AUTHORIZATION, Value::new(t.bearer.as_bytes()))];
        let tkn  = self.tkn.take().ok_or(Error::InvalidState)?;
        let tkn  = self.rpc.send(tkn, Method::Get, &url, hdrs)?;
        let tkn  = self.rpc.recv_header(tkn)?;
        let json_resp = is_json(self.response());
        match self.response().status() {
            200 if json_resp => self.rpc.reader(tkn).map_err(From::from),
            num => {
                let e = self.error_response(tkn, json_resp)?;
                error!(self.log, "error getting events"; "status" => num, "error" => format!("{:?}", e));
                Err(e)
            }
        }
    }

    pub fn reset(&mut self, i: rpc::Init) {
        self.tkn = Some(i)
    }

    pub fn send<'b>(&mut self, p: &message::send::Params, t: &AccessToken) -> Result<ClientMismatch<'b>, Error<message::send::Error>> {
        info!(self.log, "sending message");
        self.url.set_path(&format!("/conversations/{}/otr/messages", p.conv.to_string()));
        let hdrs = &[(AUTHORIZATION, Value::new(t.bearer.as_bytes()))];
        let tkn  = self.tkn.take().ok_or(Error::InvalidState)?;
        let tkn  = self.rpc.send_json(tkn, Method::Post, &self.url, hdrs, &p.message)?;
        let tkn  = self.rpc.recv_header(tkn)?;
        let json_resp = is_json(self.response());
        match self.response().status() {
            201 if json_resp => self.recv_json(tkn).map_err(From::from),
            412 if json_resp => {
                let e = self.recv_json(tkn)?;
                Err(Error::Error(message::send::Error::Mismatch(e)))
            }
            num if json_resp => {
                let e: ApiError = self.recv_json(tkn)?;
                error!(self.log, "error sending message"; "status" => num, "error" => format!("{:?}", e));
                Err(Error::Error(e.into()))
            }
            num => {
                let e = self.error_response(tkn, json_resp)?;
                error!(self.log, "error sending message"; "status" => num, "error" => format!("{:?}", e));
                Err(e)
            }
        }
    }

    fn recv_json<T: FromJson>(&mut self, t: rpc::Head) -> Result<T, rpc::Error> {
        let (x, tkn) = self.rpc.recv_json(t)?;
        self.tkn = Some(tkn);
        Ok(x)
    }

    fn drain<E>(&mut self, t: rpc::Head) -> Result<(), Error<E>> {
        self.tkn = Some(self.rpc.drain(t)?);
        Ok(())
    }

    fn error_response<E>(&mut self, t: rpc::Head, json_resp: bool) -> Result<Error<E>, Error<E>> {
        if json_resp {
            let (e, t) = self.rpc.recv_json(t)?;
            self.tkn = Some(t);
            Ok(Error::Api(e))
        } else {
            let (b, t) = {
                let mut r = self.rpc.reader(t)?;
                let b = rpc::read_bytes(&mut r, 4096)?;
                let t = r.into()?;
                (b, t)
            };
            self.tkn = Some(t);
            Ok(Error::Response(self.response().status(), b))
        }
    }
}

fn is_json(r: &Response) -> bool {
    r.content_type() == Some(header::ContentType::AppJson)
}

