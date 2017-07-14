use std::error;
use std::fmt;
use std::io;
use std::str;

use coax_api::types::ApiError;
use cookie;
use hyper;
use hyper::error::UriError;
use json::{EncodeError, DecodeError};
use url;
use websocket::WebSocketError;

// Error type ///////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct Void;

impl fmt::Display for Void {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "void")
    }
}

#[derive(Debug)]
pub enum Error<E> {
    Error(E),
    Api(ApiError<'static>),
    Cookie(cookie::ParseError),
    Hyper(hyper::Error),
    Io(io::Error),
    JsonD(DecodeError),
    JsonE(EncodeError),
    Message(&'static str),
    Status(hyper::StatusCode),
    Uri(hyper::error::UriError),
    Url(url::ParseError),
    Utf8(str::Utf8Error),
    Ws(WebSocketError)
}

impl<E: fmt::Display> fmt::Display for Error<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Api(ref e)     => write!(f, "api error: {}", e),
            Error::Cookie(ref e)  => write!(f, "cookie parse error: {}", e),
            Error::Error(ref e)   => write!(f, "error: {}", e),
            Error::Hyper(ref e)   => write!(f, "hyper error: {}", e),
            Error::Io(ref e)      => write!(f, "i/o error: {}", e),
            Error::JsonD(ref e)   => write!(f, "json decoding error: {}", e),
            Error::JsonE(ref e)   => write!(f, "json encoding error: {}", e),
            Error::Message(ref m) => write!(f, "{}", m),
            Error::Status(ref s)  => write!(f, "status error: {}", s),
            Error::Uri(ref e)     => write!(f, "hyper uri error: {}", e),
            Error::Url(ref e)     => write!(f, "url parse error: {}", e),
            Error::Utf8(ref e)    => write!(f, "utf-8 error: {}", e),
            Error::Ws(ref e)      => write!(f, "websocket error {}", e)
        }
    }
}

impl<E: fmt::Display + fmt::Debug> error::Error for Error<E> {
    fn description(&self) -> &str {
        match *self {
            Error::Error(_)   => "error",
            Error::Api(_)     => "api error",
            Error::Cookie(_)  => "cookie parse error",
            Error::Hyper(_)   => "hyper error",
            Error::Io(_)      => "i/o error",
            Error::JsonD(_)   => "json decoding error",
            Error::JsonE(_)   => "json encoding error",
            Error::Message(m) => m,
            Error::Status(_)  => "status error",
            Error::Uri(_)     => "hyper uri error",
            Error::Url(_)     => "url parse error",
            Error::Utf8(_)    => "utf-8 error",
            Error::Ws(_)      => "websocket error"
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Api(ref e)    => Some(e),
            Error::Cookie(ref e) => Some(e),
            Error::Hyper(ref e)  => Some(e),
            Error::Io(ref e)     => Some(e),
            Error::JsonD(ref e)  => Some(e),
            Error::JsonE(ref e)  => Some(e),
            Error::Uri(ref e)    => Some(e),
            Error::Url(ref e)    => Some(e),
            Error::Utf8(ref e)   => Some(e),
            Error::Ws(ref e)     => Some(e),
            _                    => None
        }
    }
}

impl<E> From<io::Error> for Error<E> {
    fn from(e: io::Error) -> Error<E> {
        Error::Io(e)
    }
}

impl<E> From<hyper::Error> for Error<E> {
    fn from(e: hyper::Error) -> Error<E> {
        Error::Hyper(e)
    }
}

impl<E> From<UriError> for Error<E> {
    fn from(e: UriError) -> Error<E> {
        Error::Uri(e)
    }
}

impl<E> From<url::ParseError> for Error<E> {
    fn from(e: url::ParseError) -> Error<E> {
        Error::Url(e)
    }
}

impl<E> From<DecodeError> for Error<E> {
    fn from(e: DecodeError) -> Error<E> {
        Error::JsonD(e)
    }
}

impl<E> From<EncodeError> for Error<E> {
    fn from(e: EncodeError) -> Error<E> {
        Error::JsonE(e)
    }
}

impl<E> From<ApiError<'static>> for Error<E> {
    fn from(e: ApiError<'static>) -> Error<E> {
        Error::Api(e)
    }
}

impl<E> From<WebSocketError> for Error<E> {
    fn from(e: WebSocketError) -> Error<E> {
        Error::Ws(e)
    }
}

impl<E> From<str::Utf8Error> for Error<E> {
    fn from(e: str::Utf8Error) -> Error<E> {
        Error::Utf8(e)
    }
}

impl<E> From<cookie::ParseError> for Error<E> {
    fn from(e: cookie::ParseError) -> Error<E> {
        Error::Cookie(e)
    }
}

