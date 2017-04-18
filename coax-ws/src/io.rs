use std::cmp::min;
use std::io::{self, ErrorKind, Read, Write};
use std::marker::PhantomData;
use std::net::{ToSocketAddrs, TcpStream};
use std::str;
use std::time::Duration;

use rand::{Rng, thread_rng};
use openssl;
use openssl::hash::{Hasher, MessageDigest};
use rustc_serialize::base64::{self, ToBase64};
use slog::Logger;
use url::Url;

use coax_net::http::{Method, Stream, HttpStream};
use coax_net::http::header::{self, Name, Value, Response};
use coax_net::http::transfer::{ChunkReader, LenReader};
use coax_net::http::tls::{self, Tls, TlsStream};
use frame::{Header, OpCode};

pub struct Init;
pub struct Open;

struct Recv(usize, bool);

pub struct Connection<'a, S, T> {
    stream:   HttpStream<S>,
    response: Response<'a>,
    msgtype:  Option<OpCode>, // message type (set during continuation frames)
    recvctr:  usize,
    log:      Logger,
    pending:  Option<Recv>,
    state:    PhantomData<T>
}

impl<'a> Connection<'a, TcpStream, Init> {
    pub fn open_tcp<A: ToSocketAddrs>(g: &Logger, a: A) -> Result<Connection<'a, TcpStream, Init>, Error> {
        let tcp = TcpStream::connect(a)?;
        let log = g.new(o!("context" => "Connection<Tcp>"));
        Ok(Connection {
            stream:   HttpStream::new(&log, tcp),
            response: Response::new(),
            msgtype:  None,
            recvctr:  0,
            log:      log,
            pending:  None,
            state:    PhantomData
        })
    }
}

impl<'a> Connection<'a, TlsStream, Init> {
    pub fn open_tls<A: ToSocketAddrs>(g: &Logger, a: A, domain: &str, ctx: &Tls) -> Result<Connection<'a, TlsStream, Init>, Error> {
        let tcp = TcpStream::connect(a)?;
        let tls = TlsStream::new(ctx, domain, tcp)?;
        let log = g.new(o!("context" => "Connection<Tls>"));
        Ok(Connection {
            stream:   HttpStream::new(&log, tls),
            response: Response::new(),
            msgtype:  None,
            recvctr:  0,
            log:      log,
            pending:  None,
            state:    PhantomData
        })
    }
}

impl<'a, S: Stream, T> Connection<'a, S, T> {
    pub fn set_read_timeout(&mut self, d: Option<Duration>) -> io::Result<()> {
        trace!(self.log, "set_read_timeout");
        self.stream.stream().as_tcp().set_read_timeout(d)
    }

    pub fn set_write_timeout(&mut self, d: Option<Duration>) -> io::Result<()> {
        trace!(self.log, "set_write_timeout");
        self.stream.stream().as_tcp().set_write_timeout(d)
    }
}

impl<'a, S: Stream> Connection<'a, S, Init> {
    pub fn handshake(mut self, u: &Url, headers: &[(Name, Value)]) -> Result<Connection<'a, S, Open>, Error> {
        trace!(self.log, "handshake"; "url" => %u);
        let n = nonce();
        let h = [(header::SEC_WEBSOCKET_KEY, header::Value::new(n.as_bytes()))];
        let s = HANDSHAKE_HEADERS.iter().chain(headers.iter()).chain(h.iter());
        self.stream.request(Method::Get, u, s)?;
        self.stream.response(&mut self.response)?;
        match self.response.status() {
            101 => {
                self.upgrade(n.as_bytes())?;
                Ok(Connection {
                    stream:   self.stream,
                    response: self.response,
                    msgtype:  self.msgtype,
                    recvctr:  self.recvctr,
                    log:      self.log,
                    pending:  self.pending,
                    state:    PhantomData
                })
            }
            _ => {
                error!(self.log, "handshake"; "status" => self.response.status());
                if self.response.is_chunked() {
                    let mut r = ChunkReader::new(&self.log, &mut self.stream);
                    if cfg!(feature = "max_level_debug") || cfg!(feature = "max_level_trace") {
                        io::copy(&mut r, &mut io::stderr())?;
                    } else {
                        io::copy(&mut r, &mut io::sink())?;
                    }
                    return Err(Error::Handshake(self.response.status(), "unexpected response status"))
                }
                if let Some(Ok(n)) = self.response.content_length() {
                    let mut r = LenReader::new(&self.log, &mut self.stream, n);
                    if cfg!(feature = "max_level_debug") || cfg!(feature = "max_level_trace") {
                        io::copy(&mut r, &mut io::stderr())?;
                    } else {
                        io::copy(&mut r, &mut io::sink())?;
                    }
                }
                Err(Error::Handshake(self.response.status(), "unexpected response status"))
            }
        }
    }

    fn upgrade(&mut self, nonce: &[u8]) -> Result<(), Error> {
        trace!(self.log, "upgrade");
        let upgrade = self.response.header(header::UPGRADE);
        if !upgrade.as_ref().map(|a| header::eq_ci(a, &header::WEBSOCKET)).unwrap_or(true) {
            error!(self.log, "invalid upgrade header"; "header" => ?upgrade.as_ref().and_then(Value::as_str));
            return Err(Error::Handshake(101, "invalid response header 'upgrade'"))
        }
        let connection = self.response.header(header::CONNECTION);
        if !connection.as_ref().map(|a| header::eq_ci(&a, &header::UPGRADE_VAL)).unwrap_or(true) {
            error!(self.log, "invalid connection header"; "header" => ?connection.as_ref().and_then(Value::as_str));
            return Err(Error::Handshake(101, "invalid response header 'connection'"))
        }
        if let Some(h) = self.response.header(header::SEC_WEBSOCKET_ACCEPT) {
            if !nonce_check(h.as_bytes(), nonce)? {
                error!(self.log, "invalid sec-websocket-accept value");
                return Err(Error::Handshake(101, "invalid 'sec-websocket-accept' value"))
            }
        } else {
            error!(self.log, "missing sec-websocket-accept header");
            return Err(Error::Handshake(101, "missign response header 'sec-websocket-accept'"))
        }
        if self.response.header(header::SEC_WEBSOCKET_EXTENSIONS).is_some() {
            error!(self.log, "unexpected sec-websocket-extensions header");
            return Err(Error::Handshake(101, "unexpected header 'sec-websocket-extensions'"))
        }
        if self.response.header(header::SEC_WEBSOCKET_PROTOCOL).is_some() {
            error!(self.log, "unexpected sec-websocket-protocol header");
            return Err(Error::Handshake(101, "unexpected header 'sec-websocket-protocol'"))
        }
        debug!(self.log, "handshake ok");
        Ok(())
    }
}

