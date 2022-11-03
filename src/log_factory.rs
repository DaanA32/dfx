use crate::log::Log;
use crate::session::SessionId;

pub trait LogFactory {
    fn create(&self, session_id: &SessionId) -> Box<dyn Log>;
}
