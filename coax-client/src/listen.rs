use std::io::{self, ErrorKind};
use std::net::TcpStream;
use std::sync::Arc;
use std::time::Duration;

use coax_api::token::AccessToken;
use coax_net::http::Stream;
use coax_net::http::header::{AUTHORIZATION, Value};
use coax_net::http::tls::{Tls, TlsStream};
use coax_ws::io as ws;
use json::{FromJson, Decoder};
use json::decoder::{ReadIter, ReadError};
use error::{Error, Void};
use slog::Logger;
use url::Url;

// Listener /////////////////////////////////////////////////////////////////

pub struct Listener<'a, S: Stream> {
    url:       Url,
    domain:    String,
    tls:       Option<Arc<Tls>>,
    websocket: ws::Connection<'a, S, ws::Open>,
    log:       Logger
}

impl<'a> Listener<'a, TlsStream> {
    pub fn open_wss(g: &Logger, u: Url, dom: &str, ctx: Arc<Tls>, tkn: &AccessToken) -> Result<Listener<'a, TlsStream>, Error<Void>> {
        let log = g.new(o!("context" => "Listener"));
        debug!(log, "open websocket (tls)"; "url" => u.as_str());
        let mut w = ws::Connection::open_tls(&log, &u, dom, &*ctx)?;
        w.set_read_timeout(Some(Duration::from_secs(5)))?;
        w.set_write_timeout(Some(Duration::from_secs(5)))?;
        let ws = w.handshake(&u, &[(AUTHORIZATION, Value::new(tkn.bearer.as_bytes()))])?;
        Ok(Listener {
            url:       u,
            domain:    String::from(dom),
            tls:       Some(ctx),
            websocket: ws,
            log:       log
        })
    }

    pub fn reconnect_wss(&mut self, t: &AccessToken) -> Result<(), Error<Void>> {
        let mut w = ws::Connection::open_tls(&self.log, &self.url, &self.domain, &*self.tls.as_ref().unwrap())?;
        w.set_read_timeout(Some(Duration::from_secs(5)))?;
        w.set_write_timeout(Some(Duration::from_secs(5)))?;
        let ws = w.handshake(&self.url, &[(AUTHORIZATION, Value::new(t.bearer.as_bytes()))])?;
        self.websocket = ws;
        Ok(())
    }

    pub fn set_read_timeout(&mut self, d: Option<Duration>) -> Result<(), Error<Void>> {
        self.websocket.set_read_timeout(d).map_err(From::from)
    }

    pub fn set_write_timeout(&mut self, d: Option<Duration>) -> Result<(), Error<Void>> {
        self.websocket.set_write_timeout(d).map_err(From::from)
    }
}

impl<'a> Listener<'a, TcpStream> {
    pub fn open_ws(g: &Logger, u: Url) -> Result<Listener<'a, TcpStream>, Error<Void>> {
        let log = g.new(o!("context" => "Listener"));
        debug!(log, "open websocket (tcp)"; "url" => u.as_str());
        let mut w = ws::Connection::open_tcp(&log, &u)?;
        w.set_read_timeout(Some(Duration::from_secs(15)))?;
        w.set_write_timeout(Some(Duration::from_secs(5)))?;
        let ws = w.handshake(&u, &[])?;
        Ok(Listener {
            url:       u,
            domain:    String::new(),
            tls:       None,
            websocket: ws,
            log:       log
        })
    }

    pub fn reconnect_wss(&mut self) -> Result<(), Error<Void>> {
        let mut w = ws::Connection::open_tcp(&self.log, &self.url)?;
        w.set_read_timeout(Some(Duration::from_secs(15)))?;
        w.set_write_timeout(Some(Duration::from_secs(5)))?;
        let ws = w.handshake(&self.url, &[])?;
        self.websocket = ws;
        Ok(())
    }
}

impl<'a, S: Stream> Listener<'a, S> {
    pub fn listen<T: FromJson>(&mut self) -> Result<T, Error<Void>> {
        debug!(self.log, "waiting for next message ...");
        let mut b = [0; 16];
        let     i = self.recv_loop(&mut b)?;
        let mut a = ReadIter::new(&mut self.websocket);
        let     x = {
            let     b = ReadIter::new(&b[0 .. i]).chain(&mut a);
            let mut d = Decoder::default(b);
            d.from_json()?
        };
        if let Some(e) = a.take_error() {
            error!(self.log, "read error: {}", e);
            match e {
                ReadError::InvalidUtf8 => Err(Error::Utf8),
                ReadError::Io(e)       => Err(Error::Io(e))
            }
        } else {
            Ok(x)
        }
    }

    fn recv_loop(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self.websocket.recv(buf) {
            Ok(x)  => return Ok(x.1),
            Err(e) => if !is_timeout(&e) { return Err(e) }
        }
        let mut counter = self.websocket.recv_ctr();
        loop {
            self.websocket.ping(&mut [])?;
            match self.websocket.recv(buf) {
                Err(ref e) if is_timeout(e) => {
                    if self.websocket.recv_ctr() == counter {
                        return Err(io::Error::new(ErrorKind::WouldBlock, "timeout"))
                    }
                    counter = self.websocket.recv_ctr()
                }
                Err(e) => return Err(e),
                Ok(x)  => return Ok(x.1)
            }
        }
    }
}

fn is_timeout(e: &io::Error) -> bool {
    e.kind() == ErrorKind::WouldBlock || e.kind() == ErrorKind::TimedOut
}

