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
use common::runner;
use common::TestApplication;

#[test]
pub fn test_client_receive_logon() {
    let runner_thread = runner::from_filename("tests/definitions/client/initiate_logon.def");

    let app = TestApplication::new();
    let session_settings = SessionSettings::from_file("tests/logon.cfg").unwrap();
    let mut initiator = SocketInitiator::new(
        session_settings.clone(),
        app,
        DefaultStoreFactory::new(&session_settings),
        DefaultDataDictionaryProvider::new(),
        PrintlnLogFactory::new(),
        DefaultMessageFactory::new(),
    );

    initiator.start();
    runner_thread.join().unwrap();
    initiator.stop();
}
