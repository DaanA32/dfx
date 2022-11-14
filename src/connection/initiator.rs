use std::{net::SocketAddr, sync::{atomic::AtomicBool, Arc}, thread::{JoinHandle, self}};

use crate::{
    connection::StreamFactory,
    parser::ParserError,
    session::{Session, SessionSettings, Application, SessionSetting},
};

use super::{ConnectionError, SocketSettings, SocketReactor};

pub struct SocketInitiator<App> {
    session: Option<Session>,
    app: App,
    session_settings: SessionSettings,
    thread: Vec<JoinHandle<()>>,
    running: Arc<AtomicBool>,
}

impl<App: Application + Clone + 'static> SocketInitiator<App> {
    pub fn new(session_settings: SessionSettings, app: App) -> Self {
        let session = None;
        // TODO move this to a concurrent map > SessionState > Sender<Message>
        SocketInitiator {
            session,
            app,
            session_settings,
            thread: Vec::new(),
            running: Arc::new(AtomicBool::new(false)),
        }
    }

    // pub fn set_session(&mut self, session: Session) {
    //     self.session = Some(session)
    // }

    pub fn start(&mut self) {
        self.running.store(true, std::sync::atomic::Ordering::SeqCst);
        for session_settings in self.session_settings.sessions() {
            let ac = SocketInitiatorThread::new(self.app.clone(), session_settings.clone());
            let thread = ac.start(&self.running);
            self.thread.push(thread);
        }
    }
    pub fn join(&mut self) {
        while self.thread.iter().any(|t| !t.is_finished()) {
        }
    }
    pub fn stop(mut self) {
        self.running.store(false, std::sync::atomic::Ordering::Relaxed);
        self.join()
    }
}

pub struct SocketInitiatorThread<App> {
    app: App,
    session_settings: SessionSetting,
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

impl<App: Application + Clone + 'static> SocketInitiatorThread<App> {

    pub(crate) fn new(app: App, session_settings: SessionSetting) -> Self {
        SocketInitiatorThread {
            app,
            session_settings,
        }
    }

    pub(crate) fn start(mut self, _running: &Arc<AtomicBool>) -> JoinHandle<()> {
        thread::Builder::new()
            .name("socket-initiator-thread".into())
            .spawn(move || {
            if let Err(e) = self.event_loop() {
                match e {
                    e => todo!("SocketInitiator::start: Error {:?}", e)
                }
            }
        }).expect("socket-acceptor-thread started")
    }

    pub fn event_loop(&mut self,) -> Result<(), InitiatorError> {
        let stream = StreamFactory::create_client_stream(
            &self.session_settings.socket_settings(),
        )?;

        let session = self.session_settings.create(Box::new(self.app.clone()));
        let reactor = SocketReactor::new(stream, Some(session), Vec::new());
        let session = reactor.start();
        Ok(())
    }

}
