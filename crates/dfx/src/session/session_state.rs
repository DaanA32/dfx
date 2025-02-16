#![allow(dead_code)]
#![allow(unused)]
use chrono::{DateTime, Utc};

use crate::logging::Logger;
use crate::message_store::MessageStore;
use dfx_base::message::Message;
use std::collections::BTreeMap;
use std::time::Instant;

use crate::session::ResetRange;

#[derive(Debug)]
pub(crate) struct SessionState<Log> {
    is_enabled: bool,
    is_initiator: bool,
    is_connected: bool,
    received_logon: bool,
    received_reset: bool,
    sent_logon: bool,
    sent_logout: bool,
    sent_reset: bool,
    logout_reason: Option<String>,
    test_request_counter: u32,
    heartbeat_int: u32,
    heartbeat_int_ms: u64,
    last_received_time_dt: Instant,
    last_sent_time_dt: Instant,
    logon_timeout: u32,
    logon_timeout_ms: u64,
    logout_timeout: u32,
    logout_timeout_ms: u64,
    resend_range: Option<ResetRange>,
    message_queue: BTreeMap<u32, Message>,
    logger: Log,
    creation_time: Option<DateTime<Utc>>,
    next_sender_msg_seq_num: u32,
    next_target_msg_seq_num: u32,
}

impl<Log: Logger> SessionState<Log> {
    pub fn new(is_initiator: bool, logger: Log, heartbeat_int: u32, last_now: Instant) -> Self {
        SessionState {
            is_enabled: true,
            is_initiator,
            is_connected: false,
            received_logon: false,
            received_reset: false,
            sent_logon: false,
            sent_logout: false,
            sent_reset: false,
            logout_reason: None,
            test_request_counter: 0,
            heartbeat_int,
            heartbeat_int_ms: u64::from(heartbeat_int) * 1000,
            last_received_time_dt: last_now,
            last_sent_time_dt: last_now,
            logon_timeout: 10,
            logon_timeout_ms: 10 * 1000,
            logout_timeout: 2,
            logout_timeout_ms: 10 * 1000,
            resend_range: None,
            message_queue: BTreeMap::default(),
            logger,
            creation_time: None,
            next_sender_msg_seq_num: 1,
            next_target_msg_seq_num: 1,
        }
    }

    pub(crate) fn reset(&mut self) {
        self.next_sender_msg_seq_num = 1;
        self.next_target_msg_seq_num = 1;
        self.creation_time = Some(Utc::now());
    }

    pub(crate) fn should_send_logon(&self) -> bool {
        self.is_initiator && !self.sent_logon
    }

    pub(crate) fn logon_timed_out(&self, now: Instant) -> bool {
        logon_timed_out(
            now,
            self.logon_timeout_ms.into(),
            self.last_received_time_dt,
        )
    }
    pub(crate) fn logout_timed_out(&self, now: Instant) -> bool {
        logout_timed_out(
            now,
            self.sent_logout,
            self.logout_timeout_ms.into(),
            self.last_sent_time_dt,
        )
    }
    pub(crate) fn timed_out(&self, now: Instant) -> bool {
        timed_out(
            now,
            self.heartbeat_int_ms.into(),
            self.last_received_time_dt,
        )
    }
    pub(crate) fn within_heartbeat(&self, now: Instant) -> bool {
        within_heartbeat(
            now,
            self.heartbeat_int_ms.into(),
            self.last_sent_time_dt,
            self.last_received_time_dt,
        )
    }
    pub(crate) fn need_heartbeat(&self, now: Instant) -> bool {
        need_heartbeat(
            now,
            self.heartbeat_int_ms.into(),
            self.last_sent_time_dt,
            self.test_request_counter,
        )
    }
    pub(crate) fn need_test_request(&self, now: Instant) -> bool {
        need_test_request(
            now,
            self.heartbeat_int_ms.into(),
            self.last_received_time_dt,
            self.test_request_counter,
        )
    }
    pub(crate) fn resend_requested(&self) -> bool {
        self.resend_range
            .as_ref()
            .is_some_and(|range| !(range.begin_seq_num == 0 && range.end_seq_num == 0))
    }

    /// Get the session state's is enabled.
    pub(crate) fn is_enabled(&self) -> bool {
        self.is_enabled
    }

    /// Get a mutable reference to the session state's is enabled.
    pub(crate) fn is_enabled_mut(&mut self) -> &mut bool {
        &mut self.is_enabled
    }

    /// Set the session state's is enabled.
    pub(crate) fn set_is_enabled(&mut self, is_enabled: bool) {
        self.is_enabled = is_enabled;
    }

