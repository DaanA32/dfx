use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;

pub(crate) trait Responder: Send {
    fn send(&mut self, message: String) -> bool;
    fn disconnect(&mut self);
}

pub(crate) enum ResponderEvent {
    Send(String),
    Disconnect,
}
pub(crate) enum ResponderResponse {
    Sent(bool),
}

pub(crate) struct ChannelResponder {
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
            Ok(_) => match self.rx.recv_timeout(Duration::from_millis(10)) {
                Ok(response) => match response {
                    ResponderResponse::Sent(sent) => sent,
                },
                Err(_e) => false,
            },
            Err(_) => false,
        }
    }

    fn disconnect(&mut self) {
        self.tx.send(ResponderEvent::Disconnect).unwrap()
    }
}
