#![allow(dead_code)]
#![allow(unused)]
use std::{
    net::SocketAddr,
    time::{Duration, Instant}, path::Path, io::{BufReader, BufRead}, fs::File, borrow::BorrowMut, env,
};

use walkdir::WalkDir;
use walkdir::DirEntry;

use dfx::{
    connection::SocketAcceptor,
    session::{Session, SessionSettings}, message_store::DefaultStoreFactory, data_dictionary_provider::DefaultDataDictionaryProvider, logging::PrintlnLogFactory, message::DefaultMessageFactory,
};

mod common;
use common::TestApplication;
use dfx_testing::runner;

fn is_def_file(entry: Result<&DirEntry, &walkdir::Error>) -> bool {
    match entry {
        Ok(entry) =>
            entry.file_name()
                .to_str()
                .map(|s| s.ends_with("def") )
                .unwrap_or(false),
        Err(_) => false,
    }
}


#[test]
pub fn test_accept() {

    let path = "tests/definitions/server-ext/";
    let start_from = 0;
    let mut i = 0;
    let mut entries: Vec<walkdir::DirEntry> = WalkDir::new(path).into_iter()
        .filter(|entry| is_def_file(entry.as_ref()))
        .map(|entry| entry.ok().unwrap())
        .collect();
    entries.sort_by(|a, b| a.path().cmp(b.path()));
    for entry in entries {
        if i <= start_from {
            i+=1;
            continue;
        }
        let path = entry.path();
        let cfg = match runner::version(path) {
            Ok(v) => match v.as_str() {
                "FIX.4.0" => "tests/cfg/at_40.cfg",
                s => { println!("Skipping {s:?}"); continue },
            },
            s => todo!("Handle {s:?}"),
        };

        let app = TestApplication::new();
        let session_settings = SessionSettings::from_file(cfg).unwrap();
        let mut acceptor = SocketAcceptor::new(
            &session_settings,
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
        // acceptor.stop();

        if runner_thread.is_finished() {
            match runner_thread.join() {
                Ok(result) => match result {
                    Ok(()) => {},
                    Err(message) => panic!("Steps failed:\n{message}\n")
                },
                Err(_) => eprintln!("Failed to join thread."),
            }
        } else {
            eprintln!("Runner did not finish in 120s {}", path.display());
        }
        println!("Finished {i}");
        println!("---------------------------------------");
        i += 1;
        // TestApplication::clear();
    }

    // let session_settings = SessionSettings::from_file("tests/cfg/at_40.cfg").unwrap();
    // let mut acceptor = SocketAcceptor::new(
    //     session_settings,
    //     app,
    //     DefaultStoreFactory::new(),
    //     DefaultDataDictionaryProvider::new(),
    //     PrintlnLogFactory::new(),
    //     DefaultMessageFactory::new(),
    // );

    // let steps = runner::steps("tests/definitions/server-ext/fix40/10_MsgSeqNumEqual.def");
    // acceptor.start();

    // let runner_thread = runner::create_thread(steps, 5002);
    // let start = Instant::now();
    // while !runner_thread.is_finished() {
    //     if Instant::now() - start > Duration::from_secs(30) {
    //         panic!("Timeout: {runner_thread:?}");
    //     }
    // }
    // acceptor.stop();
}
