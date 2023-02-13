use chrono::Utc;
use dfx::parser::Parser;
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader, Read, Write},
    net::{Shutdown, TcpListener, TcpStream},
    thread::{self, JoinHandle},
    time::Duration, path::Path,
};

#[allow(unused)]
const DATE_TIME_FORMAT_WITH_NANOSECONDS: &str = "%Y%m%d-%H:%M:%S.%f";
#[allow(unused)]
const DATE_TIME_FORMAT_WITH_MICROSECONDS: &str = "%Y%m%d-%H:%M:%S.%6f";
#[allow(unused)]
const DATE_TIME_FORMAT_WITH_MILLISECONDS: &str = "%Y%m%d-%H:%M:%S.%3f";
#[allow(unused)]
const DATE_TIME_FORMAT_WITHOUT_MILLISECONDS: &str = "%Y%m%d-%H:%M:%S";
#[allow(unused)]
const DATE_ONLY_FORMAT: &str = "%Y%m%d";
#[allow(unused)]
const TIME_ONLY_FORMAT_WITH_NANOSECONDS: &str = "%H:%M:%S.%f";
#[allow(unused)]
const TIME_ONLY_FORMAT_WITH_MICROSECONDS: &str = "%H:%M:%S.%6f";
#[allow(unused)]
const TIME_ONLY_FORMAT_WITH_MILLISECONDS: &str = "%H:%M:%S.%3f";
#[allow(unused)]
const TIME_ONLY_FORMAT_WITHOUT_MILLISECONDS: &str = "%H:%M:%S";

#[derive(Debug, Eq, PartialEq)]
pub enum TestStep {
    InitiateConnect(usize),
    ExpectConnect(usize),
    InitiateDisconnect(usize),
    ExpectDisconnect(usize),
    InitiateMessage(usize, String),
    ExpectMessage(usize, String),
    Comment(String),
}
lazy_static! {
    static ref COMMENT: Regex = Regex::new(r"^[ \t]*#(.*)").unwrap();
    static ref I_CONNECT: Regex = Regex::new(r"^i(\d,)?CONNECT").unwrap();
    static ref E_CONNECT: Regex = Regex::new(r"^e(\d,)?CONNECT").unwrap();
    static ref I_DISCONNECT: Regex = Regex::new(r"^i(\d,)?DISCONNECT").unwrap();
    static ref E_DISCONNECT: Regex = Regex::new(r"^e(\d,)?DISCONNECT").unwrap();
    static ref I_MESSAGE: Regex = Regex::new(r"^I(\d,)?(.*)").unwrap();
    static ref E_MESSAGE: Regex = Regex::new(r"^E(\d,)?(.*)").unwrap();

    // matches (FIXT?.X.X\x01)(body)(checksum);
    static ref MESSAGE_L: Regex = Regex::new(r"((8=FIXT?\.\d\.\d\|)((.*?\|)*))(10=.*\|)?").unwrap(); // (9=\d+)?
    static ref MESSAGE: Regex = Regex::new(r"((8=FIXT?\.\d\.\d\x01)(9=\d+\x01)((.*?\x01)*))(10=.*\x01)?").unwrap(); // (9=\d+)?
    static ref VERSION: Regex = Regex::new(r"^.*8=(FIXT?\.\d\.\d).*$").unwrap(); // (9=\d+)?
    static ref TIME: Regex = Regex::new(r"<TIME(([+-])(\d+))*>").unwrap(); // (9=\d+)?
}

pub(crate) fn version(path: &Path) -> Result<String, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for (_index, line) in reader.lines().enumerate() {
        let line = line?; // Ignore errors.
        let captures = VERSION.captures(&line);
        // println!("{line} -> {captures:?}");
        if let Some(captures) = captures {
            return Ok(captures.get(1).map_or("".into(), |m| m.as_str().into()));
        }
    }
    Ok("".into())
}

pub(crate) fn from_filename(filename: &str) -> JoinHandle<Result<(), String>> {
    let steps = steps(filename);
    let runner_thread = create_thread(steps, 40000, filename);
    runner_thread
}

