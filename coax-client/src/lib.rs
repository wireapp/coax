#![feature(type_ascription)]

extern crate coax_api;
extern crate coax_net;
extern crate coax_ws;
extern crate json;
#[macro_use]
extern crate slog;
extern crate url;

pub mod client;
pub mod listen;
pub mod error;

pub use client::Client;
pub use listen::Listener;
pub use error::Error;
