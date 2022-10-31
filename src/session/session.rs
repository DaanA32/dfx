use std::{
    io::Read,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use crate::session::SessionState;
use crate::session::SessionId;
use crate::session::Application;
use crate::session::SessionSchedule;
use crate::message_store::MessageStore;
use crate::message_store_factory::MessageStoreFactory;
use crate::data_dictionary::DataDictionary;
use crate::data_dictionary_provider::DataDictionaryProvider;
use crate::message_factory::MessageFactory;
use crate::log_factory::LogFactory;

const BUF_SIZE: usize = 4096;

pub struct Session<R: Read> {
    stream: Arc<Mutex<R>>,
    buffer: [u8; BUF_SIZE],
    msg_buffer: Vec<u8>,
    heartbeat: Duration,
    heartbeat_soft_tolerance: Duration,
    heartbeat_hard_tolerance: Duration,
    last_reset: Instant,
    last_heartbeat: Instant,
    session_state: SessionState,
}

#[derive(Debug, Clone)]
pub enum Event {
    /// Incoming  FIX message.
    Message(Vec<u8>),
    /// I/O error at the transport layer.
    IoError,
    /// Time to send a new `HeartBeat <0>` message.
    Heartbeat,
    /// The FIX counterparty has missed the `Heartbeat <0>` deadline by some
    /// amount of time, and it's time to send a `Test Request <1>`
    /// message to check what's going on.
    TestRequest,
    /// The FIX counterparty has missed the `Heartbeat <0>` deadline by some
    /// amount of time, and it's stopped responding. It's time to
    /// disconnect via a `Logout <5>` message.
    Logout,
    //Phanthom(&'a ()),
}

impl<R: Read> Session<R> {
    // bool isInitiator, IApplication app, IMessageStoreFactory storeFactory, SessionID sessID, DataDictionaryProvider dataDictProvider,
    //      SessionSchedule sessionSchedule, int heartBtInt, ILogFactory logFactory, IMessageFactory msgFactory, string senderDefaultApplVerID
    pub fn new(
        is_initiator: bool,
        app: Box<dyn Application>,
        store_factory: Box<dyn MessageStoreFactory>,
        data_dictionary_provider: Box<dyn DataDictionaryProvider>,
        session_id: SessionId,
        session_schedule: SessionSchedule,
        heartbeat_int: u32,
        log_factory: Option<Box<dyn LogFactory>>,
        msg_factory: Box<dyn MessageFactory>,
        sender_default_appl_ver_id: &str,
    ) -> Self {
        let application = app;
        let schedule = session_schedule;
        let app_does_early_intercept = false; //TODO app is IApplicationExt
        let session_data_dictionary = data_dictionary_provider.get_session_data_dictionary(&session_id.begin_string);
        let application_data_dictionar = if session_id.is_fixt {
            data_dictionary_provider.get_application_data_dictionary(sender_default_appl_ver_id)
        }else{
            session_data_dictionary.clone()
        };
        let log = log_factory.map(|l| l.create(&session_id));
        let msg_store = store_factory.create(&session_id);
        let state = SessionState::new(is_initiator, log, heartbeat_int, msg_store);
        // Configuration defaults.
        // Will be overridden by the SessionFactory with values in the user's configuration.
        let PersistMessages = true;
        let ResetOnDisconnect = false;
        let SendRedundantResendRequests = false;
        let ResendSessionLevelRejects = false;
        let ValidateLengthAndChecksum = true;
        let CheckCompID = true;
        let TimeStampPrecision = TimeStampPrecision.Millisecond;
        let EnableLastMsgSeqNumProcessed = false;
        let MaxMessagesInResendRequest = 0;
        let SendLogoutBeforeTimeoutDisconnect = false;
        let IgnorePossDupResendRequests = false;
        let RequiresOrigSendingTime = true;
        let CheckLatency = true;
        let MaxLatency = 120;

        //SessionTime
        todo!()
    }

    pub fn next<'a>(&'a mut self) -> Option<Event> {
        let now = Instant::now();
        match now {
            now if now > self.last_heartbeat + self.heartbeat => Some(Event::Heartbeat),
            now if now > self.last_reset + self.heartbeat_soft_tolerance => {
                Some(Event::TestRequest)
            }
            now if now > self.last_reset + self.heartbeat_hard_tolerance => Some(Event::Logout),
            _ => self.read_frame().map(|m| Event::Message(m)),
        }
    }

    pub fn reset_heartbeat(&mut self) {
        self.last_reset = Instant::now()
    }

    fn read_frame(&mut self) -> Option<Vec<u8>> {
        dbg!("read_frame");
        match self.stream.lock() {
            Ok(mut guard) => {
                let read = guard
                    .read(&mut self.buffer)
                    .ok()
                    .or_else(|| Some(0))
                    .unwrap();
                if read != 0 {
                    let buf = &self.buffer[0..read];
                    dbg!(buf);
                    self.msg_buffer.extend(buf);
                    //Parser::read_fix(&mut self.msg_buffer)
                    todo!("Parser")
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    }
}
