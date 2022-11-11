pub trait Log: Send {
    fn on_incoming(&self, incoming: &str);
    fn on_outgoing(&self, outgoing: &str);
    fn on_event(&self, event: &str);
}

#[derive(Debug, Clone, Copy)]
pub struct NoLogger;
impl Log for NoLogger {
    fn on_incoming(&self, _incoming: &str) {}
    fn on_outgoing(&self, _outgoing: &str) {}
    fn on_event(&self, _event: &str) {}
}

#[derive(Debug, Clone, Copy)]
pub struct PrintLnLogger;
impl Log for PrintLnLogger {
    fn on_incoming(&self, incoming: &str) {
        println!("Incoming: {}", incoming.replace("\x01", "|"));
    }
    fn on_outgoing(&self, outgoing: &str) {
        println!("Outgoing: {}", outgoing.replace("\x01", "|"));
    }
    fn on_event(&self, event: &str) {
        println!("Event: {}", event);
    }
}
