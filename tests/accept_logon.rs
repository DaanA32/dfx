#![allow(dead_code)]
#![allow(unused)]
use std::net::SocketAddr;

use dfx::{
    connection::{SocketSettings, SocketAcceptor},
    session::{Session, SessionId},
};

mod common;
use common::runner;
use common::TestApplication;

#[test]
pub fn test_accept() {
    let port = runner::get_next_available_port();
    let addr: SocketAddr = format!("127.0.0.1:{port}").parse().unwrap();
    println!("{}", addr);
    let session_settings = SocketSettings {};

    let function = || {
        let app = Box::new(TestApplication);
        let appl_ver_id = "FIX4.4";
        let session_id = SessionId::new("FIX.4.4", "TEST", "", "", "LOGON", "", "");
        let session_builder = Session::builder(false, app, session_id, appl_ver_id)
            .with_heartbeat_int(300);
        session_builder.build()
    };
    let mut acceptor = SocketAcceptor::new(addr, session_settings, function);

    let steps = runner::steps("tests/definitions/server/accept_logon.def");
    acceptor.start();
    let runner_thread = runner::create_thread(steps, port);
    //std::thread::sleep(Duration::from_millis(10));
    runner_thread.join().unwrap();
    acceptor.stop();
}
