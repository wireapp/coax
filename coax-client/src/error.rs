use std::error;
use std::fmt;
use std::io;
use std::str;

use coax_api::types::ApiError;
use coax_net::http::tls;
use coax_net::rpc;
use coax_ws::io as ws;
use json::DecodeError;

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
    Io(io::Error),
    Tls(tls::Error),
    Rpc(rpc::Error),
    WebSocket(ws::Error),
    Json(DecodeError),
    Response(u16, Vec<u8>),
    Message(&'static str),
    InvalidState,
    Utf8
}

impl<E: fmt::Display> fmt::Display for Error<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Error(ref e)       => write!(f, "error: {}", e),
            Error::Utf8               => write!(f, "utf-8 error"),
            Error::Api(ref e)         => write!(f, "api error: {}", e),
            Error::Io(ref e)          => write!(f, "i/o error: {}", e),
            Error::Tls(ref e)         => write!(f, "tls error: {}", e),
            Error::Rpc(ref e)         => write!(f, "rpc error: {}", e),
            Error::WebSocket(ref e)   => write!(f, "websocket error: {}", e),
            Error::Json(ref e)        => write!(f, "json decoding error: {}", e),
            Error::Message(ref m)     => write!(f, "{}", m),
            Error::InvalidState       => write!(f, "invalid state"),
            Error::Response(s, ref b) =>
                match str::from_utf8(b) {
                    Ok(msg) => write!(f, "http response: status: {}, msg: {}", s, msg),
                    Err(_)  => write!(f, "http response: status: {}", s)
                }
        }
    }
}

impl<E: fmt::Display + fmt::Debug> error::Error for Error<E> {
    fn description(&self) -> &str {
        match *self {
            Error::Error(_)     => "error",
            Error::Utf8         => "utf-8 error",
            Error::Api(_)       => "api error",
            Error::Io(_)        => "i/o error",
            Error::Tls(_)       => "tls error",
            Error::Rpc(_)       => "rpc error",
            Error::WebSocket(_) => "websocket error",
            Error::Json(_)      => "json decoding error",
            Error::Message(m)   => m,
            Error::InvalidState => "invalid state",
            Error::Response(..) => "http response"
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Api(ref e)       => Some(e),
            Error::Io(ref e)        => Some(e),
            Error::Tls(ref e)       => Some(e),
            Error::Rpc(ref e)       => Some(e),
            Error::WebSocket(ref e) => Some(e),
            Error::Json(ref e)      => Some(e),
            _                       => None
        }
    }
}

impl<E> From<io::Error> for Error<E> {
    fn from(e: io::Error) -> Error<E> {
        Error::Io(e)
    }
}

impl<E> From<tls::Error> for Error<E> {
    fn from(e: tls::Error) -> Error<E> {
        Error::Tls(e)
    }
}

impl<E> From<rpc::Error> for Error<E> {
    fn from(e: rpc::Error) -> Error<E> {
        Error::Rpc(e)
    }
}

impl<E> From<ws::Error> for Error<E> {
    fn from(e: ws::Error) -> Error<E> {
        Error::WebSocket(e)
    }
}

impl<E> From<DecodeError> for Error<E> {
    fn from(e: DecodeError) -> Error<E> {
        Error::Json(e)
    }
}

impl<E> From<ApiError<'static>> for Error<E> {
    fn from(e: ApiError<'static>) -> Error<E> {
        Error::Api(e)
    }
}
