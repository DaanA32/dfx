#![allow(dead_code)]
#![allow(unused)]
use std::net::SocketAddr;

use dfx::{
    connection::{SocketSettings, SocketAcceptor},
    session::{Session, SessionId, SessionSettings},
};

mod common;
use common::runner;
use common::TestApplication;

#[test]
pub fn test_accept() {
    let app = TestApplication;
    let session_settings = SessionSettings::from_file("tests/acceptor.cfg").unwrap();
    let mut acceptor = SocketAcceptor::new(session_settings, app);

    let steps = runner::steps("tests/definitions/server/accept_logon.def");
    acceptor.start();
    let runner_thread = runner::create_thread(steps, 40000);
    //std::thread::sleep(Duration::from_millis(10));
    runner_thread.join().unwrap();
    acceptor.stop();
}
