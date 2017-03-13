#![feature(type_ascription)]
#![recursion_limit = "1024"]

extern crate app_dirs;
extern crate chrono;
extern crate coax_api;
extern crate coax_api_proto;
extern crate coax_client;
extern crate coax_data;
extern crate coax_net;
extern crate coax_ws;
extern crate cookie;
extern crate cryptobox;
extern crate json;
extern crate openssl;
extern crate proteus;
extern crate protobuf;
#[macro_use]
extern crate quick_error;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate tempdir;
extern crate toml;
extern crate url;

pub mod actor;
pub mod config;
pub mod error;
pub mod pkg;

pub use actor::{Actor, Inbox, Delivery};
pub use error::Error;
pub use pkg::Pkg;
