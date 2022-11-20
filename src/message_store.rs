use std::collections::BTreeMap;

use chrono::{DateTime, Utc};

use crate::session::SessionId;

pub trait MessageStore: Send {
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
    fn get(&self, begin_seq_num: u32, end_seq_num: u32) -> Vec<&String>;
}

pub(crate) struct MemoryMessageStore {
    messages: BTreeMap<u32, String>,
    next_sender_msg_seq_num: u32,
    next_target_msg_seq_num: u32,
    creation_time: Option<DateTime<Utc>>,
}

impl MemoryMessageStore {
    pub fn new() -> Self {
        MemoryMessageStore {
            messages: BTreeMap::new(),
            next_sender_msg_seq_num: 1,
            next_target_msg_seq_num: 1,
            creation_time: Some(Utc::now()),
        }
    }
}

impl MessageStore for MemoryMessageStore {
    fn reset(&mut self) {
        self.messages.clear();
        self.next_sender_msg_seq_num = 1;
        self.next_target_msg_seq_num = 1;
        self.creation_time = Some(Utc::now());
    }

    fn creation_time(&self) -> Option<DateTime<Utc>> {
        self.creation_time
    }

    fn refresh(&mut self) {}

    fn next_sender_msg_seq_num(&self) -> u32 {
        self.next_sender_msg_seq_num
    }

    fn set_next_sender_msg_seq_num(&mut self, seq_num: u32) {
        self.next_sender_msg_seq_num = seq_num;
    }

    fn incr_next_sender_msg_seq_num(&mut self) {
        self.next_sender_msg_seq_num += 1;
    }

    fn next_target_msg_seq_num(&self) -> u32 {
        self.next_target_msg_seq_num
    }

    fn set_next_target_msg_seq_num(&mut self, seq_num: u32) {
        self.next_target_msg_seq_num = seq_num;
    }

    fn incr_next_target_msg_seq_num(&mut self) {
        self.next_target_msg_seq_num += 1;
    }

    fn set(&mut self, msg_seq_num: u32, message_string: &str) {
        self.messages.insert(msg_seq_num, message_string.into());
    }

    fn get(&self, begin_seq_num: u32, end_seq_num: u32) -> Vec<&String> {
        assert!(begin_seq_num < end_seq_num);
        self.messages
            .range(begin_seq_num..=end_seq_num)
            .map(|(_, v)| v)
            .collect()
    }
}

pub trait MessageStoreFactory {
    fn create(&self, session_id: &SessionId) -> Box<dyn MessageStore>;
}

#[derive(Clone, Debug)]
pub struct DefaultStoreFactory;

impl DefaultStoreFactory {
    pub fn new() -> Self {
        DefaultStoreFactory
    }
    pub fn boxed() -> Box<dyn MessageStoreFactory> {
        Box::new(DefaultStoreFactory)
    }
}

impl MessageStoreFactory for DefaultStoreFactory {
    fn create(&self, _session_id: &SessionId) -> Box<dyn MessageStore> {
        Box::new(MemoryMessageStore::new())
    }
}
