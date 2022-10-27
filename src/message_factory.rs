use crate::field_map::Group;
use crate::field_map::Tag;
use crate::message::Message;

pub trait MessageFactory {
    fn get_supported_begin_strings(&self) -> Vec<String>;
    fn create(&self, begin_string: &str, msg_type: &str) -> Result<Message, MessageFactoryError>;
    fn create_group(&self, begin_string: &str, msg_type: &str, group_counter_tag: Tag) -> Group;
}

pub enum MessageFactoryError {
    UnsupportedBeginString(String),
    UnsupportedMsgType(String),
}
