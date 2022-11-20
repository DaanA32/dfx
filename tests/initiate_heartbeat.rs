#![allow(dead_code)]
#![allow(unused)]
use dfx::{
    connection::SocketInitiator,
    session::{Session, SessionId, SessionSettings}, message_store::DefaultStoreFactory, data_dictionary_provider::DefaultDataDictionaryProvider, logging::PrintlnLogFactory, message::DefaultMessageFactory,
};

mod common;
use common::runner;
use common::TestApplication;

#[test]
pub fn test_client_heartbeat() {
    let runner_thread = runner::from_filename("tests/definitions/client/heartbeat.def");

    let app = TestApplication;
    let session_settings = SessionSettings::from_file("tests/initiator.cfg").unwrap();
    let mut initiator = SocketInitiator::new(
        session_settings,
        app,
        DefaultStoreFactory::new(),
        DefaultDataDictionaryProvider::new(),
        PrintlnLogFactory::new(),
        DefaultMessageFactory::new(),
    );

    std::thread::sleep(std::time::Duration::from_millis(2000));
    initiator.start();
    runner_thread.join().unwrap();
    initiator.stop();
}
