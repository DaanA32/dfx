use crate::{
    logging::{LogFactory, Logger},
    message_store::MessageStoreFactory,
    session::{Application, SessionSetting, SessionSettings},
};
use dfx_base::data_dictionary_provider::DataDictionaryProvider;
use dfx_base::message_factory::MessageFactory;
use std::{
    fmt::Display,
    net::{SocketAddr, TcpListener},
    sync::{atomic::AtomicBool, Arc, Mutex},
    thread::{self, JoinHandle},
    time::Duration,
};

use super::{ConnectionError, SocketReactor, StreamFactory};

pub(crate) struct SocketAcceptorThread<
    App,
    StoreFactory,
    DataDictionaryProvider,
    LogFactory,
    MessageFactory,
> {
    app: App,
    store_factory: StoreFactory,
    data_dictionary_provider: DataDictionaryProvider,
    log_factory: LogFactory,
    message_factory: MessageFactory,
    addr: SocketAddr,
    session_settings: Vec<SessionSetting>,
}

#[derive(Debug)]
pub(crate) enum AcceptorError {
    BindError(std::io::Error, SocketAddr),
    ConnectionError(ConnectionError),
}

impl Display for AcceptorError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AcceptorError::BindError(err, socket) => {
                fmt.write_fmt(format_args!("Failed to bind addr: {socket} error: {err}"))
            }
            AcceptorError::ConnectionError(err) => fmt.write_fmt(format_args!("{err}")),
        }
    }
}

impl From<ConnectionError> for AcceptorError {
    fn from(e: ConnectionError) -> Self {
        AcceptorError::ConnectionError(e)
    }
}

/// # Multi-Threaded Socket Acceptor
/// Creates one thread per port to listen to incoming connections, which then creates a new thread per connection.
/// ## Example
#[doc = include_str!("../../docs/acceptor.md")]
pub struct SocketAcceptor<App, StoreFactory, DataDictionaryProvider, LogFactory, MessageFactory> {
    app: App,
    store_factory: StoreFactory,
    data_dictionary_provider: DataDictionaryProvider,
    log_factory: LogFactory,
    message_factory: MessageFactory,
    session_settings: SessionSettings,
    thread: Vec<ThreadState>,
    running: Arc<AtomicBool>,
}

