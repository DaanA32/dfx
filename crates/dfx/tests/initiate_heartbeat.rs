#![allow(dead_code)]
#![allow(unused)]
use dfx::{
    connection::SocketInitiator,
    data_dictionary_provider::DefaultDataDictionaryProvider,
    logging::PrintlnLogFactory,
    message::DefaultMessageFactory,
    message_store::DefaultStoreFactory,
    session::{Session, SessionSettings},
};

mod common;
use common::TestApplication;
use dfx_testing::runner;

#[test]
pub fn test_client_heartbeat() {
    let runner_thread = runner::from_filename("tests/definitions/client/heartbeat.def");

    let app = TestApplication::new();
    let session_settings = SessionSettings::from_file("tests/initiator.cfg").unwrap();
    let mut initiator = SocketInitiator::new(
        session_settings.clone(),
        app,
        DefaultStoreFactory::new(&session_settings),
        DefaultDataDictionaryProvider::new(),
        PrintlnLogFactory::new(),
        DefaultMessageFactory::new(),
    );

    std::thread::sleep(std::time::Duration::from_millis(2000));
    initiator.start();
    runner_thread.join().unwrap();
    initiator.stop();
}
