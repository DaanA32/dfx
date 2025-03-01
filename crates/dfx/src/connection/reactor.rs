use std::{
    io::{Read, Write},
    time::Instant,
};

use chrono::Utc;
use dfx_base::data_dictionary_provider::DataDictionaryProvider;
use dfx_base::message::Message;
use dfx_base::message_factory::MessageFactory;
use dfx_base::session_id::SessionId;

use crate::{
    logging::{LogFactory, Logger},
    message_store::{MessageStore, MessageStoreFactory},
    parser::Parser,
    session::{Application, Event, ISession, Input, LogEvent, Output, Replay, SessionSetting},
};

pub(crate) const BUF_SIZE: usize = 512;
pub(crate) struct SocketReactor<
    App: Application,
    StoreFactory,
    DataDictionaryProvider,
    LogFactory,
    MessageFactory,
    Log,
    Stream,
> {
    buffer: [u8; BUF_SIZE],
    parser: Parser,
    stream: Stream,
    reactor_part:
        ReactorPart<App, StoreFactory, DataDictionaryProvider, LogFactory, MessageFactory, Log>,
}

enum ReactorPart<
    App: Application,
    StoreFactory,
    DataDictionaryProvider,
    LogFactory,
    MessageFactory,
    Log,
> {
    Session(SessionReactor<App, MessageFactory, Log>),
    Sessionless(
        SessionlessReactor<App, StoreFactory, DataDictionaryProvider, LogFactory, MessageFactory>,
    ),
}

struct SessionReactor<App: Application, MessageFactory, Log> {
    session: ISession<App, MessageFactory>,
    msg_store: Box<dyn MessageStore>,
    logger: Log,
}

impl<App, MF, Log> SessionReactor<App, MF, Log>
where
    App: Application + Clone + 'static,
    MF: MessageFactory + Send + Clone + 'static,
    Log: Logger + Clone + 'static,
{
    fn set_connected(&mut self, session_id: SessionId) -> Result<(), ReactorError> {
        self.session
            .set_connected(&session_id)
            .map_err(|_e| ReactorError::Disconnect)?;
        Ok(())
    }

    fn set_disconnected(&mut self, session_id: SessionId) {
        self.session.set_disconnected(&session_id);
    }
}

struct SessionlessReactor<
    App: Application,
    StoreFactory,
    DataDictionaryProvider,
    LogFactory,
    MessageFactory,
> {
    app: App,
    settings: Vec<SessionSetting>,
    store_factory: StoreFactory,
    data_dictionary_provider: DataDictionaryProvider,
    log_factory: LogFactory,
    message_factory: MessageFactory,
}

