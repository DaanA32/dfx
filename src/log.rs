pub trait Log {
    fn on_incoming(&self, _event: &str);
    fn on_outgoing(&self, _event: &str);
    fn on_event(&self, event: &str);
}

#[derive(Debug, Clone, Copy)]
pub struct NoLogger;
impl Log for NoLogger {
    fn on_incoming(&self, _event: &str) {}
    fn on_outgoing(&self, _event: &str) {}
    fn on_event(&self, _event: &str) {}
}