pub fn steps(filename: &str) -> Vec<TestStep> {
    // TODO multi session steps
    println!("Reading steps from {filename}");
    let mut steps = vec![];

    let file = File::open(filename).expect(format!("Unable to open file: {}", filename).as_str());
    let lines = BufReader::new(file).lines();
    for line in lines {
        if let Ok(line) = line {
            if let Some(capture) = COMMENT.captures(&line) {
                match capture.get(0) {
                    Some(comment) => steps.push(TestStep::Comment(comment.as_str().to_string())),
                    None => {}
                }
            } else if let Some(capture) = I_CONNECT.captures(&line) {
                let n = capture.get(1)
                    .map(|c| c.as_str().replace(",", "").parse().unwrap())
                    .unwrap_or(0);
                match capture.get(0) {
                    Some(_) => steps.push(TestStep::InitiateConnect(n)),
                    None => {}
                }
            } else if let Some(capture) = E_CONNECT.captures(&line) {
                let n = capture.get(1)
                    .map(|c| c.as_str().replace(",", "").parse().unwrap())
                    .unwrap_or(0);
                match capture.get(0) {
                    Some(_) => steps.push(TestStep::ExpectConnect(n)),
                    None => {}
                }
            } else if let Some(capture) = I_DISCONNECT.captures(&line) {
                let n = capture.get(1)
                    .map(|c| c.as_str().replace(",", "").parse().unwrap())
                    .unwrap_or(0);
                match capture.get(0) {
                    Some(_) => steps.push(TestStep::InitiateDisconnect(n)),
                    None => {}
                }
            } else if let Some(capture) = E_DISCONNECT.captures(&line) {
                let n = capture.get(1)
                    .map(|c| c.as_str().replace(",", "").parse().unwrap())
                    .unwrap_or(0);
                match capture.get(0) {
                    Some(_) => steps.push(TestStep::ExpectDisconnect(n)),
                    None => {}
                }
            } else if let Some(capture) = I_MESSAGE.captures(&line) {
                let n = capture.get(1)
                    .map(|c| c.as_str().replace(",", "").parse().unwrap())
                    .unwrap_or(0);
                match capture.get(2) {
                    Some(message) => {
                        steps.push(TestStep::InitiateMessage(0, message.as_str().to_string()))
                    }
                    None => {}
                }
            } else if let Some(capture) = E_MESSAGE.captures(&line) {
                let n = capture.get(1)
                    .map(|c| c.as_str().replace(",", "").parse().unwrap())
                    .unwrap_or(0);
                match capture.get(2) {
                    Some(message) => {
                        steps.push(TestStep::ExpectMessage(0, message.as_str().to_string()))
                    }
                    None => {}
                }
            }
        }
    }
    steps
}

pub fn create_thread(steps: Vec<TestStep>, port: u32, filename: &str) -> JoinHandle<Result<(), String>> {
    let filename: String = filename.into();
    thread::spawn(move || perform_steps(steps, port, filename.as_str()))
}

fn perform_steps(steps: Vec<TestStep>, port: u32, filename: &str) -> Result<(), String> {
    eprintln!("Running {} step(s) from {filename}", steps.len());
    println!("Runner: performing {} step(s).", steps.len());
    assert!(steps.len() > 0);
    let filtered: Vec<&TestStep> = steps.iter().filter(|s| !matches!(s, TestStep::Comment(_))).collect();
    if !(matches!(filtered[0], &TestStep::ExpectConnect(n)) || matches!(filtered[0], &TestStep::InitiateConnect(n))) {
        assert!(matches!(filtered[0], &TestStep::ExpectConnect(n)) || matches!(filtered[0], &TestStep::InitiateConnect(n)));
    }
    let mut stream = None;
    let mut parser = Parser::default();

    for step in steps {
        // println!("{step:?}");
        match step {
            TestStep::InitiateConnect(n) => {
                if stream.is_none() {
                    stream = Some(
                        TcpStream::connect(format!("127.0.0.1:{}", port))
                            .expect("Connection initiated."),
                    );
                    stream
                        .as_mut()
                        .unwrap()
                        .set_read_timeout(Some(Duration::from_secs(10)))
                        .unwrap();
                }
            }
            TestStep::ExpectConnect(n) => {
                println!("Existing: {stream:?}");
                if stream.is_none() {
                    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
                    println!("Waiting for connection.");
                    // println!("Runner: Awaiting connection");
                    let (s, addr) = listener.accept().expect("Connected Expected");
                    println!("Accepted connection : {addr}.");
                    // println!("Runner: Connected");
                    s.set_read_timeout(Some(Duration::from_secs(10))).unwrap();
                    stream = Some(s);
                }
            }
            TestStep::InitiateDisconnect(n) => {
                if let Some(s) = &mut stream {
                    s.shutdown(Shutdown::Both).expect("Closed stream");
                    stream = None;
                }
            }
            TestStep::ExpectDisconnect(n) => {
                if let Some(s) = &mut stream {
                    wait_for_disconnect(s);
                    stream = None;
                }
            }
            TestStep::InitiateMessage(n, message) => {
                if let Some(s) = &mut stream {
                    do_send(message, s);
                }
            }
            TestStep::ExpectMessage(n, message) => {
                if let Some(s) = &mut stream {
                    do_receive(s, message, &mut parser)?;
                }
            }
            TestStep::Comment(message) => println!("{}", message),
        }
        //println!("end step");
    }
    Ok(())
}

