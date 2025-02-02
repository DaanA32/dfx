use std::cmp;
use std::cmp::min;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::mpsc::channel;
use std::time::Duration;
use std::time::Instant;

use chashmap::CHashMap;
use chrono::NaiveDateTime;
use chrono::Utc;
use dfx_base::data_dictionary::TagException;
use dfx_base::field_map::FieldValue;
use dfx_base::fix_values::ApplVerID;
use dfx_base::fix_values::BusinessRejectReason;
use dfx_base::fix_values::SessionRejectReason;
use lazy_static::lazy_static;

use dfx_base::data_dictionary::DataDictionary;
use dfx_base::data_dictionary::MessageValidationError;
use dfx_base::data_dictionary_provider::DataDictionaryProvider;
use dfx_base::field_map::Field;
use dfx_base::field_map::FieldMapError;
use dfx_base::field_map::Tag;
use crate::fields::*;
use dfx_base::fields::ConversionError;
use dfx_base::fields::converters::datetime::DateTimeFormat;
use dfx_base::fix_values::BeginString;
use crate::logging::LogFactory;
use crate::logging::Logger;

use dfx_base::message::Message;
use dfx_base::message::MessageParseError;
use dfx_base::message_factory::MessageFactory;
use dfx_base::message_factory::MessageFactoryError;
use crate::message_store::MessageStoreFactory;
use crate::session::Application;
use crate::session::ApplicationError;
use crate::session::Responder;
use dfx_base::session_id::SessionId;
use crate::session::SessionSchedule;
use crate::session::SessionState;
use dfx_base::tags;

use super::FromAppError;
use super::LogonReject;
use super::Persistence;
use super::SessionSetting;

const _BUF_SIZE: usize = 4096;

lazy_static! {
    static ref SESSION_MAP: CHashMap<SessionId, Sender<Message>> = CHashMap::new();
}

#[allow(non_snake_case)]
pub mod Session {

    use dfx_base::{message::Message, session_id::SessionId};
    use super::{SESSION_MAP, SessionError};

    pub fn send_to_session(session_id: &SessionId, message: Message) -> Result<(), SessionError> {
        match SESSION_MAP.get(session_id) {
            Some(session) => session.send(message).unwrap(),
            None => return Err(SessionError::SessionNotFound),
        }
        Ok(())
    }

}
fn connect(session_id: &SessionId) -> Result<Receiver<Message>, InternalSessionError> {
    if SESSION_MAP.contains_key(session_id) {
        return Err(InternalSessionError::AlreadyConnected);
    }
    let (tx, rx) = channel();
    SESSION_MAP.insert_new(session_id.clone(), tx);
    Ok(rx)
}
fn disconnect_session(session_id: &SessionId) {
    SESSION_MAP.remove(session_id);
}

//TODO: dyn to generic?
pub(crate) struct ISession<App, DDP, Log, MF>
{
    application: App,
    session_id: SessionId,
    _data_dictionary_provider: DDP, // TODO: REMOVE candidate
    schedule: SessionSchedule,
    msg_factory: MF,
    app_does_early_intercept: bool,
    sender_default_appl_ver_id: Option<String>,
    target_default_appl_ver_id: Option<u32>,
    session_data_dictionary: DataDictionary,     //Option?
    application_data_dictionary: DataDictionary, //Option?
    log: Log,
    state: SessionState<Log>,
    persist_messages: bool,
    reset_on_disconnect: bool,
    send_redundant_resend_requests: bool,
    _resend_session_level_rejects: bool,
    validate_length_and_checksum: bool,
    check_comp_id: bool,
    time_stamp_precision: DateTimeFormat,
    enable_last_msg_seq_num_processed: bool,
    max_messages_in_resend_request: u32,
    send_logout_before_timeout_disconnect: bool,
    _ignore_poss_dup_resend_requests: bool,
    requires_orig_sending_time: bool,
    check_latency: bool,
    max_latency: u32,
    responder: Option<Box<dyn Responder>>,
    refresh_on_logon: bool,
    reset_on_logon: bool,
    reset_on_logout: bool,
    outbound: Option<Receiver<Message>>,
}

fn add_data_dictionaries<D: DataDictionaryProvider>(provider: &mut D, settings: &SessionSetting) {
    let options = settings.validation_options();
    if options.use_data_dictionary() {
        if settings.session_id().is_fixt() {
            // https://github.com/connamara/quickfixn/blob/c4e8171e9a702be29078eab3b6dc26b713002de2/QuickFIXn/SessionFactory.cs#L193
            if let Some(appl_ver_id) = settings.default_appl_ver_id() {
                let path = match options.app_data_dictionary() {
                    Some(path) => path,
                    None => settings.session_id().begin_string(), // TODO error?
                };
                let mut dd = DataDictionary::from_file(path).unwrap();
                dd.set_allow_unknown_message_fields(settings.validation_options().allow_unknown_msg_fields());
                dd.set_check_fields_have_values(settings.validation_options().validate_fields_have_values());
                dd.set_check_fields_out_of_order(settings.validation_options().validate_fields_out_of_order());
                dd.set_check_user_defined_fields(settings.validation_options().validate_user_defined_fields());

                provider.add_application_data_dictionary(appl_ver_id, dd);

                let path = match options.transport_data_dictionary() {
                    Some(path) => path,
                    None => settings.session_id().begin_string(), // TODO error?
                };
                let mut dd = DataDictionary::from_file(path).unwrap();
                dd.set_allow_unknown_message_fields(settings.validation_options().allow_unknown_msg_fields());
                dd.set_check_fields_have_values(settings.validation_options().validate_fields_have_values());
                dd.set_check_fields_out_of_order(settings.validation_options().validate_fields_out_of_order());
                dd.set_check_user_defined_fields(settings.validation_options().validate_user_defined_fields());

                provider.add_session_data_dictionary(settings.session_id().begin_string(), dd);
            }
        } else {
            let path = match options.data_dictionary() {
                Some(path) => path,
                None => settings.session_id().begin_string(),
            };

            let mut dd = DataDictionary::from_file(path).unwrap();
            dd.set_allow_unknown_message_fields(settings.validation_options().allow_unknown_msg_fields());
            dd.set_check_fields_have_values(settings.validation_options().validate_fields_have_values());
            dd.set_check_fields_out_of_order(settings.validation_options().validate_fields_out_of_order());
            dd.set_check_user_defined_fields(settings.validation_options().validate_user_defined_fields());
            provider.add_session_data_dictionary(settings.session_id().begin_string(), dd.clone());
            provider.add_application_data_dictionary(ApplVerID::from_begin_string(settings.session_id().begin_string()), dd);
        }
    }
}

