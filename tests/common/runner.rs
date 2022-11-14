use std::{fs::File, io::{BufReader, BufRead, Write, Read}, net::{TcpStream, TcpListener, Shutdown}, thread::{self, JoinHandle}, time::Duration, sync::atomic::AtomicU32};
use chrono::Utc;
use dfx::{fields::converters::datetime::DATE_TIME_FORMAT_WITHOUT_MILLISECONDS, parser::Parser};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Eq, PartialEq)]
pub enum TestStep {
    InitiateConnect,
    ExpectConnect,
    InitiateDisconnect,
    ExpectDisconnect,
    InitiateMessage(String),
    ExpectMessage(String),
    Comment(String),
}
lazy_static! {
    static ref COMMENT: Regex = Regex::new(r"^[ \t]+#(.*)").unwrap();
    static ref I_CONNECT: Regex = Regex::new(r"^iCONNECT").unwrap();
    static ref E_CONNECT: Regex = Regex::new(r"^eCONNECT").unwrap();
    static ref I_DISCONNECT: Regex = Regex::new(r"^iDISCONNECT").unwrap();
    static ref E_DISCONNECT: Regex = Regex::new(r"^eDISCONNECT").unwrap();
    static ref I_MESSAGE: Regex = Regex::new(r"^I(.*)").unwrap();
    static ref E_MESSAGE: Regex = Regex::new(r"^E(.*)").unwrap();

    // matches (FIXT?.X.X\x01)(body)(checksum);
    static ref MESSAGE: Regex = Regex::new(r"(8=FIXT?\\.\\d\\.\\d\\001)(.*?\\001)(10=.*|)").unwrap();
}

pub(crate) fn from_filename(filename: &str) -> JoinHandle<()> {
    let steps = steps(filename);
    let runner_thread = create_thread(steps, 40000);
    runner_thread
}

pub fn steps(filename: &str) -> Vec<TestStep> {
    let mut steps = vec!();

    let file = File::open(filename).expect(format!("Unable to open file: {}", filename).as_str());
    let lines = BufReader::new(file).lines();
    for line in lines {
        if let Ok(line) = line {
            if let Some(capture) = COMMENT.captures(&line) {
                match capture.get(1) {
                    Some(comment) => steps.push(TestStep::Comment(comment.as_str().to_string())),
                    None => {},
                }
            } else if let Some(capture) = I_CONNECT.captures(&line) {
                match capture.get(0) {
                    Some(_) => steps.push(TestStep::InitiateConnect),
                    None => {},
                }
            } else if let Some(capture) = E_CONNECT.captures(&line) {
                match capture.get(0) {
                    Some(_) => steps.push(TestStep::ExpectConnect),
                    None => {},
                }
            } else if let Some(capture) = I_DISCONNECT.captures(&line) {
                match capture.get(0) {
                    Some(_) => steps.push(TestStep::InitiateDisconnect),
                    None => {},
                }
            } else if let Some(capture) = E_DISCONNECT.captures(&line) {
                match capture.get(0) {
                    Some(_) => steps.push(TestStep::ExpectDisconnect),
                    None => {},
                }
            } else if let Some(capture) = I_MESSAGE.captures(&line) {
                match capture.get(1) {
                    Some(message) => steps.push(TestStep::InitiateMessage(message.as_str().to_string())),
                    None => {},
                }
            } else if let Some(capture) = E_MESSAGE.captures(&line) {
                match capture.get(1) {
                    Some(message) => steps.push(TestStep::ExpectMessage(message.as_str().to_string())),
                    None => {},
                }
            }
        }
    }
    steps
}

pub fn create_thread(steps: Vec<TestStep>, port: u32) -> JoinHandle<()> {
    thread::spawn(move || perform_steps(steps, port))
}

