use dfx::data_dictionary_provider::DefaultDataDictionaryProvider;
use dfx::logging::PrintlnLogFactory;
use dfx::message::DefaultMessageFactory;
use dfx::message_store::DefaultStoreFactory;
use dfx::{connection::SocketInitiator, session::SessionSettings};

mod common;
use common::runner;
use common::TestApplication;

#[test]
pub fn test_logout_timeout() {
    let runner_thread =
        runner::from_filename("tests/definitions/client/initiate_logon_timeout.def");

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