impl<App, DDP, Log, MF> ISession<App, DDP, Log, MF>
where App: Application + Clone + 'static,
      DDP: DataDictionaryProvider + Send + Clone + 'static,
      Log: Logger + Clone,
      MF: MessageFactory + Send + Clone + 'static,
{
    pub(crate) fn from_settings(
        session_id: SessionId,
        app: App,
        store_factory: Box<dyn MessageStoreFactory>,
        mut data_dictionary_provider: DDP,
        log: Log,
        msg_factory: MF,
        settings: SessionSetting,
    ) -> Self {
        // REVIEW is this dumb?
        add_data_dictionaries(&mut data_dictionary_provider, &settings);
        let session_data_dictionary = data_dictionary_provider.get_session_data_dictionary(settings.session_id().begin_string()).clone();
        let application_data_dictionary = if settings.session_id().is_fixt() {
            data_dictionary_provider.get_application_data_dictionary(settings.default_appl_ver_id().unwrap()).clone()
        } else {
            session_data_dictionary.clone()
        };

        let msg_store = store_factory.create(&session_id);
        let mut state = SessionState::new(settings.connection().is_initiator(), log.clone(), settings.connection().heart_bt_int().unwrap_or(30), msg_store);
        state.set_logon_timeout(settings.connection().logon_timeout());
        state.set_logout_timeout(settings.connection().logout_timeout());

        if !is_session_time(&settings.schedule().clone()) {
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

        let mut application = app;
        application.on_create(settings.session_id()).unwrap(); //TODO handle err
        log.on_event("Created session");

        ISession {
            application,
            session_id,
            _data_dictionary_provider: data_dictionary_provider,
            schedule: settings.schedule().clone(),
            msg_factory,
            //TODO app is IApplicationExt
            app_does_early_intercept: false,
            sender_default_appl_ver_id: settings.default_appl_ver_id().map(|v| ApplVerID::from_begin_string(v).into()),
            target_default_appl_ver_id: None,
            session_data_dictionary,
            application_data_dictionary,
            log,
            state,
            persist_messages: !matches!(settings.persistence(), Persistence::None),
            reset_on_disconnect: settings.validation_options().reset_on_disconnect(),
            send_redundant_resend_requests: settings.validation_options().send_redundant_resend_requests(),
            _resend_session_level_rejects: settings.validation_options().resend_session_level_rejects(),
            validate_length_and_checksum: settings.validation_options().validate_length_and_checksum(),
            check_comp_id: true,
            time_stamp_precision: settings.validation_options().time_stamp_precision().clone(),
            enable_last_msg_seq_num_processed: settings.validation_options().enable_last_msg_seq_num_processed(),
            max_messages_in_resend_request: settings.validation_options().max_messages_in_resend_request(),
            send_logout_before_timeout_disconnect: settings.validation_options().send_logout_before_disconnect_from_timeout(),
            _ignore_poss_dup_resend_requests: settings.validation_options().ignore_poss_dup_resend_requests(),
            requires_orig_sending_time: settings.validation_options().requires_orig_sending_time(),
            check_latency: settings.validation_options().check_latency(),
            max_latency: settings.validation_options().max_latency(),
            responder: None,
            refresh_on_logon: settings.validation_options().refresh_on_logon(),
            reset_on_logon: settings.validation_options().reset_on_logon(),
            reset_on_logout: settings.validation_options().reset_on_logout(),
            outbound: None,
        }
    }

    pub(crate) fn set_responder(&mut self, responder: Box<dyn Responder>) {
        self.responder = Some(responder);
    }

    pub(crate) fn set_connected(
        &mut self,
        session_id: &SessionId,
    ) -> Result<(), InternalSessionError> {
        let receiver = connect(session_id);
        self.outbound = Some(receiver?);
        Ok(())
    }

    pub(crate) fn set_disconnected(&mut self, session_id: &SessionId) {
        disconnect_session(session_id);
        self.outbound = None;
    }

    fn process_outbound(&mut self) {
        if let Some(receiver) = self.outbound.as_mut() {
            match receiver.recv_timeout(Duration::from_millis(1)) {
                Ok(mut msg) => {
                    self.initialize_header(&mut msg, None);
                    self.send_raw(msg, 0).unwrap()
                }
                Err(_) => return,
            };
        }
    }
    pub(crate) fn next(&mut self) {
        if self.responder.is_none() {
            return;
        }

        self.process_outbound();

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

        if !self.state.is_enabled() {
            if !self.is_logged_on() {
                return;
            }

            if !self.state.sent_logout() {
                self.log.on_event("Initiated logout request");
                self.generate_logout(self.state.logout_reason().cloned(), None);
            }
        }

        if !self.state.received_logon() {
            if self.state.should_send_logon() && self.is_time_to_generate_logon() {
                if self.generate_logon() {
                    self.log.on_event("Initiated logon request");
                } else {
                    self.log.on_event("Error during logon request initiation");
                }
            } else if !self.state.should_send_logon() && self.state.logon_timed_out() {
                self.disconnect("Timed out waiting for logon request");
            } else if self.state.sent_logon() && self.state.logon_timed_out() {
                self.disconnect("Timed out waiting for logon response");
            }
            return;
        }

        if self.state.logout_timed_out() {
            self.disconnect("Timed out waiting for logout response");
        }

        if self.state.within_heartbeat() {
            return;
        }

        if self.state.heartbeat_int() == 0 {
            return;
        }

        if self.state.timed_out() {
            if self.send_logout_before_timeout_disconnect {
                self.generate_logout(None, None);
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
            .map(|ct| self.schedule.is_new_session(ct, Utc::now()))
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
            self.generate_logout(logout_message.map(|v| v.into()), None);
        }
        self.disconnect("Resetting...");
        self.state.reset(logged_reason);
    }

    fn should_send_reset(&self) -> bool {
        //TODO replace hardcoded FIX.4.1
        self.session_id.begin_string() >= "FIX.4.1" && self.reset_on_logon
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
            .create(self.session_id.begin_string(), MsgType::LOGON)
            .unwrap(); // TODO handle unwrap
        logon.set_field_deref(EncryptMethod::new(EncryptMethod::NONE), None);
        logon.set_field_deref(HeartBtInt::new(self.state.heartbeat_int()), None);

        if self.session_id.is_fixt() {
            logon.set_field_deref(
                DefaultApplVerID::new(self.sender_default_appl_ver_id.as_ref().unwrap().to_string()),
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
    fn generate_logon_other(&mut self, other: &Message) -> bool {
        let mut logon = self
            .msg_factory
            .create(self.session_id.begin_string(), MsgType::LOGON)
            .unwrap(); // TODO handle unwrap
        logon.set_tag_value(tags::EncryptMethod, "0");
        if self.session_id.is_fixt() {
            logon.set_tag_value(tags::DefaultApplVerID, self.sender_default_appl_ver_id.as_ref().unwrap());
        }
        logon.set_field_base(other.get_field(tags::HeartBtInt).unwrap().clone(), None);

        if self.enable_last_msg_seq_num_processed {
            if let Some(seq) = other.header().get_field(tags::MsgSeqNum) {
                let value: &FieldValue = seq.value();
                logon.header_mut().set_tag_value(tags::LastMsgSeqNumProcessed, value);
            } else {
                self.log.on_event(format!("Error: No message sequence number: {}", other).as_str());
            }
        }

        self.initialize_header(&mut logon, None);
        let _sent_logon = self.send_raw(logon, 0).unwrap();
        let sent_logon = true; //FIXME check if logon works?
        self.state.set_sent_logon(sent_logon);
        self.state.sent_logon()
    }

    fn generate_logout(&mut self, reason: Option<String>, other: Option<Message>) -> bool {
        let mut logout = self
            .msg_factory
            .create(self.session_id.begin_string(), MsgType::LOGOUT)
            .unwrap(); // TODO handle unwrap
        self.initialize_header(&mut logout, None);
        if matches!(reason.as_ref(), Some(text) if !text.is_empty()) {
            logout.set_tag_value(tags::Text, reason.as_ref().unwrap().as_str());
        }
        if other.as_ref().is_some() && self.enable_last_msg_seq_num_processed {
            if other
                .as_ref()
                .unwrap()
                .header()
                .is_field_set(tags::MsgSeqNum)
            {
                let field = other
                    .as_ref()
                    .unwrap()
                    .header()
                    .get_field(tags::MsgSeqNum).unwrap();
                logout
                    .header_mut()
                    .set_tag_value(tags::LastMsgSeqNumProcessed, field.value());
            } else {
                self.log().on_event(
                    format!("Error: No message sequence number: {:?}", other.as_ref()).as_str(),
                );
            }
        }
        let sent_logout = matches!(self.send_raw(logout, 0), Ok(v) if v);
        self.state.set_sent_logout(sent_logout);
        sent_logout
    }

    fn generate_heartbeat(&mut self) -> bool {
        let mut heartbeat = self
            .msg_factory
            .create(self.session_id.begin_string(), MsgType::HEARTBEAT)
            .unwrap(); // TODO handle unwrap
        self.initialize_header(&mut heartbeat, None);
        matches!(self.send_raw(heartbeat, 0), Ok(v) if v)
    }
    fn generate_heartbeat_other(&mut self, message: &Message) -> bool {
        let mut heartbeat = self
            .msg_factory
            .create(self.session_id.begin_string(), MsgType::HEARTBEAT)
            .unwrap(); // TODO handle unwrap
        self.initialize_header(&mut heartbeat, None);
        heartbeat.set_field_base(message.get_field(tags::TestReqID).unwrap().clone(), None);
        self.log.on_event(format!("generate_heartbeat_other: {}", self.enable_last_msg_seq_num_processed).as_str());
        if self.enable_last_msg_seq_num_processed {
            if let Some(seq) = message.header().get_field(tags::MsgSeqNum) {
                let value: &FieldValue = seq.value();
                heartbeat.header_mut().set_tag_value(tags::LastMsgSeqNumProcessed, value);
            } else {
                self.log.on_event(format!("Error: No message sequence number: {}", message).as_str());
            }
        }
        self.send_raw(heartbeat, 0).unwrap()
    }

    fn generate_test_request(&mut self, reason: &str) -> bool {
        let mut heartbeat = self
            .msg_factory
            .create(self.session_id.begin_string(), MsgType::TEST_REQUEST)
            .unwrap(); // TODO handle unwrap
        self.initialize_header(&mut heartbeat, None);
        heartbeat.set_tag_value(tags::TestReqID, reason);
        matches!(self.send_raw(heartbeat, 0), Ok(v) if v)
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
            self.application.on_logout(&self.session_id).unwrap();
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

    fn send_raw(&mut self, mut message: Message, seq_num: u32) -> Result<bool, FieldMapError> {
        let msg_type = message.header().get_string(tags::MsgType)?;
        self.initialize_header(&mut message, Some(seq_num));
        let message = if Message::is_admin_msg_type(msg_type.as_bytes()) {
            let mut message = self.application.to_admin(message, &self.session_id)?;
            if MsgType::LOGON == msg_type && !self.state.received_reset() {
                let reset = if message.is_field_set(tags::ResetSeqNumFlag) {
                    message.get_string(tags::ResetSeqNumFlag)?.as_str() == "Y"
                } else {
                    false
                };
                if reset {
                    self.state.reset(Some("ResetSeqNumFlag"));
                    message.header_mut().set_field_base(
                        Field::new(
                            tags::MsgSeqNum,
                            format!("{}", self.state.next_sender_msg_seq_num()),
                        ),
                        None,
                    );
                }
                self.state.set_sent_reset(reset);
            }
            Ok(message)
        } else {
            self.application.to_app(&mut message, &self.session_id)
                .map(|_| message)
        };

        match message {
            Ok(mut message) => {
                let message_string = message.to_string_mut();
                if 0 == seq_num {
                    self.persist(&message, &message_string);
                }
                Ok(self.send(message_string))
            },
            Err(e) => match e {
                ApplicationError::DoNotSend => Ok(false),
                ApplicationError::FieldMapError(e) => Err(e),
            },
        }
    }
    fn send(&mut self, message: String) -> bool {
        self.state.set_last_sent_time_dt(Instant::now());
        if let Some(responder) = self.responder.as_mut() {
            self.log.on_outgoing(message.as_str());
            responder.send(message)
        } else {
            false
        }
    }

    fn initialize_header(&mut self, message: &mut Message, seq_num: Option<u32>) {
        let seq_num = seq_num.unwrap_or(0);
        self.state.set_last_sent_time_dt(Instant::now());

        message
            .header_mut()
            .set_tag_value(tags::BeginString, self.session_id.begin_string());
        message
            .header_mut()
            .set_tag_value(tags::SenderCompID, self.session_id.sender_comp_id());
        if !self.session_id.sender_sub_id().is_empty() {
            message
                .header_mut()
                .set_tag_value(tags::SenderSubID, self.session_id.sender_sub_id());
        }
        if !self.session_id.sender_location_id().is_empty() {
            message
                .header_mut()
                .set_tag_value(tags::SenderLocationID, self.session_id.sender_location_id());
        }
        message
            .header_mut()
            .set_tag_value(tags::TargetCompID, self.session_id.target_comp_id());
        if !self.session_id.target_sub_id().is_empty() {
            message
                .header_mut()
                .set_tag_value(tags::TargetSubID, self.session_id.target_sub_id());
        }
        if !self.session_id.target_location_id().is_empty() {
            message
                .header_mut()
                .set_tag_value(tags::TargetLocationID, self.session_id.target_location_id());
        }

        let seq_num = format!(
            "{}",
            if seq_num > 0 {
                seq_num
            } else {
                self.state.next_sender_msg_seq_num()
            }
        );
        message.header_mut().set_tag_value(tags::MsgSeqNum, &seq_num);

        if self.enable_last_msg_seq_num_processed
            && !message.header().is_field_set(tags::LastMsgSeqNumProcessed)
        {
            let last_seq_num = format!("{}", self.state.next_target_msg_seq_num() - 1);
            message
                .header_mut()
                .set_tag_value(tags::LastMsgSeqNumProcessed, &last_seq_num);
        }

        self.insert_sending_time(message)
    }

    fn insert_orig_sending_time(&self, message: &mut Message, sending_time: NaiveDateTime) {
        let fix42_or_above = self.session_id.begin_string() == BeginString::FIXT11 || self.session_id.begin_string() >= BeginString::FIX42;
        let precision = if fix42_or_above {
            self.time_stamp_precision.as_datetime_format()
        } else {
            DateTimeFormat::Seconds.as_datetime_format()
        };
        let send_time = format!("{}", sending_time.format(precision));
        message
            .header_mut()
            .set_tag_value(tags::OrigSendingTime, &send_time);
    }

    fn insert_sending_time(&self, message: &mut Message) {
        let fix42_or_above = self.session_id.begin_string() == BeginString::FIXT11
            || self.session_id.begin_string() >= BeginString::FIX42;
        let precision = if fix42_or_above {
            self.time_stamp_precision.as_datetime_format()
        } else {
            DateTimeFormat::Seconds.as_datetime_format()
        };
        // TODO fix timeformatting
        let send_time = format!("{}", Utc::now().format(precision));
        message
            .header_mut()
            .set_tag_value(tags::SendingTime, &send_time);
    }

    pub(crate) fn session_id(&self) -> &SessionId {
        &self.session_id
    }

    pub(crate) fn log(&mut self) -> &mut Log {
        &mut self.log
    }

    pub(crate) fn next_msg(&mut self, msg: Vec<u8>) {
        self.internal_next_msg(msg);
        self.next_queued();
    }

    // TODO!!! change to fn(&mut self, msg: Vec<u8>) -> Result<(), dfx::Error> IMPORTANT
    // enum dfx::Error {
    //     ...
    //     SessionDisconnect { context, reason }
    // }
    fn internal_next_msg(&mut self, msg: Vec<u8>) {
        self.log.on_incoming(&String::from_utf8_lossy(&msg));

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
                SessionHandleMessageError::InvalidMessageError(e) => {
                    self.log.on_event(&e.message());
                },
                SessionHandleMessageError::MessageParseError { message, parse_error } => {
                    self.log.on_event(format!("MessageParse Error: {parse_error:?}").as_str());
                    let field = parse_error.as_tag();
                    let reason = parse_error.as_session_reject();
                    match Message::new(&message) {
                        Ok(msg) => {
                            self.generate_reject(msg, reason.unwrap(), field).unwrap();
                        }
                        Err(err) => {
                            self.log.on_event(format!("Skipping message due to {err:?}.").as_str());
                        }
                    }
                },
                SessionHandleMessageError::TagException(msg, e) => {
                    if let Some(msg) = e.inner() {
                        self.log.on_event(msg.as_str());
                    }
                    self.generate_reject(msg, e.session_reject_reason().clone(), Some(e.field())).unwrap();
                },
                SessionHandleMessageError::UnsupportedVersion { message, expected, actual } => {
                    let result = message.header().get_string(tags::MsgType);
                    if matches!(result, Ok(v) if MsgType::LOGOUT == v) {
                        self.next_logout(message).unwrap();
                    } else {
                        self.log.on_event(format!("Received version {actual} but expected {expected}").as_str());
                        self.generate_logout(Some(format!("Incorrect BeginString ({actual})")), None);
                        self.state.incr_next_target_msg_seq_num();
                    }
                },
                SessionHandleMessageError::String(s) => self.log.on_event(&s),
                SessionHandleMessageError::FieldMapError(fm) => todo!("{fm:?}"),
                SessionHandleMessageError::ConversionError(conv) => todo!("{conv:?}"),
                SessionHandleMessageError::LogonReject { reason } => {
                    let disconnect_msg = match &reason {
                        Some(r) => format!("Application LogonReject: {r}"),
                        None => "Application LogonReject".into(),
                    };
                    self.generate_logout(reason, None);
                    self.disconnect(&disconnect_msg);
                },
                SessionHandleMessageError::UnknownMessageType { message, msg_type } => {
                    self.log.on_event(format!("Unsupported message type: {}", msg_type).as_str());
                    self.generate_business_message_reject(message, BusinessRejectReason::UNKNOWN_MESSAGE_TYPE()).unwrap();
                }
            }
        }

        self.next()
    }

    fn next_msg_handler(&mut self, msg: Vec<u8>) -> Result<(), SessionHandleMessageError> {
        let msg_type = Message::identify_type(&msg)
            .map_err(|mp| SessionHandleMessageError::MessageParseError { message: msg.clone(), parse_error: mp })?;
        let begin_string = Message::extract_begin_string(&msg)
            .map_err(|mp| SessionHandleMessageError::MessageParseError { message: msg.clone(), parse_error: mp })?;
        let mut message = self.msg_factory.create(begin_string.as_str(), msg_type)?;
        message.from_string(
            &msg,
            self.validate_length_and_checksum,
            Some(&self.session_data_dictionary),
            Some(&self.application_data_dictionary),
            Some(&self.msg_factory),
            false,
        ).map_err(|mp| SessionHandleMessageError::MessageParseError { message: msg.clone(), parse_error: mp })?;
        self.handle_msg(message, &begin_string, msg_type)
    }

    fn handle_msg(&mut self, message: Message, begin_string: &str, msg_type: &str) -> Result<(), SessionHandleMessageError> {
        if self.app_does_early_intercept {
            todo!("Do early intercept")
        }

        let _header = message.header();

        if begin_string != self.session_id.begin_string() {
            return Err(SessionHandleMessageError::UnsupportedVersion {
                message,
                actual: begin_string.into(),
                expected: self.session_id.begin_string().to_string(),
            });
        }

        if MsgType::LOGON == msg_type {
            if self.session_id.is_fixt() {
                self.target_default_appl_ver_id = Some(message.get_int(tags::DefaultApplVerID)?);
            } else {
                self.target_default_appl_ver_id = Some(Message::get_appl_ver_id(begin_string)?);
            }
        }

        let validation_result = if self.session_id.is_fixt() && !Message::is_admin_msg_type(msg_type.as_bytes()) {
            DataDictionary::validate(
                &message,
                Some(&self.session_data_dictionary),
                &self.application_data_dictionary,
                begin_string,
                msg_type,
            )
        } else {
            DataDictionary::validate(
                &message,
                Some(&self.session_data_dictionary),
                &self.session_data_dictionary,
                begin_string,
                msg_type,
            )
        };

        if let Err(e) = validation_result {
            return Err(match e {
                MessageValidationError::UnsupportedVersion { expected, actual } => SessionHandleMessageError::UnsupportedVersion { message, expected, actual },
                MessageValidationError::TagException(tag_exception) => SessionHandleMessageError::TagException(message, tag_exception),
                MessageValidationError::FieldMapError(fm) => SessionHandleMessageError::FieldMapError(fm),
                MessageValidationError::ConversionError(ce) => SessionHandleMessageError::ConversionError(ce),
            });
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
        } else if self.verify(message)?.is_none() {
            Ok(())
        } else {
            self.state.incr_next_target_msg_seq_num();
            Ok(())
        }
    }

    fn next_queued(&mut self) {
        while let Some(msg) = self.state.dequeue(self.state.msg_store().next_target_msg_seq_num()) {
            self.log.on_event(format!("Processing queued message: {}", self.state.msg_store().next_target_msg_seq_num()).as_str());

            match (msg.header().get_string(tags::MsgType), msg.header().get_string(tags::BeginString)) {
                (Ok(msg_type), Ok(begin_string)) => {
                    if msg_type == MsgType::LOGON || msg_type == MsgType::RESEND_REQUEST {
                        self.state.incr_next_target_msg_seq_num();
                    } else {
                        self.handle_msg(msg, &begin_string, &msg_type).unwrap();
                    }
                },
                e => todo!("session::next_queued {e:?}")
            }
        }
    }

    fn persist(&mut self, message: &Message, message_string: &str) {
        if self.persist_messages {
            let msg_seq_num = message.header().get_int(tags::MsgSeqNum).unwrap();
            self.state.set(msg_seq_num, message_string);
        }
        self.state.incr_next_sender_msg_seq_num();
    }

    fn next_logon(&mut self, logon: Message) -> Result<(), SessionHandleMessageError> {
        let received_reset =
            logon.is_field_set(tags::ResetSeqNumFlag) && logon.get_bool(tags::ResetSeqNumFlag);
        self.state.set_received_reset(received_reset);

        if received_reset {
            self.log()
                .on_event("Sequence numbers reset due to ResetSeqNumFlag=Y");
            if !self.state.sent_reset() {
                self.state.reset(Some("Reset requested by counterparty"));
            }
        }

        if !self.state.is_initiator() && self.reset_on_logon() {
            self.state.reset(Some("ResetOnLogon"));
        }
        if self.refresh_on_logon() {
            self.refresh();
        }

        let logon = self.verify_opt(logon, false, true)?;
        if logon.is_none() {
            return Ok(());
        }
        let logon = logon.unwrap();

        if !self.is_good_time(&logon) {
            self.log().on_event("Logon has bad sending time");
            self.disconnect("bad sending time");
            return Ok(());
        }

        self.state.set_received_logon(true);
        self.log().on_event("Received logon");

        if !self.state.is_initiator() {
            let heartbeat_int = logon.get_int(tags::HeartBtInt)?;
            self.state.set_heartbeat_int(heartbeat_int);
            self.generate_logon_other(&logon);
            self.log().on_event("Responding to logon request");
        }

        self.state.set_sent_reset(false);
        self.state.set_received_reset(false);

        let msg_seq_num = logon.header().get_int(tags::MsgSeqNum)?;
        if self.is_target_too_high(msg_seq_num) && !received_reset {
            self.do_target_too_high(logon, msg_seq_num)?;
        } else {
            self.state.incr_next_target_msg_seq_num()
        }

        if self.is_logged_on() {
            self.application.on_logon(&self.session_id)?;
        }
        Ok(())
    }
    fn next_logout(&mut self, logout: Message) -> Result<(), SessionHandleMessageError> {
        let logout = self.verify_opt(logout, false, false)?;
        if logout.is_none() {
            return Ok(());
        }
        let logout = logout.unwrap();

        let reason = if !self.state.sent_logout() {
            let reason = "Received logout request";
            self.log().on_event(reason);
            self.generate_logout(None, Some(logout));
            reason
        } else {
            let reason = "Received logout response";
            self.log().on_event(reason);
            reason
        };

        self.state.incr_next_target_msg_seq_num();

        if self.reset_on_logon() {
            self.state.reset(Some("ResetOnLogout"));
        }
        self.disconnect(reason);
        Ok(())
    }
    fn next_heartbeat(&mut self, message: Message) -> Result<(), SessionHandleMessageError> {
        if self.verify(message)?.is_none() {
            Ok(())
        } else {
            self.state.incr_next_target_msg_seq_num();
            Ok(())
        }
    }
    fn next_test_request(&mut self, message: Message) -> Result<(), SessionHandleMessageError> {
        match self.verify(message)? {
            Some(message) => {
                self.generate_heartbeat_other(&message);
                self.state.incr_next_target_msg_seq_num();
                Ok(())
            }
            None => Ok(()),
        }
    }

    fn next_sequence_reset(&mut self, message: Message) -> Result<(), SessionHandleMessageError> {
        let is_gap_fill =
            message.is_field_set(tags::GapFillFlag) && message.get_bool(tags::GapFillFlag);

        let message = self.verify_opt(message, is_gap_fill, is_gap_fill)?;
        if message.is_none() {
            return Ok(());
        }
        let message = message.unwrap();

        if message.is_field_set(tags::NewSeqNo) {
            let new_seq_no = message.get_int(tags::NewSeqNo)?;
            self.log.on_event(
                format!(
                    "Received SequenceReset FROM: {} TO: {}",
                    self.state.next_target_msg_seq_num(),
                    new_seq_no
                )
                .as_str(),
            );
            if new_seq_no > self.state.next_target_msg_seq_num() {
                self.state.set_next_target_msg_seq_num(new_seq_no);
            } else if new_seq_no < self.state.next_target_msg_seq_num() {
                self.generate_reject(message, SessionRejectReason::VALUE_IS_INCORRECT(), None)?;
            }
        }

        Ok(())
    }

    fn next_resend_request(&mut self, resend_request: Message) -> Result<(), SessionHandleMessageError> {
        let resend_request = self.verify_opt(resend_request, false, false)?;
        if let Some(resend_request) = resend_request {
            if !(self._ignore_poss_dup_resend_requests && resend_request.header().is_field_set(tags::PossDupFlag)) {
                let mut msg_seq_num;
                let beg_seq_no = resend_request.get_int(tags::BeginSeqNo)?;
                let mut end_seq_no = resend_request.get_int(tags::EndSeqNo)?;
                self.log.on_event(format!("Got resend request from {beg_seq_no} to {end_seq_no}").as_str());

                if end_seq_no == 999999 || end_seq_no == 0 {
                    end_seq_no = self.state.next_sender_msg_seq_num() - 1;
                }

                if !self.persist_messages {
                    end_seq_no += 1;
                    let next = self.state.next_sender_msg_seq_num();
                    if end_seq_no > next {
                        end_seq_no = next;
                    }
                    self.generate_sequence_reset(&resend_request, beg_seq_no, end_seq_no)?;
                    msg_seq_num = resend_request.header().get_int(tags::MsgSeqNum)?;
                    if !self.is_target_too_high(msg_seq_num) && !self.is_target_too_low(msg_seq_num) {
                        self.state.incr_next_target_msg_seq_num();
                    }
                    return Ok(());
                }

                let mut current = beg_seq_no;
                let mut begin = 0;
                for msg_str in self.state.get_messages(beg_seq_no, end_seq_no) {
                    let mut msg = Message::default();
                    msg.from_string(
                        msg_str.as_bytes(),
                        true,
                        Some(&self.session_data_dictionary),
                        Some(&self.application_data_dictionary),
                        Some(&self.msg_factory),
                        false
                    ).map_err(|mp| SessionHandleMessageError::MessageParseError { message: msg_str.into_bytes(), parse_error: mp })?;
                    msg_seq_num = msg.header().get_int(tags::MsgSeqNum)?;

                    if current != msg_seq_num && begin == 0 {
                        begin = current;
                    }

                    if msg.is_admin() && !(self._resend_session_level_rejects && msg.header().get_string(tags::MsgType)? == MsgType::REJECT) {
                        if begin == 0 {
                            begin = msg_seq_num;
                        }
                    } else {
                        self.initialize_resend_fields(&mut msg);
                        let approved = self.resend_approved(msg);
                        if let Some(mut msg) = approved {
                            if begin != 0 {
                                self.generate_sequence_reset(&resend_request, begin, msg_seq_num)?;
                            }

                            self.send(msg.to_string_mut());
                            begin = 0;
                        }else{
                            continue;
                        }

                    }
                    current = msg_seq_num + 1;
                }

                let next_seq_num = self.state.next_sender_msg_seq_num();
                end_seq_no += 1;
                if end_seq_no > next_seq_num {
                    end_seq_no = next_seq_num;
                }
                if begin == 0 {
                    begin = current;
                }

                if end_seq_no > begin {
                    self.generate_sequence_reset(&resend_request, begin, end_seq_no)?;
                }
            }
            let msg_seq_num = resend_request.header().get_int(tags::MsgSeqNum)?;
            if !self.is_target_too_high(msg_seq_num) && !self.is_target_too_low(msg_seq_num) {
                self.state.incr_next_target_msg_seq_num();
            }
            Ok(())
        } else {
            Ok(())
        }
    }

    /// This will pass the message into the from_admin / from_app methods from the Application
    fn verify(&mut self, message: Message) -> Result<Option<Message>, SessionHandleMessageError> {
        self.verify_opt(message, true, true)
    }
    /// This will pass the message into the from_admin / from_app methods from the Application
    fn verify_opt(
        &mut self,
        message: Message,
        check_too_high: bool,
        check_too_low: bool,
    ) -> Result<Option<Message>, SessionHandleMessageError> {
        let mut msg_seq_num = 0;
        let msg_type = message.header().get_string(tags::MsgType)?;

        let sender_comp_id = message.header().get_string(tags::SenderCompID)?;
        let target_comp_id = message.header().get_string(tags::TargetCompID)?;

        if !self.is_correct_comp_id(target_comp_id, sender_comp_id) {
            self.generate_reject(message, SessionRejectReason::COMPID_PROBLEM(), None)?;
            self.generate_logout(None, None);
            return Ok(None);
        }

        // if self.session_id.is_empty() {
        //     self.session_id = message.extract_contra_session_id();
        // }

        if check_too_high || check_too_low {
            msg_seq_num = message.header().get_int(tags::MsgSeqNum).unwrap();
        }

        if check_too_high && self.is_target_too_high(msg_seq_num) {
            self.do_target_too_high(message, msg_seq_num)?;
            return Ok(None);
        }
        if check_too_low && self.is_target_too_low(msg_seq_num) {
            self.do_target_too_low(message, msg_seq_num)?;
            return Ok(None);
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
                    self.state.set_resend_range(None);
                } else if let Some(chunk) = range.chunk_end_seq_num {
                    if msg_seq_num >= chunk {
                        self.log.on_event(format!("Chunked ResendRequest for messages FROM: {} TO: {} has been satisfied.", range.begin_seq_num, chunk).as_str());
                        let new_chunk_end_seq_no = cmp::min(
                            range.end_seq_num,
                            chunk + self.max_messages_in_resend_request,
                        );
                        self.generate_resend_request_range(
                            message.header().get_string(tags::BeginString)?,
                            chunk + 1,
                            new_chunk_end_seq_no,
                        )?;
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
            self.generate_reject(
                message,
                SessionRejectReason::SENDING_TIME_ACCURACY_PROBLEM(),
                None,
            )?;
            self.generate_logout(None, None);
            return Ok(None);
        }

        self.state.set_last_received_time_dt(Instant::now());
        self.state.set_test_request_counter(0);

        if Message::is_admin_msg_type(msg_type.as_bytes()) {
            self.application.from_admin(&message, &self.session_id)?
        } else {
            self.application.from_app(&message, &self.session_id)?
        }
        Ok(Some(message))
    }

    fn is_correct_comp_id(&self, sender_comp_id: String, target_comp_id: String) -> bool {
        !self.check_comp_id
            || (self.session_id.sender_comp_id() == sender_comp_id
                && self.session_id.target_comp_id() == target_comp_id)
    }

    fn is_target_too_high(&self, msg_seq_num: u32) -> bool {
        msg_seq_num > self.state.next_target_msg_seq_num()
    }

    fn is_target_too_low(&self, msg_seq_num: u32) -> bool {
        msg_seq_num < self.state.next_target_msg_seq_num()
    }

    fn do_target_too_high(&mut self, msg: Message, msg_seq_num: u32) -> Result<(), SessionHandleMessageError> {
        let begin_string = msg.header().get_string(tags::BeginString)?;

        self.log.on_event(
            format!(
                "MsgSeqNum too high, expecting {} but received {}",
                self.state.next_target_msg_seq_num(),
                msg_seq_num
            )
            .as_str(),
        );
        self.state.queue(msg_seq_num, msg);

        if self.state.resend_requested() {
            if let Some(range) = self.state.resend_range() {
                if !self.send_redundant_resend_requests && msg_seq_num >= range.begin_seq_num {
                    self.log.on_event(
                        format!(
                            "Already sent ResendRequest FROM: {} TO: {}.  Not sending another.",
                            range.begin_seq_num, range.end_seq_num
                        )
                        .as_str(),
                    );
                    return Ok(());
                }
            }
        }
        self.generate_resend_request(begin_string, msg_seq_num)?;
        Ok(())
    }

    fn do_target_too_low(&mut self, message: Message, msg_seq_num: u32) -> Result<(), SessionHandleMessageError> {
        let poss_dup_flag = message.header().is_field_set(tags::PossDupFlag)
            && message.header().get_bool(tags::PossDupFlag);

        if !poss_dup_flag {
            let err = format!(
                "MsgSeqNum too low, expecting {} but received {}",
                self.state.next_target_msg_seq_num(),
                msg_seq_num
            );
            self.generate_logout(Some(err.clone()), None);
            return Err(SessionHandleMessageError::String(err));
        }

        self.do_poss_dup(message)
    }

    fn is_good_time(&self, message: &Message) -> bool {
        if !self.check_latency {
            return true;
        }

        let sending_time = message.header().get_datetime(tags::SendingTime).unwrap();
        let timespan = Utc::now() - sending_time;

        // TODO change to <=
        timespan.num_seconds().abs() < self.max_latency as i64
    }

    fn generate_resend_request_range(
        &mut self,
        begin_string: String,
        start_seq_num: u32,
        end_seq_num: u32,
    ) -> Result<bool, SessionHandleMessageError> {
        let mut resend_request = self
            .msg_factory
            .create(&begin_string, MsgType::RESEND_REQUEST)?;

        resend_request.set_tag_value(tags::BeginSeqNo, format!("{}", start_seq_num).as_str());
        resend_request.set_tag_value(tags::EndSeqNo, format!("{}", end_seq_num).as_str());

        self.initialize_header(&mut resend_request, None);
        if self.send_raw(resend_request, 0)? {
            self.log.on_event(
                format!(
                    "Sent ResendRequest FROM: {} TO: {}",
                    start_seq_num, end_seq_num
                )
                .as_str(),
            );
            Ok(true)
        } else {
            self.log.on_event(
                format!(
                    "Error sending ResendRequest ({},{})",
                    start_seq_num, end_seq_num
                )
                .as_str(),
            );
            Ok(false)
        }
    }

    //TODO change reason to typeof SessionRejectReason
    fn generate_reject(
        &mut self,
        message: Message,
        reason: SessionRejectReason,
        field: Option<Tag>,
    ) -> Result<bool, SessionHandleMessageError> {
        self.log
            .on_event(format!("Reject: {}", reason.reason()).as_str());
        let field = field.unwrap_or(0);

        let begin_string = &self.session_id.begin_string();

        let mut reject = self.msg_factory.create(begin_string, MsgType::REJECT)?;
        reject.reverse_route(message.header());
        self.initialize_header(&mut reject, None);

        let msg_type = if message.header().is_field_set(tags::MsgType) {
            message.header().get_string(tags::MsgType)?
        } else {
            "".into()
        };

        let mut msg_seq_num = 0;
        if message.header().is_field_set(tags::MsgSeqNum) {
            if let Ok(seq_num) = message.header().get_int(tags::MsgSeqNum) {
                msg_seq_num = seq_num;
                reject.set_tag_value(tags::RefSeqNum, format!("{}", msg_seq_num).as_str());
            }
        }

        let begin_string = &self.session_id.begin_string();
        if begin_string >= &BeginString::FIX42 {
            if !msg_type.is_empty() {
                reject.set_tag_value(tags::RefMsgType, &msg_type);
            }
            if (&BeginString::FIX42 == begin_string
                && reason.tag() <= SessionRejectReason::INVALID_MSGTYPE().tag()/*.value*/)
                || begin_string > &BeginString::FIX42
            {
                reject.set_tag_value(tags::SessionRejectReason, format!("{}", reason.tag()) /*.value*/);
            }
        }

        if MsgType::LOGON != msg_type
            && MsgType::SEQUENCE_RESET != msg_type
            && msg_seq_num == self.state.next_target_msg_seq_num()
        {
            self.state.incr_next_target_msg_seq_num();
        }

        if field != 0 || SessionRejectReason::INVALID_TAG_NUMBER().reason() == reason.reason() {
            if SessionRejectReason::INVALID_MSGTYPE() == reason {
                if self.session_id.begin_string() >= BeginString::FIX43 {
                    self.populate_reject_reason(&mut reject, reason.description().as_str() /*.description*/);
                } else {
                    self.populate_session_reject_reason(
                        &mut reject,
                        field,
                        reason.description().as_str(), /*.description*/
                        false,
                    );
                }
            } else {
                self.populate_session_reject_reason(
                    &mut reject,
                    field,
                    reason.description().as_str(), /*.description*/
                    true,
                );
            }
            self.log.on_event(
                format!(
                    "Message {} Rejected: {} (Field={})",
                    msg_seq_num, reason.description(), /*.description*/ field
                )
                .as_str(),
            );
        } else {
            self.populate_reject_reason(&mut reject, reason.description().as_str() /*.description*/);
            self.log.on_event(
                format!(
                    "Message {} Rejected: {}",
                    msg_seq_num, reason.description() /*.description*/
                )
                .as_str(),
            );
        }

        if !self.state.received_logon() {
            Err(SessionHandleMessageError::String(
                "Tried to send a reject while not logged on".into(),
            ))
        } else {
            Ok(self.send_raw(reject, 0)?)
        }
    }

    fn generate_resend_request(
        &mut self,
        begin_string: String,
        msg_seq_num: u32,
    ) -> Result<bool, SessionHandleMessageError> {
        let begin_seq_num = self.state.next_target_msg_seq_num();
        let mut end_range_seq_num = msg_seq_num - 1;
        let end_chunk_seq_num = if self.max_messages_in_resend_request > 0 {
            min(
                end_range_seq_num,
                begin_seq_num + self.max_messages_in_resend_request - 1,
            )
        } else {
            if begin_string.as_str() >= BeginString::FIX42 {
                end_range_seq_num = 0;
            } else {
                end_range_seq_num = 999999;
            }
            end_range_seq_num
        };

        if !self.generate_resend_request_range(begin_string, begin_seq_num, end_chunk_seq_num)? {
            return Ok(false);
        }

        self.state.set_resend_range_begin_end(
            begin_seq_num,
            end_range_seq_num,
            Some(end_chunk_seq_num),
        );
        Ok(true)
    }

    fn do_poss_dup(&mut self, message: Message) -> Result<(), SessionHandleMessageError> {

        let msg_type = message.header().get_string(tags::MsgType)?;
        if msg_type == MsgType::SEQUENCE_RESET && !self.requires_orig_sending_time {
            return Ok(());
        }

        if !message.header().is_field_set(tags::OrigSendingTime) {
            self.generate_reject(
                message,
                SessionRejectReason::REQUIRED_TAG_MISSING(),
                Some(tags::OrigSendingTime),
            )?;
            return Ok(());
        }

        let orig_send_time = message.header().get_datetime(tags::OrigSendingTime)?;
        let sending_time = message.header().get_datetime(tags::SendingTime)?;
        let timespan = orig_send_time - sending_time;

        if timespan.num_seconds() > 0 {
            self.generate_reject(
                message,
                SessionRejectReason::SENDING_TIME_ACCURACY_PROBLEM(),
                None,
            )?;
            self.generate_logout(None, None);
        }

        Ok(())
    }

    fn populate_reject_reason(&self, reject: &mut Message, reason: &str) {
        reject.set_tag_value(tags::Text, reason);
    }

    fn populate_session_reject_reason(
        &self,
        reject: &mut Message,
        field: Tag,
        reason: &str,
        include_field_info: bool,
    ) {
        if self.session_id.begin_string() >= BeginString::FIX42 {
            reject.set_tag_value(tags::RefTagID, format!("{}", field).as_str());
            reject.set_tag_value(tags::Text, reason);
        } else if include_field_info {
            reject.set_tag_value(tags::Text, format!("{} ({})", reason, field).as_str());
        } else {
            reject.set_tag_value(tags::Text, reason);
        }
    }

    pub(crate) fn set_session_id(&mut self, clone: SessionId) {
        self.session_id = clone;
    }

    fn generate_sequence_reset(
        &mut self,
        received_message: &Message,
        begin_seq_no: u32,
        end_seq_no: u32,
    ) -> Result<(), SessionHandleMessageError> {
        let begin_string = self.session_id.begin_string();
        let mut sequence_reset = self.msg_factory.create(begin_string, MsgType::SEQUENCE_RESET)?;
        self.initialize_header(&mut sequence_reset, None);
        let new_seq_no = end_seq_no;
        sequence_reset.header_mut().set_tag_value(tags::PossDupFlag, true);
        let sending_time = sequence_reset.header().get_datetime(tags::SendingTime)?;
        self.insert_orig_sending_time(&mut sequence_reset, sending_time.naive_local());

        sequence_reset.header_mut().set_tag_value(tags::MsgSeqNum, begin_seq_no as i64);
        sequence_reset.set_tag_value(tags::NewSeqNo, new_seq_no as i64);
        sequence_reset.set_tag_value(tags::GapFillFlag, true);
        if /* resend_request.is_some() && */ self.enable_last_msg_seq_num_processed {
            let seq_num: Option<Result<i64, _>> = received_message.get_field(tags::MsgSeqNum).map(|v| v.as_value());
            if let Some(result) = seq_num {
                sequence_reset.header_mut().set_tag_value(tags::LastMsgSeqNumProcessed, result?);
            } else {
                self.log().on_event(format!("Error: Received message without MsgSeqNum: {}", received_message).as_str());
            }
        }

        self.send_raw(sequence_reset, begin_seq_no)?;
        self.log().on_event(format!("Sent SequenceReset TO: {}", begin_seq_no).as_str());
        Ok(())
    }

    fn initialize_resend_fields(&self, msg: &mut Message) {
        let sending_time = msg.header().get_datetime(tags::SendingTime).unwrap();
        self.insert_orig_sending_time(msg, sending_time.naive_utc());
        msg.header_mut().set_tag_value(tags::PossDupFlag, true);
        self.insert_sending_time(msg);
    }

    fn resend_approved(&mut self, mut msg: Message) -> Option<Message> {
        match self.application.to_app(&mut msg, &self.session_id) {
            Err(ApplicationError::DoNotSend) => None,
            Ok(()) => Some(msg),
            _ => Some(msg)
        }
    }

    fn generate_business_message_reject(&mut self, message: Message, business_reject_reason: BusinessRejectReason) -> Result<(), SessionHandleMessageError> {
        let msg_type = message.header().get_string(tags::MsgType)?;
        let msg_seq_num = message.header().get_int(tags::MsgSeqNum)?;
        let reason = business_reject_reason.reason();
        let mut reject = if self.session_id.begin_string() >= BeginString::FIX42 {
            let mut reject = self.msg_factory.create(self.session_id.begin_string(), MsgType::BUSINESS_MESSAGE_REJECT)?;
            reject.set_tag_value(tags::RefMsgType, msg_type);
            reject.set_tag_value(tags::BusinessRejectReason, format!("{}", business_reject_reason.index()));
            reject
        } else {
            
            // TODO magic?
        //     reject = msgFactory_.Create(this.SessionID.BeginString, MsgType.REJECT);
        //     char[] reasonArray = reason.ToLower().ToCharArray();
        //     reasonArray[0] = char.ToUpper(reasonArray[0]);
        //     reason = new string(reasonArray);
            self.msg_factory.create(self.session_id.begin_string(), MsgType::REJECT)?
        };

        self.initialize_header(&mut reject, None);
        reject.set_tag_value(tags::RefSeqNum, format!("{}", msg_seq_num));
        self.state.incr_next_target_msg_seq_num();

        reject.set_tag_value(tags::Text, reason);
        self.log.on_event("Reject sent for Message: {msg_seq_num} Reason:{reason}");
        self.send_raw(reject, 0)?;
        Ok(())
    }
}

fn is_session_time(session_schedule: &SessionSchedule) -> bool {
    session_schedule.is_session_time(&Utc::now())
}

#[derive(Debug, Clone)]
pub enum SessionError {
    NotConnected(SessionId),
    NotLoggedOn(SessionId),
    SessionNotFound,
}

#[derive(Debug, Clone)]
pub(crate) enum InternalSessionError {
    // NotConnected(SessionId),
    // NotLoggedOn(SessionId),
    // SessionNotFound,
    AlreadyConnected,
}

#[derive(Debug, Clone)]
pub(crate) enum SessionHandleMessageError {
    UnsupportedVersion { message: Message, expected: String, actual: String },
    UnknownMessageType { message: Message, msg_type: String },
    // MessageFactory::create
    InvalidMessageError(MessageFactoryError),
    // Message::from_string
    MessageParseError { message: Vec<u8>, parse_error: MessageParseError },
    // DataDictionaryError
    TagException(Message, TagException),
    //TODO?
    String(String),
    LogonReject { reason: Option<String> },
    // Handle same way?
    FieldMapError(FieldMapError),
    ConversionError(ConversionError),
}

impl From<MessageFactoryError> for SessionHandleMessageError {
    fn from(e: MessageFactoryError) -> Self {
        SessionHandleMessageError::InvalidMessageError(e)
    }
}
// impl From<MessageParseError> for SessionHandleMessageError {
//     fn from(e: MessageParseError) -> Self {
//         SessionHandleMessageError::MessageParseError(e)
//     }
// }
// impl From<MessageValidationError> for SessionHandleMessageError {
//     fn from(e: MessageValidationError) -> Self {
//         SessionHandleMessageError::MessageValidationError(e)
//     }
// }
impl From<String> for SessionHandleMessageError {
    fn from(e: String) -> Self {
        SessionHandleMessageError::String(e)
    }
}
impl From<LogonReject> for SessionHandleMessageError {
    fn from(e: LogonReject) -> Self {
        SessionHandleMessageError::LogonReject { reason: e.reason}
    }
}
impl From<FieldMapError> for SessionHandleMessageError {
    fn from(e: FieldMapError) -> Self {
        SessionHandleMessageError::FieldMapError(e)
    }
}
impl From<dfx_base::fields::ConversionError> for SessionHandleMessageError {
    fn from(e: dfx_base::fields::ConversionError) -> Self {
        SessionHandleMessageError::ConversionError(e)
    }
}
impl From<FromAppError> for SessionHandleMessageError {
    fn from(from_app_error: FromAppError) -> Self {
        match from_app_error {
            FromAppError::UnknownMessageType { message, msg_type } => SessionHandleMessageError::UnknownMessageType { message, msg_type },
            FromAppError::FieldMapError(fm) => SessionHandleMessageError::FieldMapError(fm),
        }
    }
}
