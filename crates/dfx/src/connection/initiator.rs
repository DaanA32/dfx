use std::{
    sync::{atomic::AtomicBool, Arc},
    thread::{self, JoinHandle},
};

use dfx_core::data_dictionary_provider::DataDictionaryProvider;
use dfx_core::message_factory::MessageFactory;
use crate::{
    connection::StreamFactory,
    parser::ParserError,
    session::{Application, SessionSetting, SessionSettings}, message_store::MessageStoreFactory, logging::LogFactory,
};

use super::{ConnectionError, SocketReactor};

pub struct SocketInitiator<App, StoreFactory, DataDictionaryProvider, LogFactory, MessageFactory> {
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
        // TODO move this to a concurrent map > SessionState > Sender<Message>
        SocketInitiator {
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
        let app = self.app.clone();
        let store_factory = self.store_factory.clone();
        let data_dictionary_provider = self.data_dictionary_provider.clone();
        let log_factory = self.log_factory.clone();
        let message_factory = self.message_factory.clone();
        let reactor = SocketReactor::new(
            stream,
            vec!(self.session_settings.clone()),
            app,
            store_factory,
            data_dictionary_provider,
            log_factory,
            message_factory
        );
        reactor.start();

        Ok(())
    }
}
