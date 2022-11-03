pub trait Responder {
    fn send(&mut self, message: String) -> bool;
    fn disconnect(&mut self);
}
