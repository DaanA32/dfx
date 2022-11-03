use crate::message_store::MessageStore;
use crate::session::SessionId;

pub trait MessageStoreFactory {
    fn create(&self, session_id: &SessionId) -> Box<dyn MessageStore>;
}