fn wait_for_disconnect(s: &mut TcpStream) {
    s.set_read_timeout(Some(Duration::from_millis(1))).unwrap();
    s.set_write_timeout(Some(Duration::from_millis(1))).unwrap();
    s.set_nonblocking(true).unwrap();
    let mut buffer = [0; 512];
    loop {
        match s.read(&mut buffer) {
            Ok(0) => break,
            Ok(_) => {}
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => (),
            Err(_) => break,
        };
    }
}

fn do_receive(s: &mut TcpStream, message: String, parser: &mut Parser) -> Result<(), String> {
    let mut buffer = [0; 512];
    let other;
    let start = std::time::Instant::now();
    loop {
        let read = match s.read(&mut buffer) {
            Ok(read) => Ok(read),
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => Ok(0),
            Err(e) => Err(e),
        };
        match read.expect("Some or none") {
            0 => {}
            n => parser.add_to_stream(&buffer[0..n]),
        };
        if (start - std::time::Instant::now()) > Duration::from_secs(35) {
            panic!("Test failed reading fix message: Timeout");
        }
        match parser.read_fix_message() {
            Ok(message) => {
                if let Some(value) = message {
                    other = Some(value);
                    break;
                }
            }
            Err(_) => panic!("Test failed reading fix message."),
        };
    }
    let other = other.expect("Read a message");
    let other: String = other.iter().map(|b| *b as char).collect();
    // println!("Runner: Received {}", other.replace("\x01", "|"));
    let message = message.replace("|", "\x01");
    let read_fields = to_fields(other, '\x01', true);
    let expected_fields = to_fields(message, '\x01', true);
    if read_fields != expected_fields {
        Err(format!("Expected: {expected_fields:?}\nRead: {read_fields:?}"))
    } else {
        Ok(())
    }
}

fn do_send(message: String, s: &mut TcpStream) {
    let now = Utc::now();
    let mut message = message;
    while let Some(captures) = TIME.captures(&message) {
        // println!("{captures:?}");
        let num = captures.get(3).map(|g| g.as_str().parse().unwrap_or_default()).unwrap_or_default();
        let offset = if match captures.get(2) {
            Some(s) if s.as_str() == "-" => false,
            _ => true,
        } {
            num
        } else {
            0 - num
        };

        // println!("{offset}");
        message = TIME.replacen(message.as_str(), 1, now
                .checked_add_signed(chrono::Duration::seconds(offset)).unwrap()
                .format(DATE_TIME_FORMAT_WITHOUT_MILLISECONDS)
                .to_string()
                .as_str()
        ).to_string();
        // println!("{message}");
    }

    let len = do_length(&message);
    let message = if message.contains("|9=") {
        message.replace(r"9=[0-9]+", format!("9={:03}", len).as_str())
    }else{
        message.replacen(r"|", format!("|9={}|", len).as_str(), 1)
    };
    let message = message.replace("|", "\x01");
    // println!("Runner: {}", message.replace("\x01", "|"));
    let checksum = do_checksum(&message);
    let message = if message.contains("\x0110=") {
        message.replace(r"10=0", format!("10={:03}", checksum).as_str())
    } else {
        format!("{message}10={checksum:03}\x01")
    };
    // println!("Runner: {}", message.replace("\x01", "|"));
    s.write_all(message.as_bytes()).expect("Sent message");
    s.flush().unwrap();
}

fn do_checksum(message: &str) -> u32 {
    MESSAGE
        .captures(&message)
        .map(|cap| {
            // println!("{:?}", cap.get(3));
            cap.get(1).map(|mg| checksum(mg.as_str())).unwrap_or(0)
        })
        .unwrap_or(0)
}
fn do_length(message: &str) -> u32 {
    // println!("{:?}", MESSAGE_L.captures(&message));
    let message = if message.contains("|10=") {
        format!("{}|", message[..message.find("|10=").unwrap()].to_string())
    } else {
        message.to_string()
    };
    MESSAGE_L
        .captures(&message)
        .map(|cap| {
            cap.get(3).map(|mg| mg.as_str().bytes().len()).unwrap_or(0) as u32
        })
        .unwrap_or(0)
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
    // println!("{}", sum % 256);
    sum % 256
}

fn to_fields(message: String, delim: char, skip_time: bool) -> Vec<(String, String)> {
    // println!("Runner: {}", message.replace("\x01", "|"));
    message
        .split(delim)
        .into_iter()
        .map(|f| f.split('='))
        .map(|mut s| {
            (
                s.next().unwrap_or_else(|| "").into(),
                s.next().unwrap_or_else(|| "").into(),
            )
        })
        .filter(|value| value.0 != "")
        .filter(|value| skip_time && value.0 != "52")
        .filter(|value| value.0 != "10")
        .filter(|value| value.0 != "9")
        .filter(|value| value.0 != "122")
        .collect()
}
