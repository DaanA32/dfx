use crate::message::Message;


pub trait MessageStore {
    fn create(&self, message_type: &str) -> Result<Message, ()>;
}
