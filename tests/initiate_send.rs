#![allow(dead_code)]
#![allow(unused)]
use dfx::{
    connection::{SocketInitiator, SocketSettings},
    session::{Session, SessionId},
};

mod common;
use common::runner;
use common::SendTestApplication;

#[test]
pub fn test_send() {
    let (runner_thread, port) = runner::from_filename("tests/definitions/client/send.def");

    let addr = format!("127.0.0.1:{}", port).parse().unwrap();
    println!("{}", addr);
    let session_settings = SocketSettings {};
    let mut initiator = SocketInitiator::new(addr, session_settings);

    let app = Box::new(SendTestApplication);
    let session_id = SessionId::new("FIX.4.4", "TW", "", "", "ISLD", "", "");
    let appl_ver_id = "FIX4.4";
    let session = Session::builder(true, app, session_id, appl_ver_id)
        .with_heartbeat_int(1)
        .build();
    initiator.set_session(session);

    initiator.start();
    runner_thread.join().unwrap();
    initiator.stop();
}