    /// Get the session state's is initiator.
    pub(crate) fn is_initiator(&self) -> bool {
        self.is_initiator
    }

    /// Get a mutable reference to the session state's is initiator.
    pub(crate) fn is_initiator_mut(&mut self) -> &mut bool {
        &mut self.is_initiator
    }

    /// Set the session state's is initiator.
    pub(crate) fn set_is_initiator(&mut self, is_initiator: bool) {
        self.is_initiator = is_initiator;
    }

    /// Get the session state's received logon.
    pub(crate) fn received_logon(&self) -> bool {
        self.received_logon
    }

    /// Get a mutable reference to the session state's received logon.
    pub(crate) fn received_logon_mut(&mut self) -> &mut bool {
        &mut self.received_logon
    }

    /// Set the session state's received logon.
    pub(crate) fn set_received_logon(&mut self, received_logon: bool) {
        self.received_logon = received_logon;
    }

    /// Get the session state's received reset.
    pub(crate) fn received_reset(&self) -> bool {
        self.received_reset
    }

    /// Get a mutable reference to the session state's received reset.
    pub(crate) fn received_reset_mut(&mut self) -> &mut bool {
        &mut self.received_reset
    }

    /// Set the session state's received reset.
    pub(crate) fn set_received_reset(&mut self, received_reset: bool) {
        self.received_reset = received_reset;
    }

    /// Get the session state's sent logon.
    pub(crate) fn sent_logon(&self) -> bool {
        self.sent_logon
    }

    /// Get a mutable reference to the session state's sent logon.
    pub(crate) fn sent_logon_mut(&mut self) -> &mut bool {
        &mut self.sent_logon
    }

    /// Set the session state's sent logon.
    pub(crate) fn set_sent_logon(&mut self, sent_logon: bool) {
        self.sent_logon = sent_logon;
    }

    /// Get the session state's sent logout.
    pub(crate) fn sent_logout(&self) -> bool {
        self.sent_logout
    }

    /// Get a mutable reference to the session state's sent logout.
    pub(crate) fn sent_logout_mut(&mut self) -> &mut bool {
        &mut self.sent_logout
    }

    /// Set the session state's sent logout.
    pub(crate) fn set_sent_logout(&mut self, sent_logout: bool) {
        self.sent_logout = sent_logout;
    }

    /// Get the session state's sent reset.
    pub(crate) fn sent_reset(&self) -> bool {
        self.sent_reset
    }

    /// Get a mutable reference to the session state's sent reset.
    pub(crate) fn sent_reset_mut(&mut self) -> &mut bool {
        &mut self.sent_reset
    }

    /// Set the session state's sent reset.
    pub(crate) fn set_sent_reset(&mut self, sent_reset: bool) {
        self.sent_reset = sent_reset;
    }

    /// Get a reference to the session state's logout reason.
    pub(crate) fn logout_reason(&self) -> Option<&String> {
        self.logout_reason.as_ref()
    }

    /// Get a mutable reference to the session state's logout reason.
    pub(crate) fn logout_reason_mut(&mut self) -> &mut Option<String> {
        &mut self.logout_reason
    }

    /// Set the session state's logout reason.
    pub(crate) fn set_logout_reason(&mut self, logout_reason: Option<String>) {
        self.logout_reason = logout_reason;
    }

    /// Get the session state's test request counter.
    pub(crate) fn test_request_counter(&self) -> u32 {
        self.test_request_counter
    }

    /// Get a mutable reference to the session state's test request counter.
    pub(crate) fn test_request_counter_mut(&mut self) -> &mut u32 {
        &mut self.test_request_counter
    }

    /// Set the session state's test request counter.
    pub(crate) fn set_test_request_counter(&mut self, test_request_counter: u32) {
        self.test_request_counter = test_request_counter;
    }

    /// Get the session state's heartbeat int.
    pub(crate) fn heartbeat_int(&self) -> u32 {
        self.heartbeat_int
    }

    /// Get a mutable reference to the session state's heartbeat int.
    pub(crate) fn heartbeat_int_mut(&mut self) -> &mut u32 {
        &mut self.heartbeat_int
    }

    /// Set the session state's heartbeat int.
    pub(crate) fn set_heartbeat_int(&mut self, heartbeat_int: u32) {
        self.heartbeat_int = heartbeat_int;
    }

    /// Get the session state's heartbeat int ms.
    pub(crate) fn heartbeat_int_ms(&self) -> u64 {
        self.heartbeat_int_ms
    }

