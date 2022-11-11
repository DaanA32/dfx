use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::time::Duration;

pub trait Responder: Send {
    fn send(&mut self, message: String) -> bool;
    fn disconnect(&mut self);
}

pub enum ResponderEvent {
    Send(String),
    Disconnect,
}
pub enum ResponderResponse {
    Sent(bool),
}

pub struct ChannelResponder {
    tx: Sender<ResponderEvent>,
    rx: Receiver<ResponderResponse>,
}

impl ChannelResponder {
    pub fn new() -> (Self, Receiver<ResponderEvent>, Sender<ResponderResponse>) {
        let (tx, out_rx) = mpsc::channel();
        let (out_tx, rx) = mpsc::channel();
        (ChannelResponder { tx, rx }, out_rx, out_tx)
    }
}

impl Responder for ChannelResponder {
    fn send(&mut self, message: String) -> bool {
        match self.tx.send(ResponderEvent::Send(message)) {
            Ok(_) => {
                match self.rx.recv_timeout(Duration::from_millis(10)) {
                    Ok(response) => match response {
                        ResponderResponse::Sent(sent) => sent,
                    },
                    Err(_e) => false,
                }
            },
            Err(_) => false,
        }
    }

    fn disconnect(&mut self) {
        self.tx.send(ResponderEvent::Disconnect).unwrap()
    }
}
