use std::{net::{SocketAddr, TcpListener}, thread::{self, JoinHandle}, time::Duration, sync::{atomic::AtomicBool, Arc}, fmt::Display};

use crate::{connection::SocketSettings, session::{self, Session, SessionBuilder, SessionSettings, Application, SessionSetting}};

use super::{SocketReactor, StreamFactory, ConnectionError};

type Builder = fn() -> Session;

pub(crate) struct SocketAcceptorThread<App> {
    app: App,
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
            AcceptorError::BindError(err, socket) => fmt.write_fmt(format_args!("Failed to bind addr: {} error: {}", socket, err)),
            AcceptorError::ConnectionError(err) => fmt.write_fmt(format_args!("{}", err)),
        }
    }
}

impl From<ConnectionError> for AcceptorError {
    fn from(e: ConnectionError) -> Self {
        AcceptorError::ConnectionError(e)
    }
}

pub struct SocketAcceptor<App> {
    app: App,
    session_settings: SessionSettings,
    thread: Vec<JoinHandle<()>>,
    running: Arc<AtomicBool>,
}


impl<App: Application + Clone + Sync + 'static> SocketAcceptor<App> {
    pub fn new(session_settings: SessionSettings, app: App) -> Self {
        SocketAcceptor {
            app,
            session_settings,
            thread: Vec::new(),
            running: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn start(&mut self) -> &mut Self {
        self.running.store(true, std::sync::atomic::Ordering::SeqCst);

        //TODO group by port => Vec<SessionSetting>
        for (addr, session_settings) in self.session_settings.sessions_by_address() {
            let ac = SocketAcceptorThread::new(self.app.clone(), addr, session_settings);
            let thread = ac.start(&self.running);
            self.thread.push(thread);
        }

        self
    }

    pub fn join(&mut self) {
        while self.thread.iter().any(|t| !t.is_finished()) {
        }
    }

    pub fn stop(&mut self) {
        self.running.store(false, std::sync::atomic::Ordering::Relaxed);
        self.join()
    }
}

impl<App: Application + Clone + Sync + 'static> SocketAcceptorThread<App> {
    pub(crate) fn new(app: App, addr: SocketAddr, session_settings: Vec<SessionSetting>) -> Self {
        SocketAcceptorThread {
            app,
            addr,
            session_settings,
        }
    }

    pub(crate) fn start(self, running: &Arc<AtomicBool>) -> JoinHandle<()> {
        let rt = running.clone();
        thread::Builder::new()
            .name("socket-acceptor-thread".into())
            .spawn(move || { match self.event_loop(rt) {
                Ok(()) => {},
                Err(e) => println!("{e}"),
            } }
        ).expect("socket-acceptor-thread started")
    }

    fn event_loop(&self, running: Arc<AtomicBool>) -> Result<(), AcceptorError> {
        let listener = self.bind()?;
        let mut threads = Vec::new();
        let mut n = 0;
        //TODO static listener based on sessions/ports
        while running.load(std::sync::atomic::Ordering::Relaxed) {
            match listener.accept() {
                Ok((stream, _addr)) => {
                    println!("Connected");
                    let session_setting = &self.session_settings[0];
                    let stream = StreamFactory::configure_stream(stream, &session_setting.socket_settings())?;
                    let session_settings = self.session_settings.clone();
                    let app = self.app.clone();
                    let t = thread::Builder::new()
                        .name(format!("socket-acceptor-connection-{n}"))
                        .spawn(move || {
                            let reactor = SocketReactor::new(stream, None, session_settings, app);
                            reactor.start()
                        }).unwrap();
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
        let listener = TcpListener::bind(self.addr).map_err(|e| AcceptorError::BindError(e, self.addr))?;
        listener.set_nonblocking(true).expect("Cannot set non-blocking");
        Ok(listener)
    }
}
