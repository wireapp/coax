extern crate chrono;
extern crate cookie;
extern crate coax_api_proto;
extern crate cryptobox;
#[macro_use]
extern crate json;
extern crate openssl;
extern crate proteus;
extern crate protobuf;
#[macro_use]
extern crate quick_error;
extern crate rustc_serialize;
extern crate uuid;

#[macro_use]
mod macros;
mod util;

pub mod user;
pub mod client;
pub mod message;
pub mod token;
pub mod types;
pub mod prekeys;
pub mod events;
pub mod conv;

pub fn new_session_id(u: &types::UserId, c: &types::ClientId) -> String {
    format!("{}_{}", u.to_string(), c.as_str())
}

pub use types::Page;
