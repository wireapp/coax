use chrono::{DateTime, Utc};
use coax_api::user::UserUpdate;
use coax_api::types::ConvId;
use coax_data::{User, Connection, Conversation, Message, MessageStatus, ConvStatus};

#[derive(Debug)]
pub enum Pkg {
    Message(Message<'static>),
    MessageUpdate(ConvId, String, DateTime<Utc>, MessageStatus),
    Conversation(Conversation<'static>),
    Contact(User<'static>, Connection),
    MembersChange(ConvStatus, DateTime<Utc>, ConvId, Vec<User<'static>>, User<'static>),
    UserUpdate(UserUpdate<'static>),
    Disconnected,
    Connected
}