impl<App, SF, DDP, LF, MF, Log> SessionlessReactor<App, SF, DDP, LF, MF>
where
    App: Application + Clone + 'static,
    SF: MessageStoreFactory + Send + Clone + 'static,
    DDP: DataDictionaryProvider + Send + Clone + 'static,
    LF: LogFactory<Log = Log> + Send + Clone + 'static,
    MF: MessageFactory + Send + Clone + 'static,
{
    fn create_session(
        &self,
        session_id: SessionId,
        settings: &SessionSetting,
    ) -> ISession<App, MF> {
        ISession::from_settings(
            session_id,
            self.app.clone(),
            self.data_dictionary_provider.clone(),
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
}

#[derive(Debug)]
pub(crate) enum ReactorError {
    Disconnect,
}

impl<App, SF, DDP, LF, MF, Log, Stream> SocketReactor<App, SF, DDP, LF, MF, Log, Stream>
where
    App: Application + Clone + 'static,
    SF: MessageStoreFactory + Send + Clone + 'static,
    DDP: DataDictionaryProvider + Send + Clone + 'static,
    LF: LogFactory<Log = Log> + Send + Clone + 'static,
    MF: MessageFactory + Send + Clone + 'static,
    Log: Logger + Clone + 'static,
    Stream: Read + Write,
{
    pub(crate) fn new(
        connection: Stream,
        session: Option<ISession<App, MF>>,
        settings: Vec<SessionSetting>,
        app: App,
        store_factory: SF,
        data_dictionary_provider: DDP,
        log_factory: LF,
        message_factory: MF,
    ) -> Self {
        let reactor_part = if let Some(session) = session {
            let msg_store = store_factory.create(session.session_id());
            let logger = log_factory.create(session.session_id());
            ReactorPart::Session(SessionReactor {
                session,
                msg_store,
                logger,
            })
        // Below clause should be redundant!
        // } else if settings.len() == 1
        //     && settings[0].connection().is_acceptor()
        //     && !settings[0].is_dynamic()
        // {
        //     let mut session = ISession::from_settings(
        //         settings[0].session_id().clone(),
        //         app,
        //         data_dictionary_provider,
        //         message_factory,
        //         settings[0].clone(),
        //         Instant::now(),
        //         Utc::now(),
        //     );
        //     let _ = session.set_connected(&settings[0].session_id().clone());
        //     let msg_store = store_factory.create(session.session_id());
        //     let logger = log_factory.create(session.session_id());
        //     ReactorPart::Session(SessionReactor {
        //         session,
        //         msg_store,
        //         logger,
        //     })
        } else {
            ReactorPart::Sessionless(SessionlessReactor {
                app,
                settings,
                store_factory,
                data_dictionary_provider,
                log_factory,
                message_factory,
            })
        };

        Self {
            parser: Parser::default(),
            buffer: [0; BUF_SIZE],
            stream: connection,
            reactor_part,
        }
    }

    pub(crate) fn start(mut self) -> Option<ISession<App, MF>> {
        if let Err(_err) = self.do_loop() {}
        match &mut self.reactor_part {
            ReactorPart::Session(session_reactor) => {
                session_reactor.set_disconnected(session_reactor.session.session_id().clone());
            }
            ReactorPart::Sessionless(_sessionless_reactor) => {}
        }
        // let _result = self.stream.shutdown(std::net::Shutdown::Both);
        match self.reactor_part {
            ReactorPart::Session(session_reactor) => Some(session_reactor.session),
            ReactorPart::Sessionless(_sessionless_reactor) => None,
        }
    }

    fn read_stream(stream: &mut Stream, buffer: &mut [u8]) -> std::io::Result<usize> {
        match stream.read(buffer) {
            Ok(read) => Ok(read),
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => Ok(0),
            Err(e) => Err(e),
        }
    }

    fn do_loop(&mut self) -> Result<(), ReactorError> {
        self.do_sessionless_loop()?;
        self.do_session_loop()?;
        Ok(())
    }

    fn do_session_loop(&mut self) -> Result<(), ReactorError> {
        let reactor = match &mut self.reactor_part {
            ReactorPart::Session(session_reactor) => Ok(session_reactor),
            ReactorPart::Sessionless(_sessionless_reactor) => Err(ReactorError::Disconnect),
        }?;
        let session = &mut reactor.session;
        loop {
            match session.poll_output() {
                Some(output) => match output {
                    Output::Message(message) => {
                        match (&mut self.stream)
                            .write_all(&message)
                            .and_then(|()| Write::flush(&mut self.stream))
                        {
                            Ok(()) => Ok(()),
                            Err(e) => Err(ReactorError::Disconnect),
                        }?;
                        continue;
                    }
                    Output::Event(event) => match event {
                        Event::Disconnect => break,
                        Event::Reset(reason) => {
                            (&mut reactor.msg_store).reset();
                            let event = match reason {
                                Some(reason) => format!("Session reset: {reason}"),
                                _ => "Session reset".into(),
                            };
                            (&mut reactor.logger).on_event(event.as_str());
                            continue;
                        }
                        Event::Refresh => {
                            (&mut reactor.msg_store).refresh();
                            session.process_input(Input::SetTargetSeqNum(
                                (&mut reactor.msg_store).next_target_msg_seq_num(),
                            ));
                            session.process_input(Input::SetSenderSeqNum(
                                (&mut reactor.msg_store).next_sender_msg_seq_num(),
                            ));
                            continue;
                        }
                        Event::Persist(seq_num, msg) => {
                            (&mut reactor.msg_store).set(seq_num, &msg);
                            continue;
                        }
                        Event::SetNextTargetSeqNum(seq_num) => {
                            (&mut reactor.msg_store).set_next_target_msg_seq_num(seq_num);
                            continue;
                        }
                        Event::SetNextSenderSeqNum(seq_num) => {
                            (&mut reactor.msg_store).set_next_sender_msg_seq_num(seq_num);
                            continue;
                        }
                        Event::GetMessages(replay_request) => {
                            let messages = (&mut reactor.msg_store)
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
                        Event::Log(log_event) => handle_log_event(&mut reactor.logger, log_event),
                    },
                },
                None => (),
            };

            let result = Self::read_stream(&mut self.stream, &mut self.buffer);
            let read = match result {
                Ok(n) => n,
                Err(_e) => {
                    break;
                }
            };
            if read > 0 {
                self.parser.add_to_stream(&(&mut self.buffer)[..read]);
            }

            let input = match self.parser.read_fix_message() {
                Some(msg) => Input::Message {
                    last_now: Instant::now(),
                    last_utc: Utc::now(),
                    msg,
                },
                None => Input::Timeout(Instant::now(), Utc::now()),
            };

            session.process_input(input);
        }
        Ok(())
    }

    fn do_sessionless_loop(&mut self) -> Result<(), ReactorError> {
        let reactor = match &mut self.reactor_part {
            ReactorPart::Session(_session_reactor) => return Ok(()),
            ReactorPart::Sessionless(sessionless_reactor) => sessionless_reactor,
        };

        let msg: Vec<u8> = loop {
            let read = Self::read_stream(&mut self.stream, &mut self.buffer)
                .map_err(|_| ReactorError::Disconnect)?;
            if read > 0 {
                self.parser.add_to_stream(&self.buffer[..read]);
            }
            if let Some(msg) = self.parser.read_fix_message() {
                break msg;
            }
        };

        let message = Message::new(&msg[..]).map_err(|_e| ReactorError::Disconnect)?;
        let session_id = message.extract_contra_session_id();
        let session_settings = reactor.for_session_id(&session_id);
        let result: Result<(), _> = match session_settings {
            Some(settings) => {
                if settings.accepts(&session_id) {
                    let mut session = reactor.create_session(session_id.clone(), settings);
                    session
                        .set_connected(&session_id)
                        .map_err(|_e| ReactorError::Disconnect)?;
                    session.process_input(Input::Message {
                        last_now: Instant::now(),
                        last_utc: Utc::now(),
                        msg,
                    });

                    let msg_store = reactor.store_factory.create(session.session_id());
                    let logger = reactor.log_factory.create(session.session_id());

                    self.reactor_part = ReactorPart::Session(SessionReactor {
                        session,
                        msg_store,
                        logger,
                    });

                    match &self.reactor_part {
                        ReactorPart::Session(_) => Ok(()),
                        ReactorPart::Sessionless(_) => Err(ReactorError::Disconnect),
                    }
                } else {
                    Err(ReactorError::Disconnect)?
                }
            }
            None => Err(ReactorError::Disconnect)?,
        };
        result
    }
}

fn handle_log_event(logger: &mut impl Logger, log_event: LogEvent) {
    match log_event {
        LogEvent::Event(e) => logger.on_event(e.as_str()),
        LogEvent::Inbound(inbound) => logger.on_incoming(inbound.as_str()),
        LogEvent::Outbound(outbound) => logger.on_outgoing(outbound.as_str()),
    }
}
