use crate::message_store::{MessageStore, MemoryMessageStore};
use crate::session::SessionId;

pub trait MessageStoreFactory {
    fn create(&self, session_id: &SessionId) -> Box<dyn MessageStore>;
}

pub struct DefaultStoreFactory;

impl DefaultStoreFactory {
    pub fn new() -> Box<dyn MessageStoreFactory> {
        Box::new(DefaultStoreFactory)
    }
}

impl MessageStoreFactory for DefaultStoreFactory {
    fn create(&self, session_id: &SessionId) -> Box<dyn MessageStore> {
        Box::new(MemoryMessageStore::new())
    }
}
