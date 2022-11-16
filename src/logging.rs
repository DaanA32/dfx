use crate::session::SessionId;

// #[macro_export]
// macro_rules! on_event {
//     ($target:expr, $($arg:tt)+) => ($target.on_event($($arg)+))
// }

// #[macro_export]
// macro_rules! on_outgoing {
//     ($target:expr, $($arg:tt)+) => ($target.on_outgoing($($arg)+))
// }

// #[macro_export]
// macro_rules! on_incoming {
//     ($target:expr, $($arg:tt)+) => ($target.on_incoming($($arg)+))
// }

pub trait Logger: Send {
    fn on_incoming(&self, incoming: &str);
    fn on_outgoing(&self, outgoing: &str);
    fn on_event(&self, event: &str);
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct NoLogger;
impl Logger for NoLogger {
    fn on_incoming(&self, _incoming: &str) {}
    fn on_outgoing(&self, _outgoing: &str) {}
    fn on_event(&self, _event: &str) {}
}

#[derive(Debug, Clone)]
pub(crate) struct PrintLnLogger {
    session_id: SessionId,
}
impl Logger for PrintLnLogger {
    fn on_incoming(&self, incoming: &str) {
        println!("{} [INCOMING] {}", self.session_id, incoming.replace("\x01", "|"));
    }
    fn on_outgoing(&self, outgoing: &str) {
        println!("{} [OUTGOING] {}", self.session_id, outgoing.replace("\x01", "|"));
    }
    fn on_event(&self, event: &str) {
        println!("{} [EVENT   ] {}", self.session_id, event);
    }
}

pub trait LogFactory {
    fn create(&self, session_id: &SessionId) -> Box<dyn Logger>;
}

#[derive(Debug, Clone)]
pub struct PrintlnLogFactory;
impl PrintLnLogger {
    pub fn new(session_id: &SessionId) -> Box<dyn Logger> {
        Box::new(PrintLnLogger { session_id: session_id.clone() })
    }
}
impl PrintlnLogFactory {
    pub fn new() -> Box<dyn LogFactory> {
        Box::new(PrintlnLogFactory)
    }
}

impl LogFactory for PrintlnLogFactory {
    fn create(&self, session_id: &SessionId) -> Box<dyn Logger> {
        PrintLnLogger::new(session_id)
    }
}
