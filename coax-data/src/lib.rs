#![feature(type_ascription)]

extern crate chrono;
extern crate coax_api;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
#[macro_use]
extern crate slog;
#[macro_use]
extern crate quick_error;

pub mod db;
pub mod error;
pub mod profiles;
mod model;
mod schema;
mod util;

pub use db::Database;
pub use error::Error;
pub use model::{User, Client, Conversation, Connection};
pub use model::{Message, MessageData, MessageStatus, NewMessage};
pub use model::{QueueItem, QueueItemData, QueueItemType, NewQueueItem};
