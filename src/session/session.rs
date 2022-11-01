use std::{
    io::Read,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use crate::session::SessionState;
use crate::session::SessionId;
use crate::session::Application;
use crate::session::SessionSchedule;
use crate::session::Responder;
use crate::message::Message;
use crate::message_store::MessageStore;
use crate::message_store_factory::MessageStoreFactory;
use crate::data_dictionary::DataDictionary;
use crate::data_dictionary_provider::DataDictionaryProvider;
use crate::message_factory::MessageFactory;
use crate::log::Log;
use crate::log::NoLogger;
use crate::log_factory::LogFactory;
use crate::fields::*;

const BUF_SIZE: usize = 4096;

//TODO: dyn to generic?
pub struct Session {
    application: Box<dyn Application>,
    session_id: SessionId,
    data_dictionary_provider: Box<dyn DataDictionaryProvider>, // TODO: REMOVE candidate
    schedule: SessionSchedule,
    msg_factory: Box<dyn MessageFactory>,
    app_does_early_intercept: bool,
    sender_default_appl_ver_id: String,
    session_data_dictionary: DataDictionary, //Option?
    application_data_dictionary: DataDictionary, //Option?
    log: Box<dyn Log>,
    state: SessionState,
    persist_messages: bool,
    reset_on_disconnect: bool,
    send_redundant_resend_requests: bool,
    resend_session_level_rejects: bool,
    validate_length_and_checksum: bool,
    check_comp_id: bool,
    time_stamp_precision: String,
    enable_last_msg_seq_num_processed: bool,
    max_messages_in_resend_request: u32,
    send_logout_before_timeout_disconnect: bool,
    ignore_poss_dup_resend_requests: bool,
    requires_orig_sending_time: bool,
    check_latency: bool,
    max_latency: u32,
    responder: Option<Box<dyn Responder>>,
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

impl Session {
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
        let mut application = app;
        let schedule = session_schedule;
        let app_does_early_intercept = false; //TODO app is IApplicationExt
        let session_data_dictionary = data_dictionary_provider.get_session_data_dictionary(&session_id.begin_string);
        let application_data_dictionary = if session_id.is_fixt {
            data_dictionary_provider.get_application_data_dictionary(sender_default_appl_ver_id)
        }else{
            session_data_dictionary.clone()
        };
        let log = log_factory.as_ref().map(|l| l.create(&session_id)).unwrap_or(Box::new(NoLogger));
        let msg_store = store_factory.create(&session_id);
        let mut state = SessionState::new(is_initiator, log, heartbeat_int, msg_store);
        let log = log_factory.map(|l| l.create(&session_id)).unwrap_or(Box::new(NoLogger)); //TODO clone?
        // Configuration defaults.
        // Will be overridden by the SessionFactory with values in the user's configuration.
        let persist_messages = true;
        let reset_on_disconnect = false;
        let send_redundant_resend_requests = false;
        let resend_session_level_rejects = false;
        let validate_length_and_checksum = true;
        let check_comp_id = true;
        let time_stamp_precision = "MILLISECOND".into(); //TODO
        let enable_last_msg_seq_num_processed = false;
        let max_messages_in_resend_request = 0;
        let send_logout_before_timeout_disconnect = false;
        let ignore_poss_dup_resend_requests = false;
        let requires_orig_sending_time = true;
        let check_latency = true;
        let max_latency = 120;

        if is_session_time(&schedule) {
            // Reset("Out of SessionTime (Session construction)")
            // ---
            // if(this.IsLoggedOn)
            //     GenerateLogout(logoutMessage);
            // Disconnect("Resetting...");
            // state_.Reset(loggedReason);
            state.reset("Out of SessionTime (Session construction)");
        } else {
            // Reset("New session")
            // ---
            // if(this.IsLoggedOn)
            //     GenerateLogout(logoutMessage);
            // Disconnect("Resetting...");
            // state_.Reset(loggedReason);
            state.reset("New session");
        }

        // TODO register session
        application.on_create(&session_id).unwrap(); //TODO handle err
        log.on_event("Created session");

        Session {
            application,
            session_id,
            data_dictionary_provider,
            schedule,
            msg_factory,
            app_does_early_intercept,
            sender_default_appl_ver_id: sender_default_appl_ver_id.into(),
            session_data_dictionary,
            application_data_dictionary,
            log,
            state,
            persist_messages,
            reset_on_disconnect,
            send_redundant_resend_requests,
            resend_session_level_rejects,
            validate_length_and_checksum,
            check_comp_id,
            time_stamp_precision,
            enable_last_msg_seq_num_processed,
            max_messages_in_resend_request,
            send_logout_before_timeout_disconnect,
            ignore_poss_dup_resend_requests,
            requires_orig_sending_time,
            check_latency,
            max_latency,
            responder: None,
        }
    }

    pub fn next<'a>(&'a mut self) {
        //TODO
        // if !self.has_responder() {
        //     return;
        // }

        if !self.is_session_time() {
            if self.state.is_initiator() {
                self.reset(Some("Out of SessionTime (Session.next())"), None);
            }else{
                self.reset(Some("Out of SessionTime (Session.next())"), Some("Message received outside of session time"));
            }
            return;
        }

        if self.is_new_session() {
            self.state.reset("New session (detected in Next())");
        }

        if self.state.is_enabled() {
            if !self.is_logged_on() {
                return;
            }

            if !self.state.sent_logout() {
                self.log.on_event("Initiated logout request");
                self.generate_logout(self.state.logout_reason());
            }
        }

        if !self.state.received_logon() {
            if self.state.should_send_logon() && self.is_time_to_generate_logon() {
                if self.generate_logon() {
                    self.log.on_event("Initiated logon request");
                }else{
                    self.log.on_event("Error during logon request initiation");
                }
            }else if self.state.sent_logon() && self.state.logon_timed_out() {
                self.disconnect("Timed out waiting for logon response");
            }
            return;
        }

        if self.state.heartbeat_int() == 0 {
            return;
        }

        if self.state.logout_timed_out() {
            self.disconnect("Timed out waiting for logout response");
        }

        if self.state.within_heartbeat() {
            return;
        }

        if self.state.timed_out() {
            if self.send_logout_before_timeout_disconnect {
                self.generate_logout(None);
            }
            self.disconnect("Timed out waiting for heartbeat")
        }else{
            if self.state.need_test_request() {
                self.generate_test_request("TEST");
                self.state.set_test_request_counter(self.state.test_request_counter() + 1);
                self.log.on_event("Sent test request TEST")
            }else if self.state.need_heartbeat() {
                self.generate_heartbeat();
            }
        }
    }

    pub fn reset_heartbeat(&mut self) {
        //self.last_reset = Instant::now()
        todo!()
    }

    fn read_frame(&mut self) -> Option<Vec<u8>> {
        dbg!("read_frame");
        // match self.stream.lock() {
        //     Ok(mut guard) => {
        //         let read = guard
        //             .read(&mut self.buffer)
        //             .ok()
        //             .or_else(|| Some(0))
        //             .unwrap();
        //         if read != 0 {
        //             let buf = &self.buffer[0..read];
        //             dbg!(buf);
        //             self.msg_buffer.extend(buf);
        //             //Parser::read_fix(&mut self.msg_buffer)
        //             todo!("Parser")
        //         } else {
        //             None
        //         }
        //     }
        //     Err(_) => None,
        // }
        todo!()
    }

    fn is_session_time(&self) -> bool {
        is_session_time(&self.schedule)
    }

    fn is_new_session(&self) -> bool { todo!() }
    fn is_logged_on(&self) -> bool { self.state.sent_logon() && self.state.received_logon() }
    fn is_time_to_generate_logon(&self) -> bool { todo!() }

    fn refresh(&mut self) {
        todo!()
    }

    fn refresh_on_logon(&self) -> bool {
        todo!()
    }

    fn reset(&mut self, logged_reason: Option<&str>, logout_message: Option<&str>) {
        todo!()
    }

    fn should_send_reset(&self) -> bool {
        todo!()
    }

    fn reset_on_logon(&self) -> bool {
        todo!()
    }

    fn generate_logon(&mut self) -> bool {
        let mut logon = self.msg_factory.create(&self.session_id.begin_string, MsgType::LOGON).unwrap(); // TODO handle unwrap
        logon.set_field_deref(EncryptMethod::new(EncryptMethod::NONE), None);
        logon.set_field_deref(HeartBtInt::new(self.state.heartbeat_int()), None);

        if self.session_id.is_fixt {
            logon.set_field_deref(DefaultApplVerID::new(self.sender_default_appl_ver_id.clone()), None);
        }
        if self.refresh_on_logon() {
            self.refresh();
        }
        if self.reset_on_logon() {
            self.state.reset("ResetOnLogon");
        }
        if self.should_send_reset() {
            logon.set_field_deref(ResetSeqNumFlag::new(true), None);
        }

        self.initialize_header(&mut logon);
        self.state.set_last_received_time(Instant::now());
        self.state.set_test_request_counter(0);
        self.state.set_sent_logon(true);
        return self.send_raw(logon, 0);
    }

    fn generate_logout(&mut self, reason: Option<String>) {
        todo!()
    }

    fn generate_heartbeat(&mut self) {
        todo!()
    }
    fn generate_test_request(&mut self, reason: &str) {
        todo!()
    }

    fn disconnect(&mut self, reason: &str) {
        todo!()
    }

    fn send_raw(&mut self, mut message: Message, seq_num: u32) -> bool {
        // lock (sync_)
        // {
        //     string msgType = message.Header.GetString(Fields.Tags.MsgType);
        //     InitializeHeader(message, seqNum);
        //     if (Message.IsAdminMsgType(msgType))
        //     {
        //         this.Application.ToAdmin(message, this.SessionID);
        //         if (MsgType.LOGON.Equals(msgType) && !state_.ReceivedReset)
        //         {
        //             Fields.ResetSeqNumFlag resetSeqNumFlag = new QuickFix.Fields.ResetSeqNumFlag(false);
        //             if (message.IsSetField(resetSeqNumFlag))
        //                 message.GetField(resetSeqNumFlag);
        //             if (resetSeqNumFlag.getValue())
        //             {
        //                 state_.Reset("ResetSeqNumFlag");
        //                 message.Header.SetField(new Fields.MsgSeqNum(state_.GetNextSenderMsgSeqNum()));
        //             }
        //             state_.SentReset = resetSeqNumFlag.Obj;
        //         }
        //     }
        //     else
        //     {
        //         try
        //         {
        //             this.Application.ToApp(message, this.SessionID);
        //         }
        //         catch (DoNotSend)
        //         {
        //             return false;
        //         }
        //     }
        //     string messageString = message.ToString();
        //     if (0 == seqNum)
        //         Persist(message, messageString);
        //     return Send(messageString);
        // }
        let message_string = message.to_string();
        if 0 == seq_num {
            // TODO persist
        }
        self.send(message_string)
    }
    fn send(&mut self, message: String) -> bool {
        if let Some(responder) = self.responder.as_mut() {
            self.log.on_outgoing(message.as_str());
            responder.send(message)
        }else{
            false
        }
    }

    fn initialize_header(&mut self, message: &mut Message) {
        todo!("{:?}", message)
    }
}

fn is_session_time(session_schedule: &SessionSchedule) -> bool {
    todo!("{:?}", session_schedule)
}