impl<App, SF, DDP, LF, MF, Log> SocketAcceptor<App, SF, DDP, LF, MF>
where
    App: Application + Sync + Clone + 'static,
    SF: MessageStoreFactory + Send + Clone + 'static,
    DDP: DataDictionaryProvider + Send + Clone + 'static,
    LF: LogFactory<Log = Log> + Send + Clone + 'static,
    MF: MessageFactory + Send + Clone + 'static,
    Log: Logger + Clone + 'static,
{
    pub fn new(
        session_settings: &SessionSettings,
        app: App,
        store_factory: SF,
        data_dictionary_provider: DDP,
        log_factory: LF,
        message_factory: MF,
    ) -> Self {
        SocketAcceptor {
            app,
            store_factory,
            data_dictionary_provider,
            log_factory,
            message_factory,
            session_settings: session_settings.clone(),
            thread: Vec::new(),
            running: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Starts the engine, creates one thread per socket address.
    pub fn start(&mut self) -> &mut Self {
        self.running
            .store(true, std::sync::atomic::Ordering::SeqCst);

        //TODO group by port => Vec<SessionSetting>
        for (addr, session_settings) in self.session_settings.sessions_by_address() {
            let ac = SocketAcceptorThread::new(
                self.app.clone(),
                self.store_factory.clone(),
                self.data_dictionary_provider.clone(),
                self.log_factory.clone(),
                self.message_factory.clone(),
                addr,
                session_settings,
            );
            let thread = ac.start(&self.running);
            self.thread.push(thread);
        }

        self
    }

    /// Wait for all threads to finish.
    pub fn join(&mut self) {
        while self.thread.iter().any(|t| !t.thread().is_finished()) {}
    }

    /// List available endpoints, useful for random port allocation.
    pub fn endpoints(&self) -> Vec<SocketAddr> {
        self.thread
            .iter()
            .filter_map(ThreadState::endpoint)
            .collect()
    }

    /// Stops the engine, and waits for the threads to finish
    pub fn stop(&mut self) {
        self.running
            .store(false, std::sync::atomic::Ordering::Relaxed);
        self.join();
    }
}

#[derive(Debug)]
pub(crate) struct ThreadState {
    endpoint: Arc<Mutex<Option<SocketAddr>>>,
    thread: JoinHandle<()>,
}

impl ThreadState {
    fn thread(&self) -> &JoinHandle<()> {
        &self.thread
    }

    fn endpoint(&self) -> Option<SocketAddr> {
        match self.endpoint.lock() {
            Ok(guard) => *guard,
            Err(_) => todo!(),
        }
    }
}

impl<App, SF, DDP, LF, MF, Log> SocketAcceptorThread<App, SF, DDP, LF, MF>
where
    App: Application + Sync + Clone + 'static,
    SF: MessageStoreFactory + Send + Clone + 'static,
    DDP: DataDictionaryProvider + Send + Clone + 'static,
    LF: LogFactory<Log = Log> + Send + Clone + 'static,
    MF: MessageFactory + Send + Clone + 'static,
    Log: Logger + Clone + 'static,
{
    pub(crate) fn new(
        app: App,
        store_factory: SF,
        data_dictionary_provider: DDP,
        log_factory: LF,
        message_factory: MF,
        addr: SocketAddr,
        session_settings: Vec<SessionSetting>,
    ) -> Self {
        SocketAcceptorThread {
            app,
            store_factory,
            data_dictionary_provider,
            log_factory,
            message_factory,
            addr,
            session_settings,
        }
    }

    pub(crate) fn start(self, running: &Arc<AtomicBool>) -> ThreadState {
        let rt = running.clone();
        let endpoint = Arc::new(Mutex::new(None));
        let ref_endpoint = endpoint.clone();
        let thread = thread::Builder::new()
            .name("socket-acceptor-thread".into())
            .spawn(move || match self.event_loop(rt, ref_endpoint) {
                Ok(()) => {}
                // TODO log error to main logger
                Err(e) => println!("{e}"),
            })
            .expect("socket-acceptor-thread started");
        ThreadState { endpoint, thread }
    }

    fn event_loop(
        &self,
        running: Arc<AtomicBool>,
        endpoint: Arc<Mutex<Option<SocketAddr>>>,
    ) -> Result<(), AcceptorError> {
        let listener = self.bind()?;
        let addr = listener.local_addr().unwrap();
        match endpoint.lock() {
            Ok(mut guard) => {
                guard.replace(addr);
            }
            Err(_) => todo!(),
        }
        let mut threads = Vec::new();
        let mut n = 0;
        //TODO static listener based on sessions/ports
        while running.load(std::sync::atomic::Ordering::Relaxed) {
            match listener.accept() {
                Ok((stream, _addr)) => {
                    // TODO replace with connected event.
                    println!("Connected: {_addr}");
                    let session_setting = &self.session_settings[0];
                    let stream = StreamFactory::configure_stream(
                        stream,
                        session_setting.socket_settings(),
                        true,
                    )?;
                    let stream = stream;
                    let session_settings = self.session_settings.clone();
                    let app = self.app.clone();
                    let store_factory = self.store_factory.clone();
                    let data_dictionary_provider = self.data_dictionary_provider.clone();
                    let log_factory = self.log_factory.clone();
                    let message_factory = self.message_factory.clone();

                    let t = thread::Builder::new()
                        .name(format!("socket-acceptor-connection-{n}"))
                        .spawn(move || {
                            let reactor = SocketReactor::new(
                                stream,
                                session_settings,
                                app,
                                store_factory,
                                data_dictionary_provider,
                                log_factory,
                                message_factory,
                            );
                            reactor.start()
                        })
                        .unwrap();
                    threads.push(t);
                    n += 1;
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(1));
                    continue;
                }
                Err(e) => panic!("encountered IO error: {e}"),
            }
        }

        Ok(())
    }

    fn bind(&self) -> Result<TcpListener, AcceptorError> {
        let listener =
            TcpListener::bind(self.addr).map_err(|e| AcceptorError::BindError(e, self.addr))?;
        listener
            .set_nonblocking(true)
            .expect("Cannot set non-blocking");
        Ok(listener)
    }
}
