use std::cmp;
use std::time::Instant;

use crate::data_dictionary::DataDictionary;
use crate::data_dictionary::MessageValidationError;
use crate::data_dictionary_provider::DataDictionaryProvider;
use crate::fields::*;
use crate::log::Log;
use crate::log::NoLogger;
use crate::log_factory::LogFactory;
use crate::message::Message;
use crate::message::MessageParseError;
use crate::message_builder::MessageBuilder;
use crate::message_factory::MessageFactory;
use crate::message_factory::MessageFactoryError;
use crate::message_store::MessageStoreError;
use crate::message_store_factory::MessageStoreFactory;
use crate::session::Application;
use crate::session::ApplicationError;
use crate::session::Responder;
use crate::session::SessionId;
use crate::session::SessionSchedule;
use crate::session::SessionState;
use crate::tags;

const _BUF_SIZE: usize = 4096;

//TODO: dyn to generic?
pub struct Session {
    application: Box<dyn Application>,
    session_id: SessionId,
    data_dictionary_provider: Box<dyn DataDictionaryProvider>, // TODO: REMOVE candidate
    schedule: SessionSchedule,
    msg_factory: Box<dyn MessageFactory>,
    app_does_early_intercept: bool,
    sender_default_appl_ver_id: String,
    target_default_appl_ver_id: Option<u32>,
    session_data_dictionary: DataDictionary,     //Option?
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
    refresh_on_logon: bool,
    reset_on_logon: bool,
    reset_on_logout: bool,
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
        let session_data_dictionary =
            data_dictionary_provider.get_session_data_dictionary(&session_id.begin_string);
        let application_data_dictionary = if session_id.is_fixt {
            data_dictionary_provider.get_application_data_dictionary(sender_default_appl_ver_id)
        } else {
            session_data_dictionary.clone()
        };
        let log = log_factory
            .as_ref()
            .map(|l| l.create(&session_id))
            .unwrap_or_else(|| Box::new(NoLogger));
        let msg_store = store_factory.create(&session_id);
        let mut state = SessionState::new(is_initiator, log, heartbeat_int, msg_store);
        let log = log_factory
            .map(|l| l.create(&session_id))
            .unwrap_or_else(|| Box::new(NoLogger)); //TODO clone?
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
            state.reset(Some("Out of SessionTime (Session construction)"));
        } else {
            // Reset("New session")
            // ---
            // if(this.IsLoggedOn)
            //     GenerateLogout(logoutMessage);
            // Disconnect("Resetting...");
            // state_.Reset(loggedReason);
            state.reset(Some("New session"));
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
            target_default_appl_ver_id: None,
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
            refresh_on_logon: false,
            reset_on_logon: false,
            reset_on_logout: false,
        }
    }

    pub fn next(&mut self) {
        if self.responder.is_none() {
            return;
        }

        if !self.is_session_time() {
            if self.state.is_initiator() {
                self.reset(Some("Out of SessionTime (Session.next())"), None);
            } else {
                self.reset(
                    Some("Out of SessionTime (Session.next())"),
                    Some("Message received outside of session time"),
                );
            }
            return;
        }

        if self.is_new_session() {
            self.state.reset(Some("New session (detected in Next())"));
        }

        if self.state.is_enabled() {
            if !self.is_logged_on() {
                return;
            }

            if !self.state.sent_logout() {
                self.log.on_event("Initiated logout request");
                self.generate_logout(self.state.logout_reason().cloned());
            }
        }

        if !self.state.received_logon() {
            if self.state.should_send_logon() && self.is_time_to_generate_logon() {
                if self.generate_logon() {
                    self.log.on_event("Initiated logon request");
                } else {
                    self.log.on_event("Error during logon request initiation");
                }
            } else if self.state.sent_logon() && self.state.logon_timed_out() {
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
        } else if self.state.need_test_request() {
            self.generate_test_request("TEST");
            self.state
                .set_test_request_counter(self.state.test_request_counter() + 1);
            self.log.on_event("Sent test request TEST")
        } else if self.state.need_heartbeat() {
            self.generate_heartbeat();
        }
    }

    fn is_session_time(&self) -> bool {
        is_session_time(&self.schedule)
    }

    fn is_new_session(&self) -> bool {
        self.state
            .creation_time()
            .map(|ct| self.schedule.is_new_session(ct, Instant::now()))
            .unwrap_or(false)
    }

    fn is_logged_on(&self) -> bool {
        self.state.sent_logon() && self.state.received_logon()
    }
    // FIXME
    fn is_time_to_generate_logon(&self) -> bool {
        true
    }

    fn refresh(&mut self) {
        self.state.refresh()
    }

    fn refresh_on_logon(&self) -> bool {
        self.refresh_on_logon
    }

    fn reset(&mut self, logged_reason: Option<&str>, logout_message: Option<&str>) {
        if self.is_logged_on() {
            self.generate_logout(logout_message.map(|v| v.into()));
        }
        self.disconnect("Resetting...");
        self.state.reset(logged_reason);
    }

    fn should_send_reset(&self) -> bool {
        //TODO replace hardcoded FIX.4.1
        self.session_id.begin_string.as_str() >= "FIX.4.1" && self.reset_on_logon
            || self.reset_on_logout
            || self.reset_on_disconnect
                && self.state.next_sender_msg_seq_num() == 1
                && self.state.next_target_msg_seq_num() == 1
    }

    fn reset_on_logon(&self) -> bool {
        self.reset_on_logon
    }

    fn generate_logon(&mut self) -> bool {
        let mut logon = self
            .msg_factory
            .create(&self.session_id.begin_string, MsgType::LOGON)
            .unwrap(); // TODO handle unwrap
        logon.set_field_deref(EncryptMethod::new(EncryptMethod::NONE), None);
        logon.set_field_deref(HeartBtInt::new(self.state.heartbeat_int()), None);

        if self.session_id.is_fixt {
            logon.set_field_deref(
                DefaultApplVerID::new(self.sender_default_appl_ver_id.clone()),
                None,
            );
        }
        if self.refresh_on_logon() {
            self.refresh();
        }
        if self.reset_on_logon() {
            self.state.reset(Some("ResetOnLogon"));
        }
        if self.should_send_reset() {
            logon.set_field_deref(ResetSeqNumFlag::new(true), None);
        }

        self.initialize_header(&mut logon, None);
        self.state.set_last_received_time_dt(Instant::now());
        self.state.set_test_request_counter(0);
        self.state.set_sent_logon(true);
        self.send_raw(logon, 0).is_ok()
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
        if let Some(responder) = &mut self.responder {
            self.log.on_event(
                format!("Session {} disconnecting: {}", self.session_id, reason).as_str(),
            );
            responder.disconnect();
            self.responder = None;
        } else {
            self.log.on_event(
                format!(
                    "Session {} already disconnected: {}",
                    self.session_id, reason
                )
                .as_str(),
            );
        }

        if self.state.received_logon() || self.state.sent_logon() {
            self.state.set_received_logon(false);
            self.state.set_sent_logon(false);
            self.application.on_logout(&self.session_id);
        }

        self.state.set_sent_logout(false);
        self.state.set_received_reset(false);
        self.state.set_sent_reset(false);
        self.state.clear_queue();
        self.state.set_logout_reason(None);
        if self.reset_on_disconnect {
            self.state.reset(Some("ResetOnDisconnect"));
        }
        self.state.set_resend_range_begin_end(0, 0, None);
    }

    fn send_raw(&mut self, mut message: Message, seq_num: u32) -> Result<bool, ApplicationError> {
        let msg_type = message.header().get_string(tags::MsgType);
        self.initialize_header(&mut message, Some(seq_num));
        let message = if Message::is_admin_msg_type(msg_type.as_str()) {
            let message = match self.application.to_admin(message, &self.session_id) {
                Ok(message) => Ok(message),
                Err(ApplicationError::DoNotSend(message)) => Ok(*message),
                Err(e) => Err(e),
            }?;
            if MsgType::LOGON == msg_type && !self.state.received_reset() {
                // Fields.ResetSeqNumFlag resetSeqNumFlag = new QuickFix.Fields.ResetSeqNumFlag(false);
                // if (message.IsSetField(resetSeqNumFlag))
                //     message.GetField(resetSeqNumFlag);
                // if (resetSeqNumFlag.getValue())
                // {
                //     state_.Reset("ResetSeqNumFlag");
                //     message.Header.SetField(new Fields.MsgSeqNum(state_.GetNextSenderMsgSeqNum()));
                // }
                // state_.SentReset = resetSeqNumFlag.Obj;
                todo!()
            }
            Ok(message)
        } else {
            self.application.to_admin(message, &self.session_id)
        };

        if matches!(message, Err(ApplicationError::DoNotSend(_))) {
            return Ok(false);
        }
        let mut message = message?;
        let message_string = message.to_string_mut();
        if 0 == seq_num {
            self.persist(&message, &message_string);
        }
        Ok(self.send(message_string))
    }
    fn send(&mut self, message: String) -> bool {
        if let Some(responder) = self.responder.as_mut() {
            self.log.on_outgoing(message.as_str());
            responder.send(message)
        } else {
            false
        }
    }

    fn initialize_header(&mut self, message: &mut Message, seq_num: Option<u32>) {
        let seq_num = seq_num.unwrap_or(0);
        todo!("{:?} {}", message, seq_num)
    }

    pub fn session_id(&self) -> &SessionId {
        &self.session_id
    }

    pub fn log(&mut self) -> &mut Box<dyn Log> {
        &mut self.log
    }

    pub fn next_msg(&mut self, msg: String) {
        self.log.on_incoming(msg.as_str());

        if !self.is_session_time() {
            self.reset(
                Some("Out of SessionTime (Session::next_msg(message))"),
                Some("Message received outside of session time"),
            );
        }

        if self.is_new_session() {
            self.state
                .reset(Some("New session (detected in next::msg(message))"))
        }

        let result = self.next_msg_handler(msg);

        if let Err(e) = result {
            match e {
                HandleError::UnsupportedVersion { expected, actual } => todo!(),
                HandleError::MessageFactoryError(_) => todo!(),
                HandleError::MessageParseError(_) => todo!(),
                HandleError::MessageValidationError(_) => todo!(),
                HandleError::String(_) => todo!(),
            }
        }
        // }
        // catch (InvalidMessage e)
        // {
        //     this.Log.OnEvent(e.Message);

        //     try
        //     {
        //         if (MsgType.LOGON.Equals(msgBuilder.MsgType.Obj))
        //             Disconnect("Logon message is not valid");
        //     }
        //     catch (MessageParseError)
        //     { }

        //     throw e;
        // }
        // catch (TagException e)
        // {
        //     if (null != e.InnerException)
        //         this.Log.OnEvent(e.InnerException.Message);
        //     GenerateReject(msgBuilder, e.sessionRejectReason, e.Field);
        // }
        // catch (UnsupportedVersion uvx)
        // {
        //     if (MsgType.LOGOUT.Equals(msgBuilder.MsgType.Obj))
        //     {
        //         NextLogout(message);
        //     }
        //     else
        //     {
        //         this.Log.OnEvent(uvx.ToString());
        //         GenerateLogout(uvx.Message);
        //         state_.IncrNextTargetMsgSeqNum();
        //     }
        // }
        // catch (UnsupportedMessageType e)
        // {
        //     this.Log.OnEvent("Unsupported message type: " + e.Message);
        //     GenerateBusinessMessageReject(message, Fields.BusinessRejectReason.UNKNOWN_MESSAGE_TYPE, 0);
        // }
        // catch (FieldNotFoundException e)
        // {
        //     this.Log.OnEvent("Rejecting invalid message, field not found: " + e.Message);
        //     if ((SessionID.BeginString.CompareTo(FixValues.BeginString.FIX42) >= 0) && (message.IsApp()))
        //     {
        //         GenerateBusinessMessageReject(message, Fields.BusinessRejectReason.CONDITIONALLY_REQUIRED_FIELD_MISSING, e.Field);
        //     }
        //     else
        //     {
        //         if (MsgType.LOGON.Equals(msgBuilder.MsgType.Obj))
        //         {
        //             this.Log.OnEvent("Required field missing from logon");
        //             Disconnect("Required field missing from logon");
        //         }
        //         else
        //             GenerateReject(msgBuilder, new QuickFix.FixValues.SessionRejectReason(SessionRejectReason.REQUIRED_TAG_MISSING, "Required Tag Missing"), e.Field);
        //     }
        // }
        // catch (RejectLogon e)
        // {
        //     GenerateLogout(e.Message);
        //     Disconnect(e.ToString());
        // }

        self.next()
    }

    fn next_msg_handler(&mut self, msg: String) -> Result<(), HandleError> {
        let msg_type = Message::identify_type(msg.as_str())?;
        let begin_string = Message::extract_begin_string(msg.as_str())?;
        let mut message = self.msg_factory.create(begin_string.as_str(), msg_type)?;
        message.from_string(
            msg.as_str(),
            self.validate_length_and_checksum,
            Some(&self.session_data_dictionary),
            Some(&self.application_data_dictionary),
            Some(&*self.msg_factory),
            false,
        );
        if self.app_does_early_intercept {
            //self.application.from_early_intercept(&message, self.session_id);
            todo!()
        }

        let header = message.header();

        if begin_string != self.session_id.begin_string {
            return Err(HandleError::UnsupportedVersion {
                actual: begin_string.into(),
                expected: self.session_id.begin_string.clone(),
            });
        }

        if MsgType::LOGON == msg_type {
            if self.session_id.is_fixt {
                self.target_default_appl_ver_id = todo!();
            } else {
                self.target_default_appl_ver_id = Some(Message::get_appl_ver_id(&begin_string)?);
            }
        }

        if self.session_id.is_fixt && !Message::is_admin_msg_type(msg_type) {
            DataDictionary::validate(
                &message,
                Some(&self.session_data_dictionary),
                &self.application_data_dictionary,
                &begin_string,
                msg_type,
            )?;
        } else {
            DataDictionary::validate(
                &message,
                Some(&self.session_data_dictionary),
                &self.session_data_dictionary,
                &begin_string,
                msg_type,
            )?;
        }

        if MsgType::LOGON == msg_type {
            self.next_logon(message)
        } else if MsgType::LOGOUT == msg_type {
            self.next_logout(message)
        } else if !self.is_logged_on() {
            self.disconnect(
                format!("Received msg type '{}' when not logged on", msg_type).as_str(),
            );
            Ok(())
        } else if MsgType::HEARTBEAT == msg_type {
            self.next_heartbeat(message)
        } else if MsgType::TEST_REQUEST == msg_type {
            self.next_test_request(message)
        } else if MsgType::SEQUENCE_RESET == msg_type {
            self.next_sequence_reset(message)
        } else if MsgType::RESEND_REQUEST == msg_type {
            self.next_resend_request(message)
        } else if !self.verify(message) {
            Ok(())
        } else {
            self.state.incr_next_target_msg_seq_num();
            Ok(())
        }
    }

    fn persist(&mut self, message: &Message, message_string: &str) {
        if self.persist_messages {
            let msg_seq_num = message.header().get_int(tags::MsgSeqNum).unwrap();
            self.state.set(msg_seq_num, message_string);
        }
        self.state.incr_next_sender_msg_seq_num();
    }

    fn next_logon(&self, message: Message) -> Result<(), HandleError> {
        todo!()
    }
    fn next_logout(&self, message: Message) -> Result<(), HandleError> {
        todo!()
    }
    fn next_heartbeat(&self, message: Message) -> Result<(), HandleError> {
        todo!()
    }
    fn next_test_request(&self, message: Message) -> Result<(), HandleError> {
        todo!()
    }
    fn next_sequence_reset(&self, message: Message) -> Result<(), HandleError> {
        todo!()
    }
    fn next_resend_request(&self, message: Message) -> Result<(), HandleError> {
        todo!()
    }

    /// This will pass the message into the from_admin / from_app methods from the Application
    fn verify(&mut self, message: Message) -> bool {
        self.verify_opt(message, true, true)
    }
    /// This will pass the message into the from_admin / from_app methods from the Application
    fn verify_opt(&mut self, message: Message, check_too_high: bool, check_too_low: bool) -> bool {
        // int msgSeqNum = 0;
        // string msgType = "";
        let mut msg_seq_num = 0;
        let msg_type = message.header().get_string(tags::MsgType);

        let sender_comp_id = message.header().get_string(tags::SenderCompID);
        let target_comp_id = message.header().get_string(tags::TargetCompID);

        if !self.is_correct_comp_id(sender_comp_id, target_comp_id) {
            //TODO
            //self.generate_reject(message, SessionRejectReason.COMPID_PROBLEM);
            self.generate_logout(None);
            return false;
        }
        if check_too_high || check_too_low {
            msg_seq_num = message.header().get_int(tags::MsgSeqNum).unwrap();
        }

        if check_too_high && self.is_target_too_high(msg_seq_num) {
            self.do_target_too_high(message, msg_seq_num);
            return false;
        }
        if check_too_low && self.is_target_too_low(msg_seq_num) {
            self.do_target_too_low(message, msg_seq_num);
            return false;
        }

        if (check_too_high || check_too_low) && self.state.resend_requested() {
            if let Some(range) = self.state.resend_range() {
                if msg_seq_num >= range.end_seq_num {
                    self.log.on_event(
                        format!(
                            "ResendRequest for messages FROM: {} TO: {} has been satisfied.",
                            range.begin_seq_num, range.end_seq_num
                        )
                        .as_str(),
                    );
                    self.state.set_resend_range_begin_end(0, 0, None);
                } else if let Some(chunk) = range.chunk_end_seq_num {
                    if msg_seq_num >= chunk {
                        self.log.on_event(format!("Chunked ResendRequest for messages FROM: {} TO: {} has been satisfied.", range.begin_seq_num, chunk).as_str());
                        let new_chunk_end_seq_no = cmp::min(
                            range.end_seq_num,
                            chunk + self.max_messages_in_resend_request,
                        );
                        self.generate_resend_request_range(
                            message.header().get_string(tags::BeginString),
                            chunk + 1,
                            new_chunk_end_seq_no,
                        );
                        self.state
                            .resend_range_mut()
                            .as_mut()
                            .unwrap()
                            .chunk_end_seq_num = Some(new_chunk_end_seq_no);
                    }
                }
            }
        }

        if !self.is_good_time(&message) {
            self.log().on_event("Sending time accuracy problem");
            //TODO
            //self.generate_reject(message, SessionRejectReason.SENDING_TIME_ACCURACY_PROBLEM);
            self.generate_logout(None);
            return false;
        }
        // }
        // catch (System.Exception e)
        // {
        //     this.Log.OnEvent("Verify failed: " + e.Message);
        //     Disconnect("Verify failed: " + e.Message);
        //     return false;
        // }

        self.state.set_last_received_time_dt(Instant::now());
        self.state.set_test_request_counter(0);

        if Message::is_admin_msg_type(&msg_type) {
            self.application
                .from_admin(message, &self.session_id)
                .unwrap();
        } else {
            self.application
                .from_app(message, &self.session_id)
                .unwrap();
        }
        true
    }

    fn is_correct_comp_id(&self, sender_comp_id: String, target_comp_id: String) -> bool {
        !self.check_comp_id || (self.session_id.sender_comp_id == sender_comp_id && self.session_id.target_comp_id == target_comp_id)
    }

    fn is_target_too_high(&self, msg_seq_num: u32) -> bool {
        msg_seq_num > self.state.next_target_msg_seq_num()
    }

    fn is_target_too_low(&self, msg_seq_num: u32) -> bool {
        msg_seq_num < self.state.next_target_msg_seq_num()
    }

    fn do_target_too_high(&self, msg: Message, msg_seq_num: u32) -> () {
        todo!()
    }

    fn do_target_too_low(&self, message: Message, msg_seq_num: u32) -> () {
        todo!()
    }

    fn is_good_time(&self, message: &Message) -> bool {
        // if (!CheckLatency)
        //     return true;
        if !self.check_latency {
            return true;
        }

        let sending_time = message.header().get_datetime(tags::SendingTime);
        let timespam = Instant::now() - sending_time;

        timespam.as_millis() > self.max_latency as u128
    }

    fn generate_resend_request_range(
        &mut self,
        beginstring: String,
        chunk: u32,
        new_chunk_end_seq_no: u32,
    ) -> () {
        todo!()
    }
}

fn is_session_time(session_schedule: &SessionSchedule) -> bool {
    todo!("{:?}", session_schedule)
}

pub enum HandleError {
    UnsupportedVersion { expected: String, actual: String },
    MessageFactoryError(MessageFactoryError),
    MessageParseError(MessageParseError),
    MessageValidationError(MessageValidationError),
    String(String),
}

impl From<MessageFactoryError> for HandleError {
    fn from(e: MessageFactoryError) -> Self {
        HandleError::MessageFactoryError(e)
    }
}
impl From<MessageParseError> for HandleError {
    fn from(e: MessageParseError) -> Self {
        HandleError::MessageParseError(e)
    }
}
impl From<MessageValidationError> for HandleError {
    fn from(e: MessageValidationError) -> Self {
        HandleError::MessageValidationError(e)
    }
}
impl From<String> for HandleError {
    fn from(e: String) -> Self {
        HandleError::String(e)
    }
}
