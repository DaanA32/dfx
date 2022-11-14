use std::{net::{SocketAddr, TcpListener}, thread::{self, JoinHandle}, time::Duration, sync::{atomic::AtomicBool, Arc}};

use crate::{connection::SocketSettings, session::{self, Session, SessionBuilder, SessionSettings, Application, SessionSetting}};

use super::{SocketReactor, StreamFactory, ConnectionError};

type Builder = fn() -> Session;

pub struct SocketAcceptorThread<App> {
    app: App,
    addr: SocketAddr,
    session_settings: Vec<SessionSetting>,
}

#[derive(Debug)]
pub enum AcceptorError {
    BindError(std::io::Error),
    ConnectionError(ConnectionError),
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
    pub fn start(&mut self) {
        self.running.store(true, std::sync::atomic::Ordering::SeqCst);

        //TODO group by port => Vec<SessionSetting>
        for (addr, session_settings) in self.session_settings.sessions_by_address() {
            let ac = SocketAcceptorThread::new(self.app.clone(), addr, session_settings);
            let thread = ac.start(&self.running);
            self.thread.push(thread);
        }
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
            .spawn(move || {
            if let Err(e) = self.event_loop(rt) {
                match e {
                    e => todo!("SocketInitiator::start: Error {:?}", e)
                }
            }
        }).expect("socket-acceptor-thread started")
    }

    fn event_loop(&self, running: Arc<AtomicBool>) -> Result<(), AcceptorError> {
        let listener = self.bind()?;
        let mut threads = Vec::new();
        let mut n = 0;
        //TODO static listener based on sessions/ports
        while running.load(std::sync::atomic::Ordering::Relaxed) {
            match listener.accept() {
                Ok((stream, _addr)) => {
                    let session_setting = &self.session_settings[0];
                    let stream = StreamFactory::configure_stream(stream, &session_setting.socket_settings()).expect("Setup stream");
                    let session_settings = self.session_settings.clone();
                    let t = thread::Builder::new()
                        .name(format!("socket-acceptor-connection-{n}"))
                        .spawn(move || {
                            let reactor = SocketReactor::new(stream, None, session_settings);
                            reactor.start()
                        }).unwrap();
                    threads.push(t);
                    n += 1;
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(1));
                    continue;
                }
                Err(e) => todo!("encountered IO error: {e}"),
            }
        }

        Ok(())
    }

    fn bind(&self) -> Result<TcpListener, AcceptorError> {
        let listener = TcpListener::bind(self.addr).map_err(|e| AcceptorError::BindError(e))?;
        listener.set_nonblocking(true).expect("Cannot set non-blocking");
        Ok(listener)
    }
}
