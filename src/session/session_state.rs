use std::collections::BTreeMap;
use std::time::Instant;
use crate::message::Message;
use crate::message_store::MessageStore;
use crate::log::Log;

pub struct SessionState {
    is_enabled: bool,
    is_initiator: bool,
    received_logon: bool,
    received_reset: bool,
    sent_logon: bool,
    sent_logout: bool,
    sent_reset: bool,
    logout_reason: Option<String>,
    test_request_counter: u32,
    heartbeat_int: u32,
    heartbeat_int_ms: u64,
    last_received_time_dt: Option<Instant>,
    last_sent_time_dt: Option<Instant>,
    logon_timeout: u32,
    logon_timeout_ms: u64,
    logout_timeout: u32,
    logout_timeout_ms: u64,
    resend_range: Option<u32>,
    message_queue: BTreeMap<u32, Message>,
    msg_store: Box<dyn MessageStore>,
    logger: Box<dyn Log>,
}

impl SessionState {
    pub fn new(is_initiator: bool, logger: Box<dyn Log>, heartbeat_int: u32, msg_store: Box<dyn MessageStore>) -> Self {
        SessionState {
            is_enabled: true,
            is_initiator: is_initiator,
            received_logon: false,
            received_reset: false,
            sent_logon: false,
            sent_logout: false,
            sent_reset: false,
            logout_reason: None,
            test_request_counter: 0,
            heartbeat_int,
            heartbeat_int_ms: (heartbeat_int as u64) * 1000,
            last_received_time_dt: None,
            last_sent_time_dt: None,
            logon_timeout: 10,
            logon_timeout_ms: 10 * 1000,
            logout_timeout: 2,
            logout_timeout_ms: 10 * 1000,
            resend_range: None,
            message_queue: BTreeMap::default(),
            msg_store,
            logger,
        }
    }

    pub fn is_initiator(&self) -> bool {
        self.is_initiator
    }
    pub fn should_send_logon(&self) -> bool {
        self.is_initiator && !self.sent_logon
    }
    pub fn set_is_enabled(&mut self, is_enabled: bool) {
        self.is_enabled = is_enabled;
    }
    pub fn is_enabled(&self) -> bool {
        self.is_enabled
    }

    pub fn sent_logon(&self) -> bool {
        self.sent_logon
    }
    pub fn set_sent_logon(&mut self, sent_logon: bool) {
        self.sent_logon = sent_logon;
    }
    pub fn sent_logout(&self) -> bool {
        self.sent_logout
    }
    pub fn received_logon(&self) -> bool {
        self.received_logon
    }
    pub fn logout_reason(&self) -> Option<String> {
        self.logout_reason.clone()
    }
    pub fn heartbeat_int(&self) -> u32 {
        self.heartbeat_int
    }

    pub fn test_request_counter(&self) -> u32 {
        self.test_request_counter
    }
    pub fn set_test_request_counter(&mut self, test_request_counter: u32) {
        self.test_request_counter = test_request_counter
    }

    pub fn logon_timed_out(&self) -> bool {
        todo!()
    }
    pub fn logout_timed_out(&self) -> bool {
        todo!()
    }
    pub fn timed_out(&self) -> bool {
        todo!()
    }
    pub fn within_heartbeat(&self) -> bool {
        todo!()
    }
    pub fn need_heartbeat(&self) -> bool {
        todo!()
    }
    pub fn need_test_request(&self) -> bool {
        todo!()
    }
    pub fn reset(&mut self, message: &str) {
        todo!("{}", message)
    }
    pub fn set_last_received_time(&mut self, instant: Instant) {
        todo!()
    }
}
