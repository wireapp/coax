use chrono::{DateTime, UTC};
use coax_api::types::ConvId;
use coax_data::{User, Connection, Conversation, Message, MessageStatus};

#[derive(Debug)]
pub enum Pkg {
    Message(Message<'static>),
    MessageUpdate(ConvId, String, DateTime<UTC>, MessageStatus),
    Conversation(Conversation<'static>),
    Contact(User<'static>, Connection),
    MembersAdd(DateTime<UTC>, ConvId, Vec<User<'static>>),
    Disconnected,
    Connected,
    Fin
}
