use std::{
    io::{Read, Write},
    sync::mpsc::{Receiver, Sender},
    time::{Duration, Instant},
};

use chrono::Utc;
use dfx_base::data_dictionary_provider::DataDictionaryProvider;
use dfx_base::message::{Message, MessageParseError};
use dfx_base::message_factory::MessageFactory;
use dfx_base::session_id::SessionId;

use crate::{
    logging::{LogFactory, Logger},
    message_store::{MessageStore, MessageStoreFactory},
    parser::{Parser, ParserError},
    session::{
        Application, ChannelResponder, Event, ISession, Input, Output, Replay, ReplayRequest,
        ResponderEvent, ResponderResponse, SessionSetting,
    },
};

use super::{ConnectionError, Stream, StreamError};

pub(crate) const BUF_SIZE: usize = 512;
pub(crate) struct SocketReactor<
    App: Application,
    StoreFactory,
    DataDictionaryProvider,
    LogFactory,
    MessageFactory,
    Log,
> {
    session: Option<ISession<App, DataDictionaryProvider, Log, MessageFactory>>,
    msg_store: Option<Box<dyn MessageStore>>,
    logger: Option<Log>,
    parser: Parser,
    stream: Option<Stream>,
    buffer: [u8; BUF_SIZE],
    rx: Option<Receiver<ResponderEvent>>,
    tx: Option<Sender<ResponderResponse>>,
    settings: Vec<SessionSetting>,
    app: App,
    store_factory: StoreFactory,
    data_dictionary_provider: DataDictionaryProvider,
    log_factory: LogFactory,
    message_factory: MessageFactory,
}

#[derive(Debug)]
pub(crate) enum ReactorError {
    ConnectionError(ConnectionError),
    ParserError(ParserError),
    MessageParseError(MessageParseError),
    IoError(std::io::Error),
    StreamError(StreamError),
    Disconnect,
}

impl From<ConnectionError> for ReactorError {
    fn from(e: ConnectionError) -> ReactorError {
        ReactorError::ConnectionError(e)
    }
}
impl From<ParserError> for ReactorError {
    fn from(e: ParserError) -> ReactorError {
        ReactorError::ParserError(e)
    }
}
impl From<MessageParseError> for ReactorError {
    fn from(e: MessageParseError) -> ReactorError {
        ReactorError::MessageParseError(e)
    }
}
impl From<std::io::Error> for ReactorError {
    fn from(e: std::io::Error) -> ReactorError {
        ReactorError::IoError(e)
    }
}
impl From<StreamError> for ReactorError {
    fn from(e: StreamError) -> ReactorError {
        ReactorError::StreamError(e)
    }
}

