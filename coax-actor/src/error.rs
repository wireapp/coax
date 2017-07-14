use std;
use std::io;
use std::str;
use std::time::Duration;

use cryptobox::CBoxError;
use cryptobox::store::file::{FileStore, FileStoreError};
use coax_api;
use coax_api::user;
use coax_api::message;
use coax_api::token;
use coax_api::types::UserId;
use coax_client::error as client;
use coax_data;
use hyper::StatusCode;
use json::{DecodeError, EncodeError};
use openssl;
use protobuf::ProtobufError;
use url;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Utf8(e: str::Utf8Error) {
            display("utf-8 error: {}", e)
            cause(e)
            from()
        }
        Url(e: url::ParseError) {
            display("url parse error: {}", e)
            cause(e)
            from()
        }
        Client(e: client::Error<client::Void>) {
            display("client error: {}", e)
            cause(e)
            from()
        }
        Io(e: io::Error) {
            display("i/o error: {}", e)
            cause(e)
            from()
        }
        JsonD(e: DecodeError) {
            display("error decoding json: {}", e)
            cause(e)
            from()
        }
        JsonE(e: EncodeError) {
            display("error encoding json: {}", e)
            cause(e)
            from()
        }
        Proto(e: ProtobufError) {
            display("protobuf error: {}", e)
            cause(e)
            from()
        }
        Cbox(e: CBoxError<FileStore>) {
            display("cbox error: {}", e)
            cause(e)
            from()
        }
        Decrypt(e: coax_api::conv::DecryptError<FileStore>) {
            display("decrypt error: {}", e)
            cause(e)
            from()
        }
        Encrypt(e: message::send::EncryptError<FileStore>) {
            display("encrypt error: {}", e)
            cause(e)
            from()
        }
        Database(e: coax_data::Error) {
            display("database error: {}", e)
            cause(e)
            from()
        }
        Profile(u: UserId, msg: &'static str) {
            display("profile error [{}]: {}", u, msg)
        }
        Message(msg: &'static str) {
            display("error: {}", msg)
        }
        Login(e: client::Error<user::login::Error>) {
            display("login error: {}", e)
            cause(e)
            from()
        }
        RegUser(e: client::Error<user::register::Error>) {
            display("user registration error: {}", e)
            cause(e)
            from()
        }
        RegClient(e: client::Error<coax_api::client::register::Error>) {
            display("client registration error: {}", e)
            cause(e)
            from()
        }
        Renew(e: client::Error<token::renew::Error>) {
            display("error renewing access: {}", e)
            cause(e)
            from()
        }
        Connect(e: client::Error<user::connect::Error>) {
            display("error connecting to user: {}", e)
            cause(e)
            from()
        }
        ConnectUpdate(e: client::Error<user::connect::update::Error>) {
            display("error updating user connection: {}", e)
            cause(e)
            from()
        }
        MsgSend(e: client::Error<message::send::Error>) {
            display("error sending message: {}", e)
            cause(e)
            from()
        }
        OpenSsl(e: openssl::error::ErrorStack) {
            display("openssl error: {}", e)
            cause(e)
            from()
        }
    }
}

impl From<FileStoreError> for Error {
    fn from(e: FileStoreError) -> Error {
        Error::Cbox(CBoxError::StorageError(e))
    }
}

pub fn is_unauthorised(err: &Error) -> bool {
    match *err {
        Error::Client(client::Error::Status(StatusCode::Unauthorized))        => true,
        Error::Login(client::Error::Status(StatusCode::Unauthorized))         => true,
        Error::RegUser(client::Error::Status(StatusCode::Unauthorized))       => true,
        Error::RegClient(client::Error::Status(StatusCode::Unauthorized))     => true,
        Error::Connect(client::Error::Status(StatusCode::Unauthorized))       => true,
        Error::ConnectUpdate(client::Error::Status(StatusCode::Unauthorized)) => true,
        Error::MsgSend(client::Error::Status(StatusCode::Unauthorized))       => true,
        _                                                                     => false
    }
}
