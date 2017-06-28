#![feature(type_ascription, conservative_impl_trait)]
#![recursion_limit = "1024"]

extern crate app_dirs;
extern crate chrono;
extern crate coax_api;
extern crate coax_api_proto;
extern crate coax_client;
extern crate coax_data;
extern crate cryptobox;
extern crate futures;
extern crate hyper;
extern crate json;
extern crate native_tls;
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
extern crate tempdir;
extern crate toml;
extern crate url;

pub mod actor;
pub mod config;
pub mod error;
pub mod pkg;

pub use actor::{Actor, Delivery};
pub use error::Error;
pub use pkg::Pkg;
