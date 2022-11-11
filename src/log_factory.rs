use crate::log::{Log, PrintLnLogger};
use crate::session::SessionId;

pub trait LogFactory {
    fn create(&self, session_id: &SessionId) -> Box<dyn Log>;
}

#[derive(Debug, Clone)]
pub struct PrintlnLogFactory;
impl PrintlnLogFactory {
    pub fn new() -> Box<dyn LogFactory> {
        Box::new(PrintlnLogFactory)
    }
}

impl LogFactory for PrintlnLogFactory {
    fn create(&self, session_id: &SessionId) -> Box<dyn Log> {
        Box::new(PrintLnLogger)
    }
}