fn perform_steps(steps: Vec<TestStep>, port: u32) {
    println!("Runner: performing steps: {:?}", steps);
    assert!(steps.len() > 0);
    assert!(steps[0] == TestStep::ExpectConnect || steps[0] == TestStep::InitiateConnect);
    let mut stream = None;
    let mut parser = Parser::default();

    for step in steps {
        match step {
            TestStep::InitiateConnect => if stream.is_none() {
                stream = Some(TcpStream::connect(format!("127.0.0.1:{}", port)).expect("Connection initiated."));
                stream.as_mut().unwrap().set_read_timeout(Some(Duration::from_secs(10))).unwrap();
            },
            TestStep::ExpectConnect => if stream.is_none() {
                let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
                // println!("Runner: Awaiting connection");
                let (s, _) = listener.accept().expect("Connected Expected");
                // println!("Runner: Connected");
                s.set_read_timeout(Some(Duration::from_secs(10))).unwrap();
                stream = Some(s);
            },
            TestStep::InitiateDisconnect => if let Some(s) = &mut stream {
                s.shutdown(Shutdown::Both).expect("Closed stream");
                stream = None;
            },
            TestStep::ExpectDisconnect => if let Some(s) = &mut stream {
                wait_for_disconnect(s);
                stream = None;
            },
            TestStep::InitiateMessage(message) => if let Some(s) = &mut stream {
                do_send(message, s);
            },
            TestStep::ExpectMessage(message) => if let Some(s) = &mut stream {
                do_receive(s, message, &mut parser);
            },
            TestStep::Comment(message) => println!("Runner: Comment {}", message),
        }
    }
}

fn wait_for_disconnect(s: &mut TcpStream) {
    s.set_read_timeout(Some(Duration::from_millis(1))).unwrap();
    s.set_write_timeout(Some(Duration::from_millis(1))).unwrap();
    s.set_nonblocking(true).unwrap();
    let mut buffer = [0; 512];
    loop {
        match s.read(&mut buffer) {
            Ok(0) => break,
            Ok(_) => {},
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => (),
            Err(_) => break,
        };
    }
}

fn do_receive(s: &mut TcpStream, message: String, parser: &mut Parser) {
    let mut buffer = [0; 512];
    let other;
    loop {
        let read = match s.read(&mut buffer) {
            Ok(read) => Ok(read),
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => Ok(0),
            Err(e) => Err(e),
        };
        match read.expect("Some or none") {
            0 => {},
            n => parser.add_to_stream(&buffer[0..n]),
        };
        match parser.read_fix_message() {
            Ok(message) => if let Some(value) = message {
                other = Some(value);
                break;
            },
            Err(_) => todo!(),
        };
    }
    let other = other.expect("Read a message");
    let other: String = other.iter().map(|b| *b as char).collect();
    // println!("Runner: Received {}", other.replace("\x01", "|"));
    let message = message.replace("|", "\x01");
    let read_fields = to_fields(other, '\x01', true);
    let expected_fields = to_fields(message, '\x01', true);
    assert_eq!(read_fields, expected_fields);
}

fn do_send(message: String, s: &mut TcpStream) {
    let message = message.replace("|", "\x01");
    let message = message.replace(r"<TIME>", Utc::now().format(DATE_TIME_FORMAT_WITHOUT_MILLISECONDS).to_string().as_str());
    let checksum = do_checksum(&message);
    let message = message.replace(r"10=...", format!("10={:3}", checksum).as_str());
    // println!("Runner: {}", message.replace("\x01", "|"));
    s.write_all(message.as_bytes()).expect("Sent message");
    s.flush().unwrap();
}

fn do_checksum(message: &str) -> u32 {
    COMMENT.captures(&message)
        .map(|cap| {
            // println!("{:?}", cap.get(3));
            cap.get(3).map(|mg| checksum(mg.as_str())).unwrap_or(0)
        }).unwrap_or(0)
}

fn checksum(body: &str) -> u32 {
    let mut sum = 0;
    let mut _field_sum = 0;
    for i in body.chars() {
        sum += i as u32;
        _field_sum += i as u32;
        if i == '\x01' {
            _field_sum = 0;
        }
    }
    sum % 256
}

fn to_fields(message: String, delim: char, skip_time: bool) -> Vec<(String, String)> {
    // println!("Runner: {}", message.replace("\x01", "|"));
    message
        .split(delim)
        .into_iter()
        .map(|f| f.split('='))
        .map(|mut s| (s.next().unwrap_or_else(|| "").into(), s.next().unwrap_or_else(|| "").into()))
        .filter(|value| value.0 != "" )
        .filter(|value| skip_time && value.0 != "52" )
        .filter(|value| value.0 != "10" )
        .collect()
}
