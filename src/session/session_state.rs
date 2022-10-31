use std::collections::BTreeMap;
use std::time::Instant;
use crate::message::Message;
use crate::message_store::MessageStore;
use crate::log::Log;

// private object sync_ = new object();
// private bool isEnabled_ = true;
// private bool receivedLogon_ = false;
// private bool receivedReset_ = false;
// private bool sentLogon_ = false;
// private bool sentLogout_ = false;
// private bool sentReset_ = false;
// private string logoutReason_ = "";
// private int testRequestCounter_ = 0;
// private int heartBtInt_ = 0;
// private int heartBtIntAsMilliSecs_ = 0;
// private DateTime lastReceivedTimeDT_ = DateTime.MinValue;
// private DateTime lastSentTimeDT_ = DateTime.MinValue;
// private int logonTimeout_ = 10;
// private long logonTimeoutAsMilliSecs_ = 10 * 1000;
// private int logoutTimeout_ = 2;
// private long logoutTimeoutAsMilliSecs_ = 2 * 1000;
// private ResendRange resendRange_ = new ResendRange();
// private Dictionary<int, Message> msgQueue = new Dictionary<int, Message>();


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
    msg_store: Box<dyn MessageStore>, //TODO Message Store
    logger: u32, //TODO logger
}

impl SessionState {
    pub fn new(is_initiator: bool, logger: Option<Box<dyn Log>>, heartbeat_int: u32, msg_store: Box<dyn MessageStore>) -> Self {
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
            msg_store, //TODO Message Store
            logger: 0, //TODO logger
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
}
