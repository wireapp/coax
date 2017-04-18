use std::io::{self, ErrorKind, Read, Write};
use std::mem;
use std::net::TcpStream;

use buf::Buf;
use url::Url;
use self::header::{Response, Name, Value};
use slog::Logger;

pub mod header;
pub mod tls;
pub mod transfer;

use self::tls::TlsStream;

// HTTP Method //////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Method {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Trace,
    Options,
    Connect,
    Patch
}

impl Method {
    pub fn as_str(&self) -> &str {
        match *self {
            Method::Get     => "GET",
            Method::Head    => "HEAD",
            Method::Post    => "POST",
            Method::Put     => "PUT",
            Method::Delete  => "DELETE",
            Method::Trace   => "TRACE",
            Method::Options => "OPTIONS",
            Method::Connect => "CONNECT",
            Method::Patch   => "PATCH"
        }
    }
}

// Stream interface /////////////////////////////////////////////////////////

pub trait Stream: Read + Write + AsTcp { }

impl Stream for TcpStream {}
impl Stream for TlsStream {}

pub trait AsTcp {
    fn as_tcp(&self) -> &TcpStream;
}

impl AsTcp for TcpStream {
    fn as_tcp(&self) -> &TcpStream {
        self
    }
}

// HttpStream ///////////////////////////////////////////////////////////////

pub struct HttpStream<S> {
    stream: S,
    rbuf:   Buf, // read buffer
    wbuf:   Buf, // write buffer
    log:    Logger
}

impl<S: Stream> HttpStream<S> {
    pub fn new(g: &Logger, s: S) -> HttpStream<S> {
        HttpStream {
            stream: s,
            rbuf:   Buf::new(0x40000), // 256 KiB
            wbuf:   Buf::new(0x8000),  //  32 KiB
            log:    g.new(o!("context" => "HttpStream"))
        }
    }

    pub fn stream(&self) -> &S {
        &self.stream
    }

    pub fn set_stream(&mut self, s: S) -> S {
        let old = mem::replace(&mut self.stream, s);
        self.rbuf.reset();
        self.wbuf.reset();
        old
    }

    pub fn request<'a, I>(&mut self, m: Method, u: &Url, h: I) -> io::Result<()>
        where I: Iterator<Item=&'a (Name<'a>, Value<'a>)>
    {
        trace!(self.log, "request"; "method" => m.as_str(), "url" => %u);

        self.write(m.as_str().as_bytes())?;
        self.write(b" ")?;
        self.write(u.path().as_bytes())?;
        if let Some(q) = u.query() {
            self.write(b"?")?;
            self.write(q.as_bytes())?;
        }
        self.write(b" HTTP/1.1\r\n")?;
        self.write_headers(u, h)?;
        self.write(b"\r\n")?;
        self.flush()?;

        trace!(self.log, "header sent"; "url" => %u);

        Ok(())
    }

    pub fn response(&mut self, r: &mut Response) -> io::Result<()> {
        trace!(self.log, "response");
        self.rbuf.reset();
        loop {
            if self.top_up()? == 0 {
                return Err(io::Error::new(ErrorKind::UnexpectedEof, "incomplete response header"))
            }
            if let Some(i) = r.parse(self.rbuf.as_ref())? {
                self.rbuf.consume(i);
                break
            }
            if self.rbuf.capacity() == 0 {
                error!(self.log, "response header too large");
                return Err(io::Error::new(io::ErrorKind::Other, "response header too large"))
            }
        }
        Ok(())
    }

    pub fn iter(&mut self) -> Iter<S> {
        Iter::new(self)
    }

    fn write_headers<'a, I>(&mut self, u: &Url, headers: I) -> io::Result<()>
        where I: Iterator<Item=&'a (Name<'a>, Value<'a>)>
    {
        trace!(self.log, "write_headers"; "url" => %u);
        if let Some(h) = u.host_str() { // Add "Host" header.
            self.write(b"Host: ")?;
            self.write(h.as_bytes())?;
            if let Some(p) = u.port() {
                self.write(b":")?;
                self.write(p.to_string().as_bytes())?;
            }
            self.write(b"\r\n")?;
        }
        for &(ref name, ref val) in headers {
            self.write(name.as_str().as_bytes())?;
            self.write(b": ")?;
            self.write(val.as_bytes())?;
            self.write(b"\r\n")?;
        }
        Ok(())
    }

    fn top_up(&mut self) -> io::Result<usize> {
        trace!(self.log, "top_up");
        let n = self.stream.read(self.rbuf.as_mut())?;
        if self.rbuf.extend(n) != n {
            error!(self.log, "buffer too small");
            Err(io::Error::new(ErrorKind::Other, "buffer too small to extend"))
        } else {
            trace!(self.log, "{} bytes read from socket", n);
            Ok(n)
        }
    }
}

impl<S: Write> Write for HttpStream<S> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        trace!(self.log, "write bytes"; "data-size" => buf.len());
        if buf.len() == 0 {
            return Ok(0)
        }
        let n = self.wbuf.write(buf)?;
        if n != 0 {
            Ok(n)
        } else {
            self.flush()?;
            self.wbuf.write(buf)
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        trace!(self.log, "flush");
        io::copy(&mut self.wbuf, &mut self.stream)?;
        self.wbuf.reset();
        Ok(())
    }
}

impl<S: Stream> Read for HttpStream<S> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        trace!(self.log, "read bytes"; "buf-size" => buf.len());
        if self.rbuf.is_empty() {
            self.rbuf.reset();
            self.top_up()?;
        }
        self.rbuf.read(buf)
    }
}

// HttpStream Iterator //////////////////////////////////////////////////////

pub struct Iter<'a, S: 'a> {
    stream: &'a mut HttpStream<S>,
    error:  Option<io::Error>
}

impl<'a, S> Iter<'a, S> {
    pub fn new(s: &'a mut HttpStream<S>) -> Iter<'a, S> {
        Iter {
            stream: s,
            error: None
        }
    }

    pub fn error(&self) -> Option<&io::Error> {
        self.error.as_ref()
    }

    pub fn take_error(&mut self) -> Option<io::Error> {
        self.error.take()
    }
}

impl<'a, S: Stream> Iterator for Iter<'a, S> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        match self.stream.rbuf.take1() {
            None => {
                self.stream.rbuf.reset();
                match self.stream.top_up() {
                    Err(e) => {
                        self.error = Some(e);
                        return None
                    }
                    Ok(0) => None,
                    Ok(_) => self.next()
                }
            }
            val => val
        }
    }
}

