use chrono::{DateTime, Utc};

use crate::message::Message;

pub enum MessageStoreError {}

pub trait MessageStore {
    fn create(&self, message_type: &str) -> Result<Message, MessageStoreError>;
    fn reset(&mut self);
    fn creation_time(&self) -> Option<DateTime<Utc>>;
    fn refresh(&mut self);

    fn next_sender_msg_seq_num(&self) -> u32;
    fn set_next_sender_msg_seq_num(&mut self, seq_num: u32);
    fn incr_next_sender_msg_seq_num(&mut self);

    fn next_target_msg_seq_num(&self) -> u32;
    fn set_next_target_msg_seq_num(&mut self, seq_num: u32);
    fn incr_next_target_msg_seq_num(&mut self);

    fn set(&mut self, msg_seq_num: u32, message_string: &str);
}
