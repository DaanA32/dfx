use crate::session::SessionId;
use crate::message_store::MessageStore;

pub trait MessageStoreFactory {
    fn create(&self, session_id: &SessionId) -> Box<dyn MessageStore>;
}
