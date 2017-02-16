use std::marker::PhantomData;
use messages::{self, GenericMessage};
use uuid::Uuid;

pub struct Init;
pub struct Text;

pub struct Builder<T> {
    msg: GenericMessage,
    tag: PhantomData<T>
}

impl Builder<Init> {
    pub fn new() -> Builder<Init> {
        let     i = Uuid::new_v4();
        let mut m = GenericMessage::new();
        m.set_message_id(i.to_string());
        Builder {
            msg: m,
            tag: PhantomData
        }
    }
}

impl<T> Builder<T> {
    pub fn text<S: Into<String>>(mut self, txt: S) -> Builder<Text> {
        let mut t = messages::Text::new();
        t.set_content(txt.into());
        self.msg.set_text(t);
        self.cast()
    }

    pub fn finish(self) -> GenericMessage {
        self.msg
    }

    fn cast<U>(self) -> Builder<U> {
        Builder { msg: self.msg, tag: PhantomData }
    }
}

impl Builder<Text> {
    pub fn add_mention<S: Into<String>>(mut self, id: Uuid, name: S) -> Builder<Text> {
        let mut m = messages::Mention::new();
        m.set_user_id(id.to_string());
        m.set_user_name(name.into());
        self.msg.mut_text().mut_mention().push(m);
        self
    }
}
