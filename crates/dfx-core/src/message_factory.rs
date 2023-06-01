use crate::field_map::Group;
use crate::field_map::Tag;
use crate::message::Message;
use crate::tags;
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::Mutex;

pub trait MessageFactory: Debug + Send {
    fn get_supported_begin_strings(&self) -> Vec<String>;
    fn create(&self, begin_string: &str, msg_type: &str) -> Result<Message, MessageFactoryError>;
    fn create_group(&self, begin_string: &str, msg_type: &str, group_counter_tag: Tag) -> Option<Group>;
}

#[derive(Clone, Debug)]
pub enum MessageFactoryError {
    UnsupportedBeginString { begin_string: String, message: String },
    UnsupportedMsgType { msg_type: String, message: String },
}

impl MessageFactoryError {
    pub fn message(&self) -> String {
        match self {
            MessageFactoryError::UnsupportedBeginString { begin_string, message } => format!("{message}: {begin_string}"),
            MessageFactoryError::UnsupportedMsgType { msg_type, message } => format!("{message}: {msg_type}"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct DefaultMessageFactory {
    factory_map: BTreeMap<String, Arc<Mutex<Box<dyn MessageFactory>>>>,
}
impl DefaultMessageFactory {
    pub fn new() -> Self {
        DefaultMessageFactory {
            factory_map: Default::default()
        }
    }
    pub fn boxed() -> Box<dyn MessageFactory> {
        Box::new(DefaultMessageFactory::new())
    }
}

//TODO delegate to msg factory of impl;
impl MessageFactory for DefaultMessageFactory {
    fn get_supported_begin_strings(&self) -> Vec<String> {
        todo!()
    }

    fn create(&self, begin_string: &str, msg_type: &str) -> Result<Message, MessageFactoryError> {
        let mut msg = Message::default();
        msg.header_mut().set_tag_value(tags::BeginString, begin_string);
        msg.header_mut().set_tag_value(tags::MsgType, msg_type);
        Ok(msg)
    }

    fn create_group(&self, begin_string: &str, msg_type: &str, group_counter_tag: Tag) -> Option<Group> {
        if let Some(factory) = self.factory_map.get(begin_string) {
            todo!("# TODO create_group({begin_string}, {msg_type}, {group_counter_tag}, {factory:?})");
            //let _ = Group::new(0, group_counter_tag);
        }
        None
    }
}
