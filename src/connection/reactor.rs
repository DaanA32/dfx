use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpStream}, sync::mpsc::{Receiver, Sender}, time::Duration,
};

use crate::{
    connection::StreamFactory,
    parser::{Parser, ParserError},
    session::{Session, SessionId, Responder, ChannelResponder, ResponderEvent, ResponderResponse, SessionSetting, Application}, message::{Message, MessageParseError},
};

use super::ConnectionError;

pub(crate) const BUF_SIZE: usize = 512;
pub(crate) struct SocketReactor<App> {
    session: Option<Session>,
    parser: Parser,
    state: ConnectionState,
    stream: Option<TcpStream>,
    buffer: [u8; BUF_SIZE],
    rx: Option<Receiver<ResponderEvent>>,
    tx: Option<Sender<ResponderResponse>>,
    settings: Vec<SessionSetting>,
    app: App,
}

pub(crate) enum ConnectionState {
    Pending(SessionId),
    Connected(SessionId),
    Disconnected(SessionId),
    NotStarted,
}

#[derive(Debug)]
pub(crate) enum ReactorError {
    Timeout(String),
    ConnectionError(ConnectionError),
    ParserError(ParserError),
    MessageParseError(MessageParseError),
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
impl From<MessageParseError> for ReactorError {
    fn from(e: MessageParseError) -> ReactorError {
        ReactorError::MessageParseError(e)
    }
}
impl From<std::io::Error> for ReactorError {
    fn from(e: std::io::Error) -> ReactorError {
        ReactorError::IoError(e)
    }
}

impl<App: Application + Clone + 'static> SocketReactor<App> {

    pub(crate) fn new(connection: TcpStream, mut session: Option<Session>, settings: Vec<SessionSetting>, app: App) -> Self {
        let mut reactor = SocketReactor {
            session,
            settings,
            parser: Parser::default(),
            // TODO move this to a concurrent map > SessionState > Sender<Message>
            state: ConnectionState::NotStarted,
            stream: Some(connection),
            buffer: [0; BUF_SIZE],
            rx: None,
            tx: None,
            app,
        };
        reactor.create_responder();
        reactor

    }

    fn create_responder(&mut self) {
        if let Some(s) = self.session.as_mut() {
            let (responder, rx1, tx1) = ChannelResponder::new();
            s.set_responder(Box::new(responder));
            self.rx = Some(rx1);
            self.tx = Some(tx1);
        }
    }

    pub(crate) fn get_session_mut(&mut self) -> Option<&mut Session> {
        self.session.as_mut()
    }

    pub(crate) fn start(mut self) -> Option<Session> {
        if let Err(e) = self.event_loop() {
            match e {
                ReactorError::Disconnect => {
                    if let Some(session) = self.session.as_ref() {
                        let session_id = session
                            .session_id()
                            .clone();
                        self.set_disconnected(session_id);
                    } else {
                        // TODO
                    }
                },
                e => todo!("SocketReactor::start: Error {:?}", e)
            }
        }
        self.session
    }

    fn event_loop(&mut self) -> Result<(), ReactorError> {

        while let None = self.session {
            self.read()?;
        }

        //TODO empty session
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
        let session_id = self
            .session
            .as_ref()
            .expect("Session not found!")
            .session_id()
            .clone();
        self.set_disconnected(session_id);
        self.stream.as_mut().unwrap().shutdown(std::net::Shutdown::Both);
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

    fn read(&mut self) -> Result<(), ReactorError> {
        let read = self.read_some()?;
        if read > 0 {
            self.parser.add_to_stream(&self.buffer[..read]);
        } else if let Some(session) = self.get_session_mut() {
            session.next();
        // } else {
        //     return Err(ReactorError::Timeout(
        //         "Reactor timed out while reading socket".into(),
        //     ));
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
            if let Some(session) = self.session.as_mut() {
                session
                    .next_msg(msg);
            } else {
                let message = Message::new(&msg[..])?;
                let session_id = message.extract_contra_session_id();
                let session_settings = self.for_session_id(&session_id);
                match session_settings {
                    Some(settings) => {
                        if settings.accepts(&session_id) {
                            let mut session = settings.create(Box::new(self.app.clone()));
                            session.set_session_id(session_id.clone());
                            self.session = Some(session);
                            self.create_responder();
                            self.session.as_mut().unwrap().next_msg(msg);
                        } else {
                            return Err(ReactorError::Disconnect);
                        }
                    },
                    None => {
                        // TODO this.Log("ERROR: Disconnecting; received message for unknown session: " + msg);
                        return Err(ReactorError::Disconnect);
                    }
                }
            }
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
            _ => Ok(()) //TODO should we just drop messages?
        }
    }

    fn for_session_id(&self, session_id: &SessionId) -> Option<&SessionSetting> {
        let best_match = &self.settings.iter()
                .map(|s| (s.score(session_id), s))
                .filter(|(score, _)| score > &0)
                .max_by(|(k1, _), (k2, _)| k1.cmp(k2))
                .map(|(k,v )| v);
        *best_match
    }
}