impl<'a, S: Stream> Connection<'a, S, Open> {
    pub fn recv_ctr(&self) -> usize {
        self.recvctr
    }

    pub fn ping(&mut self, buf: &mut [u8]) -> io::Result<()> {
        self.send(OpCode::Ping, true, buf)
    }

    pub fn send(&mut self, c: OpCode, fin: bool, buf: &mut [u8]) -> io::Result<()> {
        trace!(self.log, "send"; "code" => c.as_str(), "fin" => fin);
        let k = thread_rng().next_u32();
        mask_data(buf, k);
        let mut h = Header::empty();
        h.set_opcode(c);
        h.set_fin(fin);
        h.set_len(buf.len() as u64);
        h.mask(k);
        h.write(&mut self.stream)?;
        self.stream.write_all(buf)?;
        self.stream.flush()
    }

    pub fn close(mut self) -> io::Result<()> {
        trace!(self.log, "close");
        self.send(OpCode::Close, true, &mut [])?;
        Ok(())
    }

    pub fn recv(&mut self, buf: &mut [u8]) -> io::Result<(OpCode, usize, bool)> {
        trace!(self.log, "receive");
        if let Some(Recv(len, fin)) = self.pending.take() {
            let n = min(buf.len(), len);
            self.stream.read_exact(&mut buf[0 .. n])?;
            if n < len {
                self.pending = Some(Recv(len - n, fin));
                return Ok((OpCode::Cont, n, false))
            } else {
                return Ok((OpCode::Cont, n, fin))
            }
        }
        loop {
            let hd = self.header()?;
            match hd.opcode() {
                c@OpCode::Ping | c@OpCode::Pong | c@OpCode::Close => {
                    self.recvctr = self.recvctr.wrapping_add(1);
                    debug_assert!(hd.len() < 126);
                    let mut d = [0; 125];
                    let     n = hd.len() as usize;
                    self.stream.read_exact(&mut d[0 .. n])?;
                    match c {
                        OpCode::Ping  => self.send(OpCode::Pong, true, &mut d[0 .. n])?,
                        OpCode::Close => {
                            let _ = self.send(OpCode::Close, true, &mut d[0 .. n]);
                            return Ok((OpCode::Close, 0, true))
                        }
                        _ => ()
                    }
                }
                OpCode::Reserved => {
                    return Err(io::Error::new(ErrorKind::InvalidData, "reserved opcode"))
                }
                c@OpCode::Text | c@OpCode::Binary | c@OpCode::Cont => {
                    if c != OpCode::Cont && self.msgtype.is_some() {
                        return Err(io::Error::new(ErrorKind::InvalidData, "unexpected non-continuation"))
                    }
                    if c == OpCode::Cont && self.msgtype.is_none() {
                        return Err(io::Error::new(ErrorKind::InvalidData, "unexpected continuation"))
                    }
                    self.recvctr = self.recvctr.wrapping_add(1);
                    let n = min(buf.len(), hd.len() as usize);
                    self.stream.read_exact(&mut buf[0 .. n])?;
                    if hd.fin() {
                        self.msgtype = None
                    } else {
                        self.msgtype = Some(c.clone())
                    }
                    if n < hd.len() as usize {
                        self.pending = Some(Recv(hd.len() as usize - n, hd.fin()));
                        return Ok((c, n, false))
                    } else {
                        return Ok((c, n, hd.fin()))
                    }
                }
            }
        }
    }

