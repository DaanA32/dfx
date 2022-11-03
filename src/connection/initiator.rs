use std::{
    io::Read,
    net::{SocketAddr, TcpStream},
};

use crate::{
    connection::StreamFactory,
    parser::{Parser, ParserError},
    session::{Session, SessionId},
};

use super::{ConnectionError, SocketSettings};

pub struct SocketInitiator {
    session: Option<Session>,
    parser: Parser,
    state: ConnectionState,
    address: SocketAddr,
    socket_settings: SocketSettings,
    stream: Option<TcpStream>,
    buffer: [u8; SocketInitiator::BUF_SIZE],
}

pub enum ConnectionState {
    Pending(SessionId),
    Connected(SessionId),
    Disconnected(SessionId),
    NotStarted,
}

#[derive(Debug)]
pub enum InitiatorError {
    Timeout(String),
    ConnectionError(ConnectionError),
    ParserError(ParserError),
    IoError(std::io::Error),
}
impl From<ConnectionError> for InitiatorError {
    fn from(e: ConnectionError) -> InitiatorError {
        InitiatorError::ConnectionError(e)
    }
}
impl From<ParserError> for InitiatorError {
    fn from(e: ParserError) -> InitiatorError {
        InitiatorError::ParserError(e)
    }
}
impl From<std::io::Error> for InitiatorError {
    fn from(e: std::io::Error) -> InitiatorError {
        InitiatorError::IoError(e)
    }
}

impl SocketInitiator {
    pub const BUF_SIZE: usize = 512;

    pub fn new(address: SocketAddr, socket_settings: SocketSettings) -> Self {
        let session = None;
        let parser = Parser::default();
        let state = ConnectionState::NotStarted;
        let stream = None;
        let buffer = [0; Self::BUF_SIZE];
        SocketInitiator {
            session,
            parser,
            state,
            address,
            socket_settings,
            stream,
            buffer,
        }
    }

    pub fn get_session_mut(&mut self) -> Option<&mut Session> {
        self.session.as_mut()
    }

    pub fn start(&mut self) {
        if let Err(e) = self.event_loop() {
            println!("SocketInitiator::start: Error {:?}", e);
        }
    }

    pub fn event_loop(&mut self) -> Result<(), InitiatorError> {
        //let session: &mut Session = self.get_session_mut();
        self.connect()?;
        //  t.Initiator.SetConnected(t.Session.SessionID);
        let session_id = self
            .session
            .as_ref()
            .expect("Session not found!")
            .session_id()
            .clone();
        self.set_connected(session_id);

        let session = self.session.as_mut().expect("Session not found!");
        session.log().on_event("Connection succeeded");
        session.next();
        while self.read()? {}
        // if (t.Initiator.IsStopped)
        //     t.Initiator.RemoveThread(t);
        let session_id = self
            .session
            .as_ref()
            .expect("Session not found!")
            .session_id()
            .clone();
        self.set_disconnected(session_id);
        Ok(())
    }

    fn set_connected(&mut self, session_id: SessionId) {
        self.state = ConnectionState::Connected(session_id);
    }

    fn set_disconnected(&mut self, session_id: SessionId) {
        self.state = ConnectionState::Disconnected(session_id);
    }

    fn connect(&mut self) -> Result<(), InitiatorError> {
        assert!(self.stream.is_none());
        self.stream = Some(StreamFactory::create_client_stream(
            &self.address,
            &self.socket_settings,
        )?);
        //TODO set responder (channels!?)
        Ok(())
    }

    fn read(&mut self) -> Result<bool, InitiatorError> {
        let read = self.read_some()?;
        if read > 0 {
            self.parser.add_to_stream(&self.buffer[..read]);
        } else if let Some(session) = self.get_session_mut() {
            session.next();
        } else {
            return Err(InitiatorError::Timeout(
                "Initiator timed out while reading socket".into(),
            ));
        }

        self.process_stream()?;
        Ok(true)
    }

    pub fn read_some(&mut self) -> Result<usize, InitiatorError> {
        // read bytes nonblocking from stream...
        // add bytes to parser
        // return bytes read
        if let Some(stream) = self.stream.as_mut() {
            let read = stream.read(&mut self.buffer)?;
            Ok(read)
        } else {
            //TODO Error?
            Ok(0)
        }
    }

    pub fn process_stream(&mut self) -> Result<(), InitiatorError> {
        while let Some(msg) = self.parser.read_fix_message()? {
            self.session
                .as_mut()
                .expect("Session should not be None at this point.")
                .next_msg(msg);
        }
        Ok(())
    }
}
