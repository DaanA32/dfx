use std::{
    net::SocketAddr,
    sync::{atomic::AtomicBool, Arc},
    thread::{self, JoinHandle},
};

use crate::{
    connection::StreamFactory,
    parser::ParserError,
    session::{Application, Session, SessionSetting, SessionSettings}, data_dictionary_provider::{self, DataDictionaryProvider}, message_factory::{self, MessageFactory}, message_store::MessageStoreFactory, logging::LogFactory,
};

use super::{ConnectionError, SocketReactor, SocketSettings};

pub struct SocketInitiator<App, StoreFactory, DataDictionaryProvider, LogFactory, MessageFactory> {
    session: Option<Session>,
    app: App,
    store_factory: StoreFactory,
    data_dictionary_provider: DataDictionaryProvider,
    log_factory: LogFactory,
    message_factory: MessageFactory,
    session_settings: SessionSettings,
    thread: Vec<JoinHandle<()>>,
    running: Arc<AtomicBool>,
}

impl<App, SF, DDP, LF, MF> SocketInitiator<App, SF, DDP, LF, MF>
where App: Application + Clone + 'static,
      SF: MessageStoreFactory + Send + Clone + 'static,
      DDP: DataDictionaryProvider + Send + Clone + 'static,
      LF: LogFactory + Send + Clone + 'static,
      MF: MessageFactory + Send + Clone + 'static,
{
    pub fn new(session_settings: SessionSettings, app: App, store_factory: SF, data_dictionary_provider: DDP, log_factory: LF, message_factory: MF) -> Self {
        let session = None;
        // TODO move this to a concurrent map > SessionState > Sender<Message>
        SocketInitiator {
            session,
            app,
            store_factory,
            data_dictionary_provider,
            log_factory,
            message_factory,
            session_settings,
            thread: Vec::new(),
            running: Arc::new(AtomicBool::new(false)),
        }
    }

    // pub fn set_session(&mut self, session: Session) {
    //     self.session = Some(session)
    // }

    pub fn start(&mut self) {
        self.running
            .store(true, std::sync::atomic::Ordering::SeqCst);
        for session_settings in self.session_settings.sessions() {
            let ac = SocketInitiatorThread::new(self.app.clone(), self.store_factory.clone(), self.data_dictionary_provider.clone(), self.log_factory.clone(), self.message_factory.clone(), session_settings.clone());
            let thread = ac.start(&self.running);
            self.thread.push(thread);
        }
    }
    pub fn join(&mut self) {
        while self.thread.iter().any(|t| !t.is_finished()) {}
    }
    pub fn stop(mut self) {
        self.running
            .store(false, std::sync::atomic::Ordering::Relaxed);
        self.join()
    }
}

pub(crate) struct SocketInitiatorThread<App, StoreFactory, DataDictionaryProvider, LogFactory, MessageFactory> {
    app: App,
    store_factory: StoreFactory,
    data_dictionary_provider: DataDictionaryProvider,
    log_factory: LogFactory,
    message_factory: MessageFactory,
    session_settings: SessionSetting,
}

#[derive(Debug)]
pub(crate) enum InitiatorError {
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

impl<App, SF, DDP, LF, MF> SocketInitiatorThread<App, SF, DDP, LF, MF>
where App: Application + Clone + 'static,
      SF: MessageStoreFactory + Send + Clone + 'static,
      DDP: DataDictionaryProvider + Send + Clone + 'static,
      LF: LogFactory + Send + Clone + 'static,
      MF: MessageFactory + Send + Clone + 'static,
{
    pub fn new(app: App, store_factory: SF, data_dictionary_provider: DDP, log_factory: LF, message_factory: MF, session_settings: SessionSetting) -> Self {
        SocketInitiatorThread {
            app,
            store_factory,
            data_dictionary_provider,
            log_factory,
            message_factory,
            session_settings,
        }
    }

    pub(crate) fn start(mut self, _running: &Arc<AtomicBool>) -> JoinHandle<()> {
        thread::Builder::new()
            .name("socket-initiator-thread".into())
            .spawn(move || {
                if let Err(e) = self.event_loop() {
                    match e {
                        e => todo!("SocketInitiator::start: Error {:?}", e),
                    }
                }
            })
            .expect("socket-acceptor-thread started")
    }

    fn event_loop(&mut self) -> Result<(), InitiatorError> {
        let stream = StreamFactory::create_client_stream(&self.session_settings.socket_settings())?;

        let session = Session::from_settings(Box::new(self.app.clone()), Box::new(self.store_factory.clone()), Box::new(self.data_dictionary_provider.clone()), Some(Box::new(self.log_factory.clone())), Box::new(self.message_factory.clone()), self.session_settings.clone());

        let reactor = SocketReactor::new(stream, Some(session), Vec::new(), self.app.clone());
        let session = reactor.start();
        Ok(())
    }
}
