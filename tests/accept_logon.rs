#![allow(dead_code)]
#![allow(unused)]
use std::{
    net::SocketAddr,
    time::{Duration, Instant},
};

use dfx::{
    connection::SocketAcceptor,
    session::{Session, SessionId, SessionSettings}, message_store::DefaultStoreFactory, data_dictionary_provider::DefaultDataDictionaryProvider, logging::PrintlnLogFactory, message::DefaultMessageFactory,
};

mod common;
use common::runner;
use common::TestApplication;

#[test]
pub fn test_accept() {
    let app = TestApplication;
    let session_settings = SessionSettings::from_file("tests/acceptor.cfg").unwrap();
    let mut acceptor = SocketAcceptor::new(
        session_settings,
        app,
        DefaultStoreFactory::new(),
        DefaultDataDictionaryProvider::new(),
        PrintlnLogFactory::new(),
        DefaultMessageFactory::new(),
    );

    let steps = runner::steps("tests/definitions/server/accept_logon.def");
    acceptor.start();

    let runner_thread = runner::create_thread(steps, 40000);
    let start = Instant::now();
    while !runner_thread.is_finished() {
        if Instant::now() - start > Duration::from_secs(30) {
            panic!("Timeout: {runner_thread:?}");
        }
    }
    acceptor.stop();
}
