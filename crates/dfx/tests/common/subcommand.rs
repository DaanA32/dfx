#![allow(dead_code)]
#![allow(unused)]
use std::{
    net::SocketAddr,
    time::{Duration, Instant}, path::Path, io::{BufReader, BufRead}, fs::File, borrow::BorrowMut, env,
};

use dfx::{
    connection::SocketAcceptor,
    session::{Session, SessionSettings}, message_store::DefaultStoreFactory, data_dictionary_provider::DefaultDataDictionaryProvider, logging::PrintlnLogFactory, message::DefaultMessageFactory,
};

use crate::runner;
use crate::common::test_application;
use crate::common::test_application::TestApplication;

pub fn intern_main(i: usize, cfg: &str, path: &str) {
    let path = std::path::Path::new(path);
    let app = TestApplication;
    let session_settings = SessionSettings::from_file(cfg).unwrap();
    let mut acceptor = SocketAcceptor::new(
        session_settings,
        app,
        DefaultStoreFactory::new(),
        DefaultDataDictionaryProvider::new(),
        PrintlnLogFactory::new(),
        DefaultMessageFactory::new(),
    );

    let steps = runner::steps(format!("{}", path.display()).as_str());
    acceptor.start();

    while acceptor.endpoints().len() == 0 {
        std::thread::sleep(Duration::from_millis(10));
    }

    let endpoint = acceptor.endpoints()[0];

    let runner_thread = runner::create_thread(steps, endpoint.port().into(), format!("{}", path.display()).as_str());
    let start = Instant::now();
    while !runner_thread.is_finished() {
        if Instant::now() - start > Duration::from_secs(120) {
            println!("ERROR: Timeout: {runner_thread:?}");
            break;
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    acceptor.stop();

    match runner_thread.join() {
        Ok(result) => match result {
            Ok(()) => {},
            Err(message) => panic!("Steps failed:\n{message}\n")
        },
        Err(_) => println!("Failed to join thread."),
    }
    println!("Finished {i}");
    println!("---------------------------------------");
    TestApplication::clear();
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    eprintln!("{args:?}");
    assert!(args.len() >= 4);
    let (i, cfg, path) = (&args[1], &args[2], &args[3]);
    let i = i.parse().unwrap();
    intern_main(i, cfg, path);
}
