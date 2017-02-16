use std::cmp::min;
use std::fmt;
use std::io::{self, ErrorKind, Write};
use std::num::ParseIntError;
use std::str::{self, FromStr};
use httparse::Header;
use unicase::UniCase;
use httparse::{self, Status, EMPTY_HEADER};
use buf::Buf;

pub use cookie::Cookie;

/// Header name.
///
/// With case-insensitive equality comparison.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Name<'a>(UniCase<&'a str>);

impl<'a> Name<'a> {
    pub fn new(s: &'a str) -> Name<'a> {
        Name(UniCase(s))
    }

    pub fn as_str(&self) -> &str {
        *self.0
    }
}

/// Header value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Value<'a>(&'a [u8]);

impl<'a> Value<'a> {
    pub fn new(s: &'a [u8]) -> Value<'a> {
        Value(s)
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0
    }

    pub fn as_str(&self) -> Option<&str> {
        str::from_utf8(self.0).ok()
    }
}

pub fn eq_ci(a: &Value, b: &Value) -> bool {
    a.as_str().and_then(|ua| {
        b.as_str().and_then(|ub| {
            Some(UniCase(ua) == UniCase(ub))
        })
    }).unwrap_or(false)
}

pub const ACCEPT: Name<'static> = Name(UniCase("Accept"));
pub const ACCEPT_ENCODING: Name<'static> = Name(UniCase("Accept-Encoding"));
pub const ASSET_TOKEN: Name<'static> = Name(UniCase("Asset-Token"));
pub const AUTHORIZATION: Name<'static> = Name(UniCase("Authorization"));
pub const CONTENT_TYPE: Name<'static> = Name(UniCase("Content-Type"));
pub const CONTENT_LENGTH: Name<'static> = Name(UniCase("Content-Length"));
pub const CONTENT_ENCODING: Name<'static> = Name(UniCase("Content-Encoding"));
pub const COOKIE: Name<'static> = Name(UniCase("Cookie"));
pub const LOCATION: Name<'static> = Name(UniCase("Location"));
pub const SET_COOKIE: Name<'static> = Name(UniCase("Set-Cookie"));
pub const TRANSFER_ENCODING: Name<'static> = Name(UniCase("Transfer-Encoding"));
pub const HOST: Name<'static> = Name(UniCase("Host"));
pub const USER_AGENT: Name<'static> = Name(UniCase("User-Agent"));
pub const UPGRADE: Name<'static> = Name(UniCase("Upgrade"));
pub const CONNECTION: Name<'static> = Name(UniCase("Connection"));
pub const SEC_WEBSOCKET_KEY: Name<'static> = Name(UniCase("Sec-WebSocket-Key"));
pub const SEC_WEBSOCKET_VERSION: Name<'static> = Name(UniCase("Sec-WebSocket-Version"));
pub const SEC_WEBSOCKET_ACCEPT: Name<'static> = Name(UniCase("Sec-WebSocket-Accept"));
pub const SEC_WEBSOCKET_EXTENSIONS: Name<'static> = Name(UniCase("Sec-WebSocket-Extensions"));
pub const SEC_WEBSOCKET_PROTOCOL: Name<'static> = Name(UniCase("Sec-WebSocket-Protocol"));

pub const APPLICATION_JSON: Value<'static> = Value(b"application/json");
pub const APPLICATION_PROTOBUF: Value<'static> = Value(b"application/x-protobuf");
pub const CHUNKED: Value<'static> = Value(b"chunked");
pub const GZIP: Value<'static> = Value(b"gzip");
pub const ZLIB: Value<'static> = Value(b"zlib");
pub const DEFLATE: Value<'static> = Value(b"deflate");
pub const WEBSOCKET: Value<'static> = Value(b"websocket");
pub const UPGRADE_VAL: Value<'static> = Value(b"upgrade");
pub const VERSION_13: Value<'static> = Value(b"13");

// Content-Type /////////////////////////////////////////////////////////////

/// Some important content-type values.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ContentType<'a> {
    AppJson,
    AppProtobuf,
    Other(Value<'a>)
}

impl<'a> ContentType<'a> {
    fn from_bytes(s: &[u8]) -> ContentType {
        match str::from_utf8(s) {
            Err(_) => ContentType::Other(Value(s)),
            Ok(c)  => {
                let ct = UniCase(c);
                if ct == UniCase("application/json") {
                    return ContentType::AppJson
                }
                if ct == UniCase("application/x-protobuf") {
                    return ContentType::AppProtobuf
                }
                ContentType::Other(Value(s))
            }
        }
    }

    pub fn as_value(&self) -> Value {
        match *self {
            ContentType::AppJson      => APPLICATION_JSON,
            ContentType::AppProtobuf  => APPLICATION_PROTOBUF,
            ContentType::Other(ref v) => Value(v.0)
        }
    }
}

impl<'a> fmt::Display for ContentType<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            ContentType::AppJson      => write!(f, "application/json"),
            ContentType::AppProtobuf  => write!(f, "application/x-protobuf"),
            ContentType::Other(ref v) =>
                match str::from_utf8(v.as_bytes()) {
                    Ok(s)  => write!(f, "{}", s),
                    Err(_) => write!(f, "{:?}", v.as_bytes())
                }
        }
    }
}

