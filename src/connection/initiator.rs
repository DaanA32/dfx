use std::{net::SocketAddr, sync::{atomic::AtomicBool, Arc}, thread::{JoinHandle, self}};

use crate::{
    connection::StreamFactory,
    parser::ParserError,
    session::Session,
};

use super::{ConnectionError, SocketSettings, SocketReactor};

pub struct SocketInitiator {
    session: Option<Session>,
    address: SocketAddr,
    socket_settings: SocketSettings,
    thread: Option<JoinHandle<Session>>,
    running: Arc<AtomicBool>,
}

impl SocketInitiator {
    pub fn new(address: SocketAddr, socket_settings: SocketSettings) -> Self {
        let session = None;
        // TODO move this to a concurrent map > SessionState > Sender<Message>
        SocketInitiator {
            session,
            address,
            socket_settings,
            thread: None,
            running: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn set_session(&mut self, session: Session) {
        self.session = Some(session)
    }

    pub fn start(&mut self) {
        self.running.store(true, std::sync::atomic::Ordering::SeqCst);
        let ac = SocketInitiatorThread::new(
            self.address.clone(),
            self.socket_settings.clone(),
            self.session.take()
        );
        let thread = ac.start(&self.running);
        self.thread = Some(thread);
    }
    pub fn join(&mut self) {
        if let Some(t) = self.thread.take() {
            t.join().unwrap();
        }else{
            panic!();
        }
    }
    pub fn stop(&mut self) {
        self.running.store(false, std::sync::atomic::Ordering::Relaxed);
        if let Some(t) = self.thread.take() {
            let session = t.join().unwrap();
            self.session.replace(session);
        }
    }
}

pub struct SocketInitiatorThread {
    session: Option<Session>,
    address: SocketAddr,
    socket_settings: SocketSettings,
}

#[derive(Debug)]
pub enum InitiatorError {
    Timeout(String),
    ConnectionError(ConnectionError),
    ParserError(ParserError),
    IoError(std::io::Error),
    Disconnect,
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

impl SocketInitiatorThread {

    pub fn new(address: SocketAddr, socket_settings: SocketSettings, session: Option<Session>) -> Self {
        SocketInitiatorThread {
            session,
            address,
            socket_settings,
        }
    }

    pub(crate) fn start(mut self, _running: &Arc<AtomicBool>) -> JoinHandle<Session> {
        thread::Builder::new()
            .name("socket-initiator-thread".into())
            .spawn(move || {
            if let Err(e) = self.event_loop() {
                match e {
                    e => todo!("SocketInitiator::start: Error {:?}", e)
                }
            }
            self.session.unwrap()
        }).expect("socket-acceptor-thread started")
    }

    pub fn event_loop(&mut self,) -> Result<(), InitiatorError> {
        let stream = StreamFactory::create_client_stream(
            &self.address,
            &self.socket_settings,
        )?;

        let session = self.session.take();
        assert!(session.is_some(), "Expected existing session");
        let reactor = SocketReactor::new(stream, session);
        let session = reactor.start();
        match session {
            Some(session) => { self.session.replace(session); },
            None => {},
        }
        Ok(())
    }

}
