use crate::field_map::Group;
use crate::field_map::Tag;
use crate::message::Message;
use std::fmt::Debug;

pub trait MessageFactory: Debug {
    fn get_supported_begin_strings(&self) -> Vec<String>;
    fn create(&self, begin_string: &str, msg_type: &str) -> Result<Message, MessageFactoryError>;
    fn create_group(&self, begin_string: &str, msg_type: &str, group_counter_tag: Tag) -> Group;
}

#[derive(Clone, Debug)]
pub enum MessageFactoryError {
    UnsupportedBeginString(String),
    UnsupportedMsgType(String),
}

// pub struct DefaultMessageFactory {
//     factories: Hash
// }
