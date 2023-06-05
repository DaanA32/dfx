#![allow(dead_code)]
#![allow(unused)]
use dfx::{
    connection::SocketInitiator,
    session::{Session, SessionSettings}, message_store::DefaultStoreFactory, logging::PrintlnLogFactory, message::DefaultMessageFactory, data_dictionary_provider::DefaultDataDictionaryProvider,
};

mod common;
use common::runner;
use common::SendTestApplication;

#[test]
pub fn test_send() {
    let runner_thread = runner::from_filename("tests/definitions/client/send.def");

    let app = SendTestApplication::new();
    let session_settings = SessionSettings::from_file("tests/initiator.cfg").unwrap();
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
