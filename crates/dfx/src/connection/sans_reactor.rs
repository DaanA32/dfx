use std::{
    collections::VecDeque,
    io::{ErrorKind, Read, Write},
    net::{SocketAddr, TcpStream},
    str::FromStr,
    sync::Arc,
    time::{Duration, Instant},
};

use chrono::{DateTime, Utc};
use dfx_base::{
    data_dictionary_provider::{DataDictionaryProvider, DefaultDataDictionaryProvider},
    message::{DefaultMessageFactory, MessageFactory},
    parser::Parser,
    session_id::SessionId,
};

use crate::{
    logging::{LogFactory, Logger, PrintlnLogFactory},
    message_store::{MemoryStoreFactory, MessageStore, MessageStoreFactory},
    session::{Application, ISession, SessionSetting, SessionSettings},
};

#[derive(Debug)]
struct Receive<'a> {
    contents: &'a [u8],
}

#[derive(Debug)]
struct Transmit {
    contents: Arc<[u8]>,
}

#[derive(Debug)]
enum Output {
    Timeout(Instant),
    Transmit(Transmit),
}

#[derive(Debug)]
enum Input<'a> {
    Receive(Instant, Receive<'a>),
    Timeout(Instant),
}

trait Reactor {
    fn poll_output(&mut self) -> Output;
    fn handle_input(&mut self, input: Input) -> ();
}

struct SessionReactor<App, DDP, Log, MF> {
    last_now: Instant,
    transmits: VecDeque<Transmit>,
    messages: VecDeque<Vec<u8>>,
    parser: Parser,
    session: ISession<App, DDP, Log, MF>,
}

trait AsyncLogger {
    async fn log();
}

impl<App, DDP, Log, MF> SessionReactor<App, DDP, Log, MF>
where
    App: Application + Clone + 'static,
    DDP: DataDictionaryProvider + Send + Clone + 'static,
    Log: Logger + Clone,
    MF: MessageFactory + Send + Clone + 'static,
{
    fn new(
        session_id: SessionId,
        app: App,
        store_factory: Box<dyn MessageStoreFactory>,
        data_dictionary_provider: DDP,
        log: Log,
        msg_factory: MF,
        settings: SessionSetting,
        last_now: Instant,
        last_utc: DateTime<Utc>,
    ) -> Self {
        Self {
            last_now,
            transmits: VecDeque::new(),
            messages: VecDeque::new(),
            parser: Parser::default(),
            session: ISession::from_settings(
                session_id,
                app,
                store_factory,
                data_dictionary_provider,
                log,
                msg_factory,
                settings,
                last_now,
                last_utc,
            ),
        }
    }

    fn do_handle_timeout(&mut self, instant: Instant) {
        self.last_now = instant
    }

    fn do_handle_receive(&mut self, receive: Receive<'_>) {
        self.parser.add_to_stream(receive.contents);
        if let Some(message) = self.parser.read_fix_message().unwrap() {
            self.session.next_msg(message);
        }
    }
}

impl<App, DDP, Log, MF> Reactor for SessionReactor<App, DDP, Log, MF>
where
    App: Application + Clone + 'static,
    DDP: DataDictionaryProvider + Send + Clone + 'static,
    Log: Logger + Clone,
    MF: MessageFactory + Send + Clone + 'static,
{
    fn poll_output(&mut self) -> Output {
        if let Some(transmit) = self.transmits.pop_front() {
            return Output::Transmit(transmit);
        }
        // TODO: events?
        return Output::Timeout(self.last_now + Duration::from_millis(1000));
    }

    fn handle_input(&mut self, input: Input) -> () {
        match input {
            Input::Receive(instant, receive) => {
                self.do_handle_receive(receive);
                self.do_handle_timeout(instant);
            }
            Input::Timeout(instant) => self.do_handle_timeout(instant),
        }
    }
}

pub fn sans_loop<App: Application + Clone + 'static>(app: App) {
    let endpoint: SocketAddr = SocketAddr::from_str("localhost:40365").unwrap();
    let session_settings = SessionSettings::from_string("fix.cfg").unwrap();
    let session_setting = session_settings.sessions().first().unwrap();
    let session_id = session_setting.session_id();
    let log = PrintlnLogFactory::new().create(session_id);
    let mut reactor = SessionReactor::new(
        session_id.clone(),
        app,
        Box::new(MemoryStoreFactory::new()),
        DefaultDataDictionaryProvider::new(),
        log,
        DefaultMessageFactory::new(),
        session_setting.clone(),
        Instant::now(),
        Utc::now(),
    );
    let mut stream = TcpStream::connect(endpoint).unwrap();
    let mut buffer = [0; 8192];

    loop {
        let output = reactor.poll_output();
        let timeout = match output {
            Output::Timeout(instant) => instant,
            Output::Transmit(transmit) => {
                stream.write_all(&transmit.contents).unwrap();
                continue;
            }
        };

        // Duration until timeout.
        let duration = timeout - Instant::now();

        // socket.set_read_timeout(Some(0)) is not ok
        if duration.is_zero() {
            // Drive time forwards in rtc straight away.
            reactor.handle_input(Input::Timeout(Instant::now()));
            continue;
        }

        let input = match stream.read(&mut buffer) {
            Ok(read) => Input::Receive(
                Instant::now(),
                Receive {
                    contents: &buffer[0..read],
                },
            ),
            Err(e) => match e.kind() {
                // Expected error for set_read_timeout().
                // One for windows, one for the rest.
                ErrorKind::WouldBlock | ErrorKind::TimedOut => Input::Timeout(Instant::now()),

                e => {
                    eprintln!("Error: {:?}", e);
                    return; // abort
                }
            },
        };

        reactor.handle_input(input);
    }
}
