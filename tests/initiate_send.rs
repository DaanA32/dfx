#![allow(dead_code)]
#![allow(unused)]
use dfx::{
    connection::SocketInitiator,
    session::{Session, SessionId, SessionSettings}, message_store::DefaultStoreFactory, logging::PrintlnLogFactory, message::DefaultMessageFactory, data_dictionary_provider::DefaultDataDictionaryProvider,
};

mod common;
use common::runner;
use common::SendTestApplication;

#[test]
pub fn test_send() {
    let runner_thread = runner::from_filename("tests/definitions/client/send.def");

    let app = SendTestApplication;
    let session_settings = SessionSettings::from_file("tests/initiator.cfg").unwrap();
    let mut initiator = SocketInitiator::new(
        session_settings,
        app,
        DefaultStoreFactory::new(),
        DefaultDataDictionaryProvider::new(),
        PrintlnLogFactory::new(),
        DefaultMessageFactory::new(),
    );

    initiator.start();
    runner_thread.join().unwrap();
    initiator.stop();
}