    fn header(&mut self) -> io::Result<Header> {
        let h = Header::read(&mut self.stream)?;
        trace!(self.log, "header: {}", h);
        if !h.is_valid() {
            Err(io::Error::new(ErrorKind::InvalidData, "invalid header"))
        } else {
            Ok(h)
        }
    }
}

impl<'a, S: Stream> Read for Connection<'a, S, Open> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let res = self.recv(buf)?;
        Ok(res.1)
    }
}

fn nonce() -> String {
    let nonce = thread_rng().gen::<[u8; 16]>();
    nonce.as_ref().to_base64(base64::STANDARD)
}

fn nonce_check(theirs: &[u8], ours: &[u8]) -> Result<bool, Error> {
    let mut h = Hasher::new(MessageDigest::sha1())?;
    h.update(ours)?;
    h.update(MAGIC_UUID.as_bytes())?;
    let sha1 = h.finish2()?;
    let b64  = sha1.to_base64(base64::STANDARD);
    Ok(b64.as_bytes() == theirs) // TODO: trim theirs
}

fn mask_data(d: &mut [u8], k: u32) {
    for i in 0 .. d.len() {
        d[i] ^= octet(k, i)
    }
}

fn octet(x: u32, i: usize) -> u8 {
    match i % 4 {
        0 => (x >> 24) as u8,
        1 => (x >> 16) as u8,
        2 => (x >>  8) as u8,
        _ => x as u8
    }
}

const HANDSHAKE_HEADERS: &'static [(Name<'static>, Value<'static>)] = &[
    (header::UPGRADE, header::WEBSOCKET),
    (header::CONNECTION, header::UPGRADE_VAL),
    (header::SEC_WEBSOCKET_VERSION, header::VERSION_13)
];

const MAGIC_UUID: &'static str = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";

// Error type ///////////////////////////////////////////////////////////////

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Io(e: io::Error) {
            display("i/o error: {}", e)
            cause(e)
            from()
        }
        Tls(e: tls::Error) {
            display("tls error: {}", e)
            cause(e)
            from()
            from(e: openssl::error::ErrorStack) -> (tls::Error::Openssl(e))
        }
        Handshake(s: u16, msg: &'static str) {
            display("handshake error: status={}, msg={}", s, msg)
        }
        Msg(m: &'static str) {
            display("error: {}", m)
        }
    }
}

// Tests ////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    extern crate slog_term;

    use frame::OpCode;
    use url::Url;
    use slog::{Logger, DrainExt};
    use std::io::Read;
    use std::str::{self, FromStr};
    use super::*;

    fn case_count(g: &Logger) -> usize {
        let     u = Url::parse("ws://127.0.0.1:9001/getCaseCount").unwrap();
        let mut c = Connection::open_tcp(g, &u).unwrap();
        c.handshake(&u, &[]).unwrap();
        let mut buf = [0; 8];
        let n = c.read(&mut buf).unwrap();
        usize::from_str(str::from_utf8(&buf[0 .. n]).ok().unwrap_or_default()).unwrap()
    }

    fn update_reports(g: &Logger) {
        let     u = Url::parse("ws://127.0.0.1:9001/updateReports?agent=coax").unwrap();
        let mut c = Connection::open_tcp(g, &u).unwrap();
        c.handshake(&u, &[]).unwrap();
        c.send(OpCode::Close, true, &mut []).unwrap()
    }

    fn test_case(g: &Logger, url: &mut Url, buf: &mut [u8], i: usize) -> Result<(), Error> {
        debug!(g, "test"; "number" => i);
        url.set_query(Some(format!("case={}&agent=coax", i).as_str()));
        let mut c = Connection::open_tcp(g, url.clone())?;
        c.handshake(&url, &[])?;
        let mut t = None;
        let mut m = Vec::new();
        loop {
            let (k, n, fin) = c.recv(buf)?;
            if k == OpCode::Close {
                break
            }
            if k != OpCode::Cont {
                m.clear();
                t = Some(k.clone())
            }
            m.extend_from_slice(&buf[0 .. n]);
            if fin {
                if t == Some(OpCode::Text) && str::from_utf8(m.as_slice()).is_err() {
                    break
                }
                c.send(t.clone().unwrap(), true, m.as_mut_slice())?;
            }
        }
        Ok(())
    }

    #[test]
    fn autobahn() {
        let  root = Logger::root(slog_term::streamer().compact().build().fuse(), o!());
        let mut u = Url::parse("ws://127.0.0.1:9001/runCase").unwrap();
        let tests = case_count(&root);
        info!(root, "running {} test cases", tests);
        let mut m = vec![0; 4096];
        for i in 1 .. tests + 1 {
            if let Err(e) = test_case(&root, &mut u, &mut m, i) {
                error!(root, "error running test"; "number" => i, "error" => %e)
            }
        }
        update_reports(&root)
    }
}

