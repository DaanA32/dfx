#![allow(dead_code)]
#![allow(unused)]
use dfx::{
    connection::{SocketInitiator, SocketSettings},
    session::{Session, SessionId, SessionSettings},
};

mod common;
use common::runner;
use common::TestApplication;

#[test]
pub fn test_client_heartbeat() {
    let runner_thread = runner::from_filename("tests/definitions/client/heartbeat.def");

    let app = TestApplication;
    let session_settings = SessionSettings::from_file("tests/initiator.cfg").unwrap();
    let mut initiator = SocketInitiator::new(session_settings, app);

    initiator.start();
    runner_thread.join().unwrap();
    initiator.stop();
}