    /// Get a mutable reference to the session state's heartbeat int ms.
    pub(crate) fn heartbeat_int_ms_mut(&mut self) -> &mut u64 {
        &mut self.heartbeat_int_ms
    }

    /// Set the session state's heartbeat int ms.
    pub(crate) fn set_heartbeat_int_ms(&mut self, heartbeat_int_ms: u64) {
        self.heartbeat_int_ms = heartbeat_int_ms;
    }

    /// Get the session state's last received time dt.
    pub(crate) fn last_received_time_dt(&self) -> Instant {
        self.last_received_time_dt
    }

    /// Get a mutable reference to the session state's last received time dt.
    pub(crate) fn last_received_time_dt_mut(&mut self) -> &mut Instant {
        &mut self.last_received_time_dt
    }

    /// Set the session state's last received time dt.
    pub(crate) fn set_last_received_time_dt(&mut self, last_received_time_dt: Instant) {
        self.last_received_time_dt = last_received_time_dt;
    }

    /// Get the session state's last sent time dt.
    pub(crate) fn last_sent_time_dt(&self) -> Instant {
        self.last_sent_time_dt
    }

    /// Get a mutable reference to the session state's last sent time dt.
    pub(crate) fn last_sent_time_dt_mut(&mut self) -> &mut Instant {
        &mut self.last_sent_time_dt
    }

    /// Set the session state's last sent time dt.
    pub(crate) fn set_last_sent_time_dt(&mut self, last_sent_time_dt: Instant) {
        self.last_sent_time_dt = last_sent_time_dt;
    }

    /// Get the session state's logon timeout.
    pub(crate) fn logon_timeout(&self) -> u32 {
        self.logon_timeout
    }

    /// Get a mutable reference to the session state's logon timeout.
    pub(crate) fn logon_timeout_mut(&mut self) -> &mut u32 {
        &mut self.logon_timeout
    }

    /// Set the session state's logon timeout.
    pub(crate) fn set_logon_timeout(&mut self, logon_timeout: u32) {
        self.logon_timeout = logon_timeout;
    }

    /// Get the session state's logon timeout ms.
    pub(crate) fn logon_timeout_ms(&self) -> u64 {
        self.logon_timeout_ms
    }

    /// Get a mutable reference to the session state's logon timeout ms.
    pub(crate) fn logon_timeout_ms_mut(&mut self) -> &mut u64 {
        &mut self.logon_timeout_ms
    }

    /// Set the session state's logon timeout ms.
    pub(crate) fn set_logon_timeout_ms(&mut self, logon_timeout_ms: u64) {
        self.logon_timeout_ms = logon_timeout_ms;
    }

    /// Get the session state's logout timeout.
    pub(crate) fn logout_timeout(&self) -> u32 {
        self.logout_timeout
    }

    /// Get a mutable reference to the session state's logout timeout.
    pub(crate) fn logout_timeout_mut(&mut self) -> &mut u32 {
        &mut self.logout_timeout
    }

    /// Set the session state's logout timeout.
    pub(crate) fn set_logout_timeout(&mut self, logout_timeout: u32) {
        self.logout_timeout = logout_timeout;
    }

    /// Get the session state's logout timeout ms.
    pub(crate) fn logout_timeout_ms(&self) -> u64 {
        self.logout_timeout_ms
    }

    /// Get a mutable reference to the session state's logout timeout ms.
    pub(crate) fn logout_timeout_ms_mut(&mut self) -> &mut u64 {
        &mut self.logout_timeout_ms
    }

    /// Set the session state's logout timeout ms.
    pub(crate) fn set_logout_timeout_ms(&mut self, logout_timeout_ms: u64) {
        self.logout_timeout_ms = logout_timeout_ms;
    }

    /// Get the session state's resend range.
    pub(crate) fn resend_range(&self) -> Option<&ResetRange> {
        self.resend_range.as_ref()
    }

    /// Get a mutable reference to the session state's resend range.
    pub(crate) fn resend_range_mut(&mut self) -> &mut Option<ResetRange> {
        &mut self.resend_range
    }

    /// Set the session state's resend range.
    pub(crate) fn set_resend_range(&mut self, resend_range: Option<ResetRange>) {
        self.resend_range = resend_range;
    }
    pub(crate) fn set_resend_range_begin_end(
        &mut self,
        begin: u32,
        end: u32,
        chunk_end: Option<u32>,
    ) {
        let chunk_end = chunk_end.unwrap_or(end);
        let resend_range = ResetRange {
            begin_seq_num: begin,
            end_seq_num: end,
            chunk_end_seq_num: Some(chunk_end),
        };
        self.resend_range.replace(resend_range);
    }

