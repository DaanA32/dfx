use std::{net::{SocketAddr, TcpListener}, thread::{self, JoinHandle}, time::Duration, sync::{atomic::AtomicBool, Arc}};

use crate::{connection::SocketSettings, session::{self, Session, SessionBuilder}};

use super::{SocketReactor, StreamFactory};

type Builder = fn() -> Session;

pub struct SocketAcceptorThread {
    address: SocketAddr,
    socket_settings: SocketSettings,
    session_builder: Builder,
}

#[derive(Debug)]
pub enum AcceptorError {
    BindError(std::io::Error)
}

pub struct SocketAcceptor {
    address: SocketAddr,
    socket_settings: SocketSettings,
    session_builder: Builder,
    thread: Option<JoinHandle<()>>,
    running: Arc<AtomicBool>,
}



impl SocketAcceptor {
    pub fn new<A: Into<SocketAddr>>(addr: A, socket_settings: SocketSettings, session_builder: Builder, ) -> Self {
        SocketAcceptor {
            address: addr.into(),
            socket_settings,
            session_builder,
            thread: None,
            running: Arc::new(AtomicBool::new(false)),
        }
    }
    pub fn start(&mut self) {
        self.running.store(true, std::sync::atomic::Ordering::SeqCst);
        let ac = SocketAcceptorThread::new(self.address.clone(), self.socket_settings.clone(), self.session_builder);
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
            t.join().unwrap();
        }
    }
}

impl SocketAcceptorThread {
    pub(crate) fn new<A: Into<SocketAddr>>(addr: A, socket_settings: SocketSettings, session_builder: Builder, ) -> Self {
        SocketAcceptorThread {
            address: addr.into(),
            socket_settings,
            session_builder,
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
                Ok((s, _addr)) => {
                    let s = StreamFactory::configure_stream(s, &self.socket_settings).expect("Setup stream");
                    // TODO lookup session
                    let session = (self.session_builder)();
                    let t = thread::Builder::new()
                        .name(format!("socket-connection-{n}"))
                        .spawn(move || {
                            let reactor = SocketReactor::new(s, Some(session));
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
        let listener = TcpListener::bind(self.address).map_err(|e| AcceptorError::BindError(e))?;
        listener.set_nonblocking(true).expect("Cannot set non-blocking");
        Ok(listener)
    }
}
