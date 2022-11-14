use dfx::{
    connection::SocketInitiator,
    session::SessionSettings,
};

mod common;
use common::runner;
use common::TestApplication;

#[test]
pub fn test_logout_timeout() {
    let runner_thread = runner::from_filename("tests/definitions/client/initiate_logon_timeout.def");

    let app = TestApplication;
    let session_settings = SessionSettings::from_file("tests/initiator.cfg").unwrap();
    let mut initiator = SocketInitiator::new(session_settings, app);

    initiator.start();
    runner_thread.join().unwrap();
    initiator.stop();
}