    /// Get a reference to the session state's message queue.
    pub(crate) fn message_queue(&self) -> &BTreeMap<u32, Message> {
        &self.message_queue
    }

    /// Get a mutable reference to the session state's message queue.
    pub(crate) fn message_queue_mut(&mut self) -> &mut BTreeMap<u32, Message> {
        &mut self.message_queue
    }

    /// Set the session state's message queue.
    pub(crate) fn set_message_queue(&mut self, message_queue: BTreeMap<u32, Message>) {
        self.message_queue = message_queue;
    }

    /// Get a reference to the session state's logger.
    pub(crate) fn logger(&self) -> &Log {
        &self.logger
    }

    /// Get a mutable reference to the session state's logger.
    pub(crate) fn logger_mut(&mut self) -> &mut Log {
        &mut self.logger
    }

    /// Set the session state's logger.
    pub(crate) fn set_logger(&mut self, logger: Log) {
        self.logger = logger;
    }

    pub(crate) fn creation_time(&self) -> Option<DateTime<Utc>> {
        self.creation_time
    }

    pub(crate) fn set_creation_time(&mut self, creation_time: Option<DateTime<Utc>>) {
        self.creation_time = creation_time;
    }

    pub(crate) fn next_sender_msg_seq_num(&self) -> u32 {
        self.next_sender_msg_seq_num
    }

    pub(crate) fn set_next_sender_msg_seq_num(&mut self, seq_num: u32) {
        self.next_sender_msg_seq_num = seq_num
    }

    pub(crate) fn next_target_msg_seq_num(&self) -> u32 {
        self.next_target_msg_seq_num
    }

    pub(crate) fn set_next_target_msg_seq_num(&mut self, seq_num: u32) {
        self.next_target_msg_seq_num = seq_num;
    }

    pub(crate) fn clear_queue(&mut self) {
        self.message_queue.clear();
    }

    pub(crate) fn queue(&mut self, msg_seq_num: u32, msg: Message) {
        self.message_queue.insert(msg_seq_num, msg);
    }

    pub(crate) fn dequeue(&mut self, next_target_msg_seq_num: u32) -> Option<Message> {
        self.message_queue.remove(&next_target_msg_seq_num)
    }

    pub(crate) fn is_connected(&self) -> bool {
        self.is_connected
    }

    pub(crate) fn is_connected_mut(&mut self) -> &mut bool {
        &mut self.is_connected
    }

    pub(crate) fn set_is_connected(&mut self, is_connected: bool) {
        self.is_connected = is_connected;
    }

    pub(crate) fn incr_next_sender_msg_seq_num(&mut self) {
        self.next_sender_msg_seq_num += 1;
    }

    pub(crate) fn incr_next_target_msg_seq_num(&mut self) {
        self.next_target_msg_seq_num += 1;
    }
}

/// All time args are in milliseconds
pub(crate) fn logon_timed_out(
    now: Instant,
    logon_timeout: u128,
    last_received_time: Instant,
) -> bool {
    (now - last_received_time).as_millis() >= logon_timeout
}

/// All time args are in milliseconds
pub(crate) fn timed_out(
    now: Instant,
    heart_bt_int_millis: u128,
    last_received_time: Instant,
) -> bool {
    let elapsed = (now - last_received_time).as_millis();
    elapsed as f64 >= (2.4 * heart_bt_int_millis as f64)
}

/// All time args are in milliseconds
pub(crate) fn logout_timed_out(
    now: Instant,
    sent_logout: bool,
    logout_timeout: u128,
    last_sent_time: Instant,
) -> bool {
    sent_logout && (now - last_sent_time).as_millis() >= logout_timeout
}

pub(crate) fn within_heartbeat(
    now: Instant,
    heartbeat_int_ms: u128,
    last_sent_time: Instant,
    last_received_time: Instant,
) -> bool {
    ((now - last_sent_time).as_millis() < heartbeat_int_ms)
        && ((now - last_received_time).as_millis() < heartbeat_int_ms)
}

pub(crate) fn need_heartbeat(
    now: Instant,
    heartbeat_int_ms: u128,
    last_sent_time: Instant,
    test_request_counter: u32,
) -> bool {
    0 == test_request_counter && (now - last_sent_time).as_millis() >= heartbeat_int_ms
}

pub(crate) fn need_test_request(
    now: Instant,
    heartbeat_int_ms: u128,
    last_received_time: Instant,
    test_request_counter: u32,
) -> bool {
    (now - last_received_time).as_millis()
        >= (1.2 * ((u128::from(test_request_counter) + 1) * heartbeat_int_ms) as f64) as u128
}
