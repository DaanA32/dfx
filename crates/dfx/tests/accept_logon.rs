#![allow(dead_code)]
#![allow(unused)]
use std::{
    net::SocketAddr,
    time::{Duration, Instant},
};

use dfx::{
    connection::SocketAcceptor,
    session::{Session, SessionSettings}, message_store::{DefaultStoreFactory, FileStoreFactory}, data_dictionary_provider::DefaultDataDictionaryProvider, logging::{PrintlnLogFactory, FileLogFactory}, message::DefaultMessageFactory,
};

#[cfg(feature = "log")]
use dfx::logging::MacroLogFactory;

mod common;
use common::runner;
use common::TestApplication;

#[test]
#[cfg(not(feature = "log"))]
pub fn test_accept() {
    let app = TestApplication::new();
    let session_settings = SessionSettings::from_file("tests/acceptor.cfg").unwrap();
    let mut acceptor = SocketAcceptor::new(
        &session_settings,
        app,
        DefaultStoreFactory::new(&session_settings),
        DefaultDataDictionaryProvider::new(),
        FileLogFactory::new(&session_settings),
        DefaultMessageFactory::new(),
    );

    let steps = runner::steps("tests/definitions/server/accept_logon.def");
    acceptor.start();

    let runner_thread = runner::create_thread(steps, 40000, "tests/definitions/server/accept_logon.def");
    let start = Instant::now();
    while !runner_thread.is_finished() {
        if Instant::now() - start > Duration::from_secs(30) {
            panic!("Timeout: {runner_thread:?}");
        }
    }
    acceptor.stop();
}

#[test]
#[cfg(feature = "log")]
pub fn test_accept() {
    env_logger::init();
    let app = TestApplication::new();
    let session_settings = SessionSettings::from_file("tests/acceptor.cfg").unwrap();
    let mut acceptor = SocketAcceptor::new(
        &session_settings,
        app,
        DefaultStoreFactory::new(&session_settings),
        DefaultDataDictionaryProvider::new(),
        MacroLogFactory::new(&session_settings),
        DefaultMessageFactory::new(),
    );

    let steps = runner::steps("tests/definitions/server/accept_logon.def");
    acceptor.start();

    let runner_thread = runner::create_thread(steps, 40000, "tests/definitions/server/accept_logon.def");
    let start = Instant::now();
    while !runner_thread.is_finished() {
        if Instant::now() - start > Duration::from_secs(30) {
            panic!("Timeout: {runner_thread:?}");
        }
    }
    acceptor.stop();
}
