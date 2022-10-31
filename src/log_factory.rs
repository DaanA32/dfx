use crate::session::SessionId;
use crate::log::Log;

pub trait LogFactory {
    fn create(&self, session_id: &SessionId) -> Box<dyn Log>;
}