impl<App, SF, DDP, LF, MF, Log> SocketReactor<App, SF, DDP, LF, MF, Log>
where
    App: Application + Clone + 'static,
    SF: MessageStoreFactory + Send + Clone + 'static,
    DDP: DataDictionaryProvider + Send + Clone + 'static,
    LF: LogFactory<Log = Log> + Send + Clone + 'static,
    MF: MessageFactory + Send + Clone + 'static,
    Log: Logger + Clone + 'static,
{
    pub(crate) fn new(
        connection: Stream,
        settings: Vec<SessionSetting>,
        app: App,
        store_factory: SF,
        data_dictionary_provider: DDP,
        log_factory: LF,
        message_factory: MF,
    ) -> Self {
        let mut reactor = SocketReactor {
            session: None,
            msg_store: None,
            logger: None,
            settings,
            parser: Parser::default(),
            // TODO move this to a concurrent map > SessionState > Sender<Message>
            stream: Some(connection),
            buffer: [0; BUF_SIZE],
            rx: None,
            tx: None,
            app,
            store_factory: store_factory.clone(),
            data_dictionary_provider,
            log_factory: log_factory.clone(),
            message_factory,
        };
        if reactor.settings.len() == 1 {
            let session_setting = &reactor.settings[0];
            if session_setting.connection().is_initiator() {
                eprintln!("Is initiator");
                reactor.session = Some(
                    reactor.create_session(session_setting.session_id().clone(), session_setting),
                );
                reactor.msg_store = Some(store_factory.create(session_setting.session_id()));
                reactor.logger = Some(log_factory.create(session_setting.session_id()));
            }
            if session_setting.connection().is_acceptor() && !session_setting.is_dynamic() {
                eprintln!("Is acceptor");
                reactor.session = Some(
                    reactor.create_session(session_setting.session_id().clone(), session_setting),
                );
            }
        }
        reactor.create_responder();
        reactor
    }

    fn create_responder(&mut self) {
        // if let Some(s) = self.session.as_mut() {
        //     let (responder, rx1, tx1) = ChannelResponder::new();
        //     s.set_responder(Box::new(responder));
        //     self.rx = Some(rx1);
        //     self.tx = Some(tx1);
        // }reactor
    }

    pub(crate) fn get_session_mut(&mut self) -> Option<&mut ISession<App, DDP, Log, MF>> {
        self.session.as_mut()
    }

    pub(crate) fn start(mut self) -> Option<ISession<App, DDP, Log, MF>> {
        // TODO while within session time
        if let Err(e) = self.event_loop() {
            match e {
                ReactorError::Disconnect => {
                    if let Some(session) = self.session.as_ref() {
                        let session_id = session.session_id().clone();
                        self.set_disconnected(session_id);
                    } else {
                        // TODO
                    }
                }
                e => todo!("SocketReactor::start: Error {:?}", e),
            }
        }
        self.session
    }

    fn event_loop(&mut self) -> Result<(), ReactorError> {
        while self.session.is_none() {
            self.read()?;
        }

        //TODO empty session
        let session_id = self
            .session
            .as_ref()
            .expect("Session not found!")
            .session_id()
            .clone();
        self.set_connected(session_id.clone())?;

        let session = self.session.as_mut().expect("Session not found!");
        session
            .log()
            .on_event(format!("Connection succeeded {}", &session_id).as_str());
        session.last_now(Instant::now());
        session.last_utc(Utc::now());
        session.next();
        while let Ok(()) = self.read() {}
        let session_id = self
            .session
            .as_ref()
            .expect("Session not found!")
            .session_id()
            .clone();
        self.set_disconnected(session_id);
        let remote = self.stream.as_ref().unwrap().peer_addr()?;
        self.stream
            .as_mut()
            .unwrap()
            .shutdown(std::net::Shutdown::Both)?;
        println!("Disconnected: {remote:?}");
        Ok(())
    }

    // TODO move this to a concurrent map > SessionState > Sender<Message>
    fn set_connected(&mut self, session_id: SessionId) -> Result<(), ReactorError> {
        self.session
            .as_mut()
            .unwrap()
            .set_connected(&session_id)
            .map_err(|_e| ReactorError::Disconnect)?;
        Ok(())
    }

    // TODO move this to a concurrent map > SessionState > Sender<Message>
    fn set_disconnected(&mut self, session_id: SessionId) {
        self.session.as_mut().unwrap().set_disconnected(&session_id);
    }

    fn read(&mut self) -> Result<(), ReactorError> {
        let read = self.read_some()?;
        if read > 0 {
            self.parser.add_to_stream(&self.buffer[..read]);
        } else if let Some(session) = self.get_session_mut() {
            session.last_now(Instant::now());
            session.last_utc(Utc::now());
            session.next();
            // } else {
            //     return Err(ReactorError::Timeout(
            //         "Reactor timed out while reading socket".into(),
            //     ));
        }

        self.process_responder()?;
        self.process_stream()?;
        Ok(())
    }

    fn read_some(&mut self) -> Result<usize, ReactorError> {
        // read bytes nonblocking from stream...
        // add bytes to parser
        // return bytes read
        if let Some(stream) = self.stream.as_mut() {
            match stream.read(&mut self.buffer) {
                Ok(read) => Ok(read),
                Err(ref e)
                    if e.as_io_error().is_some()
                        && e.as_io_error().unwrap().kind() == std::io::ErrorKind::WouldBlock =>
                {
                    // println!("Would block {e:?}");
                    Ok(0)
                }
                Err(e) => Err(e.into()),
            }
        } else {
            panic!("reactor::read_some")
        }
    }

    fn process_stream(&mut self) -> Result<(), ReactorError> {
        while let Some(msg) = self.parser.read_fix_message()? {
            println!("Received Message {:?}", msg);
            if let Some(session) = self.session.as_mut() {
                session.last_now(Instant::now());
                session.last_utc(Utc::now());
                session.next_msg(msg);
            } else {
                let message = Message::new(&msg[..]).map_err(|_e| ReactorError::Disconnect)?;
                let session_id = message.extract_contra_session_id();
                eprintln!("Extracted session id {session_id}");
                let session_settings = self.for_session_id(&session_id);
                match session_settings {
                    Some(settings) => {
                        if settings.accepts(&session_id) {
                            let mut session = self.create_session(session_id.clone(), settings);
                            session.last_now(Instant::now());
                            session.last_utc(Utc::now());
                            session.next_msg(msg);
                            self.session = Some(session);
                            self.msg_store = Some(self.store_factory.create(&session_id));
                            self.logger = Some(self.log_factory.create(&session_id));
                        } else {
                            return Err(ReactorError::Disconnect);
                        }
                    }
                    None => {
                        // TODO this.Log("ERROR: Disconnecting; received message for unknown session: " + msg);
                        return Err(ReactorError::Disconnect);
                    }
                }
            }
        }
        Ok(())
    }

    fn create_session(
        &self,
        session_id: SessionId,
        settings: &SessionSetting,
    ) -> ISession<App, DDP, Log, MF> {
        let log = self.log_factory.create(&session_id);
        ISession::from_settings(
            session_id,
            self.app.clone(),
            Box::new(self.store_factory.clone()),
            self.data_dictionary_provider.clone(),
            log,
            self.message_factory.clone(),
            settings.clone(),
            Instant::now(),
            Utc::now(),
        )
    }

    fn process_responder(&mut self) -> Result<(), ReactorError> {
        match (
            &mut self.stream,
            &mut self.session,
            &mut self.msg_store,
            &mut self.logger,
        ) {
            (Some(stream), Some(session), Some(msg_store), Some(logger)) => {
                match session.poll_output() {
                    Some(output) => match output {
                        Output::Timeout => Ok(()),
                        Output::Message(message) => {
                            println!("Writing {message:?} to {}", session.session_id());
                            stream.write_all(&message)?;
                            stream.flush()?;
                            Ok(())
                        }
                        Output::Event(event) => match event {
                            Event::Disconnect => Err(ReactorError::Disconnect),
                            Event::Reset(reason) => {
                                msg_store.reset();
                                let event = match reason {
                                    Some(reason) => format!("Session reset: {reason}"),
                                    _ => "Session reset".into(),
                                };
                                logger.on_event(event.as_str());
                                Ok(())
                            }
                            Event::Refresh => Ok(msg_store.refresh()),
                            Event::Persist(seq_num, msg) => Ok(msg_store.set(seq_num, &msg)),
                            Event::IncreaseTargetSeqNum => {
                                msg_store.incr_next_target_msg_seq_num();
                                Ok(())
                            }
                            Event::IncreaseSenderSeqNum => {
                                msg_store.incr_next_sender_msg_seq_num();
                                Ok(())
                            }
                            Event::SetNextTargetSeqNum(seq_num) => {
                                msg_store.set_next_sender_msg_seq_num(seq_num);
                                Ok(())
                            }
                            Event::GetMessages(replay_request) => {
                                let messages = msg_store
                                    .get(replay_request.beg_seq_no, replay_request.end_seq_no);
                                session.last_now(Instant::now());
                                session.last_utc(Utc::now());
                                session.process_input(Input::ReplayMessage(Replay {
                                    resend_request: replay_request.resend_request,
                                    beg_seq_no: replay_request.beg_seq_no,
                                    end_seq_no: replay_request.end_seq_no,
                                    messages,
                                }));

                                Ok(())
                            }
                        },
                    },
                    None => Ok(()),
                }
            }
            _ => Ok(()),
        }
    }

    fn for_session_id(&self, session_id: &SessionId) -> Option<&SessionSetting> {
        let best_match = &self
            .settings
            .iter()
            .map(|s| (s.score(session_id), s))
            .filter(|(score, _)| score > &0)
            .max_by(|(k1, _), (k2, _)| k1.cmp(k2))
            .map(|(_, v)| v);
        *best_match
    }
}