// Content-Encoding /////////////////////////////////////////////////////////

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ContentEncoding {
    Deflate,
    Gzip,
    Zlib
}

impl ContentEncoding {
    fn from_bytes(s: &[u8]) -> Option<ContentEncoding> {
        match str::from_utf8(s) {
            Err(_) => None,
            Ok(s) => {
                let ce = UniCase(s);
                if ce == UniCase("gzip") {
                    return Some(ContentEncoding::Gzip)
                }
                if ce == UniCase("zlib") {
                    return Some(ContentEncoding::Zlib)
                }
                if ce == UniCase("deflate") {
                    return Some(ContentEncoding::Deflate)
                }
                None
            }
        }
    }

    pub fn as_value(&self) -> Value {
        match *self {
            ContentEncoding::Deflate => DEFLATE,
            ContentEncoding::Zlib    => ZLIB,
            ContentEncoding::Gzip    => GZIP
        }
    }
}

impl fmt::Display for ContentEncoding {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            ContentEncoding::Deflate => write!(f, "deflate"),
            ContentEncoding::Zlib    => write!(f, "zlib"),
            ContentEncoding::Gzip    => write!(f, "gzip")
        }
    }
}

// HTTP Response ////////////////////////////////////////////////////////////

pub struct Response<'a> {
    status:  u16,
    buffer:  Buf,
    headers: [Header<'a>; 64],
    length:  usize
}

impl<'a> Response<'a> {
    pub fn new() -> Response<'a> {
        Response {
            status:  0,
            buffer:  Buf::new(0x20000),
            headers: [EMPTY_HEADER; 64],
            length:  0
        }
    }

    pub fn parse(&mut self, bytes: &[u8]) -> io::Result<Option<usize>> {
        self.buffer.reset();
        let n = self.buffer.capacity();
        self.buffer.write_all(&bytes[0 .. min(n, bytes.len())])?;
        let mut r = httparse::Response::new(&mut self.headers);
        match r.parse(self.buffer.as_ref()) {
            Ok(Status::Complete(i)) =>
                match r.code {
                    None    => Err(io::Error::new(ErrorKind::Other, "missing response status")),
                    Some(c) => {
                        self.status = c;
                        self.length = r.headers.len();
                        Ok(Some(i))
                    }
                },
            Ok(Status::Partial) => Ok(None),
            Err(_) => Err(io::Error::new(io::ErrorKind::InvalidData, "malformed response header"))
        }
    }

    pub fn status(&self) -> u16 {
        self.status
    }

    pub fn header(&self, n: Name) -> Option<Value> {
        (&self.headers[.. self.length])
            .iter()
            .find(|h| Name::new(h.name) == n)
            .map(|h| Value(h.value))
    }

    pub fn from_header<T: FromStr>(&self, n: Name) -> Option<Result<T, ParseError<T::Err>>> {
        self.header(n).map(|v| {
            match str::from_utf8(v.as_bytes()) {
                Ok(s)  => s.parse().or_else(|e| Err(ParseError::Parse(e))),
                Err(e) => Err(ParseError::Utf8(e))
            }
        })
    }

    pub fn content_length(&self) -> Option<Result<usize, ParseError<ParseIntError>>> {
        self.from_header(CONTENT_LENGTH)
    }

    pub fn content_type(&self) -> Option<ContentType> {
        self.header(CONTENT_TYPE).map(|v| ContentType::from_bytes(v.0))
    }

    pub fn content_encoding(&self) -> Option<ContentEncoding> {
        self.header(CONTENT_ENCODING).and_then(|v| ContentEncoding::from_bytes(v.0))
    }

    pub fn is_chunked(&self) -> bool {
        self.header(TRANSFER_ENCODING).map(|v| v.as_bytes() == b"chunked").unwrap_or(false)
    }

    pub fn cookie(&'a self, name: &str) -> Option<Cookie<'a>> {
        (&self.headers[.. self.length])
            .iter()
            .filter(|h| Name::new(h.name) == SET_COOKIE)
            .filter_map(|h| str::from_utf8(h.value).ok().and_then(|s| Cookie::parse(s).ok()))
            .filter(|c| c.name() == name)
            .next()
    }

    pub fn cookies(&'a self, out: &mut Vec<Cookie<'a>>) {
        for c in (&self.headers[.. self.length])
            .iter()
            .filter(|h| Name::new(h.name) == SET_COOKIE)
            .filter_map(|h| str::from_utf8(h.value).ok().and_then(|s| Cookie::parse(s).ok()))
        {
            out.push(c)
        }
    }
}

// Error type ///////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub enum ParseError<E> {
    Utf8(str::Utf8Error),
    Parse(E)
}

