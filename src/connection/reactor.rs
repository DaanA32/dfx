use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpStream}, sync::mpsc::{Receiver, Sender}, time::Duration,
};

use crate::{
    connection::StreamFactory,
    parser::{Parser, ParserError},
    session::{Session, SessionId, Responder, ChannelResponder, ResponderEvent, ResponderResponse},
};

use super::ConnectionError;

pub struct SocketReactor {
    session: Option<Session>,
    parser: Parser,
    state: ConnectionState,
    stream: Option<TcpStream>,
    buffer: [u8; SocketReactor::BUF_SIZE],
    rx: Option<Receiver<ResponderEvent>>,
    tx: Option<Sender<ResponderResponse>>,
}

pub enum ConnectionState {
    Pending(SessionId),
    Connected(SessionId),
    Disconnected(SessionId),
    NotStarted,
}

#[derive(Debug)]
pub enum ReactorError {
    Timeout(String),
    ConnectionError(ConnectionError),
    ParserError(ParserError),
    IoError(std::io::Error),
    Disconnect,
}

impl From<ConnectionError> for ReactorError {
    fn from(e: ConnectionError) -> ReactorError {
        ReactorError::ConnectionError(e)
    }
}
impl From<ParserError> for ReactorError {
    fn from(e: ParserError) -> ReactorError {
        ReactorError::ParserError(e)
    }
}
impl From<std::io::Error> for ReactorError {
    fn from(e: std::io::Error) -> ReactorError {
        ReactorError::IoError(e)
    }
}

impl SocketReactor {
    pub const BUF_SIZE: usize = 512;

    //TODO remove some if not needed
    //TODO dynamic acceptor (ie set session id)
    pub fn new(connection: TcpStream, mut session: Option<Session>) -> Self {
        let parser = Parser::default();
        // TODO move this to a concurrent map > SessionState > Sender<Message>
        let state = ConnectionState::NotStarted;
        let stream = Some(connection);
        let buffer = [0; Self::BUF_SIZE];
        let mut rx = None;
        let mut tx = None;
        if let Some(s) = session.as_mut() {
            let (responder, rx1, tx1) = ChannelResponder::new();
            s.set_responder(Box::new(responder));
            rx = Some(rx1);
            tx = Some(tx1);
        }
        SocketReactor {
            session,
            parser,
            state,
            stream,
            buffer,
            rx,
            tx,
        }
    }

    pub(crate) fn get_session_mut(&mut self) -> Option<&mut Session> {
        self.session.as_mut()
    }

    pub(crate) fn start(mut self) -> Option<Session> {
        if let Err(e) = self.event_loop() {
            match e {
                ReactorError::Disconnect => {
                    //TODO move this?
                    let session_id = self
                        .session
                        .as_ref()
                        .expect("Session not found!")
                        .session_id()
                        .clone();
                    self.set_disconnected(session_id);
                },
                e => todo!("SocketReactor::start: Error {:?}", e)
            }
        }
        self.session
    }

    fn event_loop(&mut self) -> Result<(), ReactorError> {
        //let session: &mut Session = self.get_session_mut();
        //TODO: session.log().on_event(format!("Connecting... {} {}", self.address).as_str());
        // self.connect()?;
        //  t.Reactor.SetConnected(t.Session.SessionID);
        let session_id = self
            .session
            .as_ref()
            .expect("Session not found!")
            .session_id()
            .clone();
        self.set_connected(session_id.clone());

        let session = self.session.as_mut().expect("Session not found!");
        session.log().on_event(format!("Connection succeeded {}", &session_id).as_str());
        session.next();
        while let Ok(()) = self.read() {}
        println!("loop - break");
        // if (t.Reactor.IsStopped)
        //     t.Reactor.RemoveThread(t);
        let session_id = self
            .session
            .as_ref()
            .expect("Session not found!")
            .session_id()
            .clone();
        self.set_disconnected(session_id);
        self.stream.as_mut().unwrap().shutdown(std::net::Shutdown::Both);
        println!("stream - closed");
        Ok(())
    }

    // TODO move this to a concurrent map > SessionState > Sender<Message>
    fn set_connected(&mut self, session_id: SessionId) {
        self.session.as_mut().unwrap().set_connected(&session_id).unwrap();
        self.state = ConnectionState::Connected(session_id);
    }

    // TODO move this to a concurrent map > SessionState > Sender<Message>
    fn set_disconnected(&mut self, session_id: SessionId) {
        self.session.as_mut().unwrap().set_disconnected(&session_id);
        self.state = ConnectionState::Disconnected(session_id);
    }

    // fn connect(&mut self) -> Result<(), ReactorError> {
    //     assert!(self.stream.is_none());
    //     self.stream = Some(StreamFactory::create_client_stream(
    //         &self.address,
    //         &self.socket_settings,
    //     )?);
    //     let (responder, rx, tx) = ChannelResponder::new();
    //     self.session.as_mut().unwrap().set_responder(Box::new(responder));
    //     self.rx = Some(rx);
    //     self.tx = Some(tx);
    //     Ok(())
    // }

    fn read(&mut self) -> Result<(), ReactorError> {
        let read = self.read_some()?;
        if read > 0 {
            self.parser.add_to_stream(&self.buffer[..read]);
        } else if let Some(session) = self.get_session_mut() {
            session.next();
        } else {
            return Err(ReactorError::Timeout(
                "Reactor timed out while reading socket".into(),
            ));
        }

        self.process_responder()?;
        self.process_stream()?;
        Ok(())
    }

    fn read_some(&mut self) -> Result<usize, ReactorError> {
        // read bytes nonblocking from stream...
        // add bytes to parser
        // return bytes read
        if let Some(stream) = self.stream.as_mut() {
            match stream.read(&mut self.buffer) {
                Ok(read) => Ok(read),
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => Ok(0),
                Err(e) => Err(e.into()),
            }
        } else {
            panic!("reactor::read_some")
        }
    }

    fn process_stream(&mut self) -> Result<(), ReactorError> {
        while let Some(msg) = self.parser.read_fix_message()? {
            // println!("{}", msg.iter().map(|b| *b as char).collect::<String>());
            self.session
                .as_mut()
                .expect("Session should not be None at this point.")
                .next_msg(msg);
        }
        Ok(())
    }

    fn process_responder(&mut self) -> Result<(), ReactorError> {
        match (self.tx.as_mut(), self.rx.as_mut()) {
            (Some(tx), Some(rx)) => {
                match rx.recv_timeout(Duration::from_millis(1)) {
                    Ok(event) => match event {
                        ResponderEvent::Send(message) => {
                            match self.stream.as_mut().unwrap().write_all(message.as_bytes()) {
                                Ok(_) => Ok(tx.send(ResponderResponse::Sent(true)).unwrap_or(())),
                                Err(_) => Ok(tx.send(ResponderResponse::Sent(false)).unwrap_or(())),
                            }
                        },
                        ResponderEvent::Disconnect => {
                            println!("Reactor: Disconnect");
                            Err(ReactorError::Disconnect)
                        },
                    },
                    Err(_) => Ok(()),
                }
            },
            _ => todo!()
        }
    }
}
