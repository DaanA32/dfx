use std::{io::Write, time::Instant};

use chrono::Utc;
use dfx_base::data_dictionary_provider::DataDictionaryProvider;
use dfx_base::message::Message;
use dfx_base::message_factory::MessageFactory;
use dfx_base::session_id::SessionId;

use crate::{
    logging::{LogFactory, Logger},
    message_store::{MessageStore, MessageStoreFactory},
    parser::Parser,
    session::{Application, Event, ISession, Input, Output, Replay, SessionSetting},
};

use super::{Stream, StreamError};

pub(crate) const BUF_SIZE: usize = 512;
pub(crate) struct SocketReactor<
    App: Application,
    StoreFactory,
    DataDictionaryProvider,
    LogFactory,
    MessageFactory,
    Log,
> {
    session: Option<ISession<App, Log, MessageFactory>>,
    msg_store: Option<Box<dyn MessageStore>>,
    logger: Option<Log>,
    parser: Parser,
    stream: Option<Stream>,
    buffer: [u8; BUF_SIZE],
    settings: Vec<SessionSetting>,
    app: App,
    store_factory: StoreFactory,
    data_dictionary_provider: DataDictionaryProvider,
    log_factory: LogFactory,
    message_factory: MessageFactory,
}

#[derive(Debug)]
pub(crate) enum ReactorError {
    Disconnect,
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
                reactor.msg_store = Some(store_factory.create(session_setting.session_id()));
                reactor.logger = Some(log_factory.create(session_setting.session_id()));
            }
        }
        reactor
    }

    pub(crate) fn start(mut self) -> Option<ISession<App, Log, MF>> {
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
            {
                let read = self.read_some().map_err(|_| ReactorError::Disconnect)?;
                if read > 0 {
                    self.parser.add_to_stream(&self.buffer[..read]);
                }

                while let Some(msg) = self.parser.read_fix_message() {
                    println!("Received Message {:?}", msg);
                    let message = Message::new(&msg[..]).map_err(|_e| ReactorError::Disconnect)?;
                    let session_id = message.extract_contra_session_id();
                    eprintln!("Extracted session id {session_id}");
                    let session_settings = self.for_session_id(&session_id);
                    match session_settings {
                        Some(settings) => {
                            if settings.accepts(&session_id) {
                                let mut session = self.create_session(session_id.clone(), settings);
                                session.process_input(Input::Message {
                                    last_now: Instant::now(),
                                    last_utc: Utc::now(),
                                    msg,
                                });
                                // session.last_now(Instant::now());
                                // session.last_utc(Utc::now());
                                // session.next_msg(msg);
                                self.session = Some(session);
                                self.msg_store = Some(self.store_factory.create(&session_id));
                                self.logger = Some(self.log_factory.create(&session_id));
                            } else {
                                return Err(ReactorError::Disconnect);
                            }
                        }
                        None => {
                            return Err(ReactorError::Disconnect);
                        }
                    }
                }
                Ok::<(), ReactorError>(())
            }?;
        }

        //TODO empty session
        let session_id = self
            .session
            .as_ref()
            .expect("Session not found!")
            .session_id()
            .clone();
        self.set_connected(session_id.clone())?;

        {
            let session = self.session.as_mut().expect("Session not found!");
            session
                .log()
                .on_event(format!("Connection succeeded {}", &session_id).as_str());
            session.last_now(Instant::now());
            session.last_utc(Utc::now());
            session.next();
        }

        if let Err(err) = self.do_loop() {
            println!("Disconnected: {:?}", err);
        }

        let session_id = self
            .session
            .as_ref()
            .expect("Session not found!")
            .session_id()
            .clone();
        self.set_disconnected(session_id);
        if let Some(stream) = self.stream.as_mut() {
            let _result = stream.shutdown(std::net::Shutdown::Both);
        }
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

    fn read_some(&mut self) -> Result<usize, StreamError> {
        // read bytes nonblocking from stream...
        // add bytes to parser
        // return bytes read
        if let Some(stream) = self.stream.as_mut() {
            Self::read_stream(stream, &mut self.buffer)
        } else {
            panic!("reactor::read_some")
        }
    }

    fn read_stream(stream: &mut Stream, buffer: &mut [u8]) -> Result<usize, StreamError> {
        match stream.read(buffer) {
            Ok(read) => Ok(read),
            Err(ref e)
                if e.as_io_error().is_some()
                    && e.as_io_error().unwrap().kind() == std::io::ErrorKind::WouldBlock =>
            {
                // println!("Would block {e:?}");
                Ok(0)
            }
            Err(e) => Err(e),
        }
    }

    fn create_session(
        &self,
        session_id: SessionId,
        settings: &SessionSetting,
    ) -> ISession<App, Log, MF> {
        let log = self.log_factory.create(&session_id);
        ISession::from_settings(
            session_id,
            self.app.clone(),
            self.data_dictionary_provider.clone(),
            log,
            self.message_factory.clone(),
            settings.clone(),
            Instant::now(),
            Utc::now(),
        )
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

    fn do_loop(&mut self) -> Result<(), ReactorError> {
        let parts = (
            &mut self.stream,
            &mut self.session,
            &mut self.msg_store,
            &mut self.logger,
            &mut self.buffer,
        );
        let session_parts = match parts {
            (Some(stream), Some(session), Some(msg_store), Some(logger), buffer) => {
                (stream, session, msg_store, logger, buffer)
            }
            (stream, session, msg_store, logger, _buffer) => {
                println!(
                    "{} {} {} {}",
                    stream.is_some(),
                    session.is_some(),
                    msg_store.is_some(),
                    logger.is_some()
                );
                return Ok(());
            }
        };
        let (stream, session, msg_store, logger, buffer) = session_parts;
        loop {
            match session.poll_output() {
                Some(output) => {
                    println!("[DEBUG]: {}: {:?}", Utc::now(), output);
                    match output {
                        Output::Message(message) => {
                            match stream
                                .write_all(&message)
                                .and_then(|()| Write::flush(stream))
                            {
                                Ok(()) => (),
                                Err(e) => {
                                    println!("Failed write: {:?}", e);
                                    return Err(ReactorError::Disconnect);
                                }
                            }
                            continue;
                        }
                        Output::Event(event) => match event {
                            Event::Disconnect => break,
                            Event::Reset(reason) => {
                                msg_store.reset();
                                let event = match reason {
                                    Some(reason) => format!("Session reset: {reason}"),
                                    _ => "Session reset".into(),
                                };
                                logger.on_event(event.as_str());
                                continue;
                            }
                            Event::Refresh => {
                                msg_store.refresh();
                                session.process_input(Input::SetTargetSeqNum(
                                    msg_store.next_target_msg_seq_num(),
                                ));
                                session.process_input(Input::SetSenderSeqNum(
                                    msg_store.next_sender_msg_seq_num(),
                                ));
                                continue;
                            }
                            Event::Persist(seq_num, msg) => {
                                msg_store.set(seq_num, &msg);
                                continue;
                            }
                            Event::SetNextTargetSeqNum(seq_num) => {
                                msg_store.set_next_target_msg_seq_num(seq_num);
                                continue;
                            }
                            Event::SetNextSenderSeqNum(seq_num) => {
                                msg_store.set_next_sender_msg_seq_num(seq_num);
                                continue;
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

                                continue;
                            }
                        },
                    }
                }
                None => (),
            };

            let result = Self::read_stream(stream, buffer);
            let read = match result {
                Ok(n) => n,
                Err(e) => {
                    println!("Failed read: {:?}", e);
                    break;
                }
            };
            if read > 0 {
                self.parser.add_to_stream(&buffer[..read]);
            }

            let input = match self.parser.read_fix_message() {
                Some(msg) => {
                    println!(
                        "Received {} from {}",
                        msg.iter()
                            .map(|byte| if *byte == 1 { '|' } else { *byte as char })
                            .collect::<String>(),
                        session.session_id()
                    );
                    Input::Message {
                        last_now: Instant::now(),
                        last_utc: Utc::now(),
                        msg,
                    }
                }
                None => Input::Timeout(Instant::now(), Utc::now()),
            };

            session.process_input(input);
        }
        Ok(())
    }
}
