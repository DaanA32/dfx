use std::{fs::File, io::{BufReader, BufRead}, collections::BTreeMap, net::SocketAddr};

use crate::connection::SocketSettings;

use super::{SessionId, Session, Application};

enum Setting {
    IsDynamic,
    BeginString,
    SenderCompID,
    SenderSubID,
    SenderLocationID,
    TargetCompID,
    TargetSubID,
    TargetLocationID,
    SessionQualifier,
    DefaultApplVerID,
    ConnectionType,
    UseDataDictionary,
    NonStopSession,
    UseLocalTime,
    TimeZone,
    StartDay,
    EndDay,
    StartTime,
    EndTime,
    HeartBtInt,
    SocketAcceptHost,
    SocketAcceptPort,
    SocketConnectHost,
    SocketConnectPort,
    ReconnectInterval,
    FileLogPath,
    DebugFileLogPath,
    FileStorePath,
    RefreshOnLogon,
    ResetOnLogon,
    ResetOnLogout,
    ResetOnDisconnect,
    ValidateFieldsOutOfOrder,
    ValidateFieldsHaveValues,
    ValidateUserDefinedFields,
    ValidateLengthAndChecksum,
    AllowUnknownMsgFields,
    DataDictionary,
    TransportDataDictionary,
    AppDataDictionary,
    PersistMessages,
    LogonTimeout,
    LogoutTimeout,
    SendRedundantResendRequests,
    ResendSessionLevelRejects,
    MillisecondsInTimeStamp,
    TimeStampPrecision,
    EnableLastMsgSeqNumProcessed,
    MaxMessagesInResendRequest,
    SendLogoutBeforeDisconnectFromTimeout,
    SocketNodelay,
    SocketSendBufferSize,
    SocketReceiveBufferSize,
    SocketSendTimeout,
    SocketReceiveTimeout,
    IgnorePossDupResendRequests,
    RequiresOrigSendingTime,
    CheckLatency,
    MaxLatency,
    SSLEnable,
    SSLServerName,
    SSLProtocols,
    SSLValidateCertificates,
    SSLCheckCertificateRevocation,
    SSLCertificate,
    SSLCertificatePassword,
    SSLRequireClientCertificate,
    SSLCACertificate,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(crate) struct SessionSetting {
    is_dynamic: Option<String>,
    begin_string: Option<String>,
    sender_comp_id: Option<String>,
    sender_sub_id: Option<String>,
    sender_location_id: Option<String>,
    target_comp_id: Option<String>,
    target_sub_id: Option<String>,
    target_location_id: Option<String>,
    session_qualifier: Option<String>,
    default_appl_ver_id: Option<String>,
    connection_type: Option<String>,
    non_stop_session: Option<String>,
    use_local_time: Option<String>,
    time_zone: Option<String>,
    start_day: Option<String>,
    end_day: Option<String>,
    start_time: Option<String>,
    end_time: Option<String>,
    milliseconds_in_time_stamp: Option<String>,
    refresh_on_logon: Option<String>,
    reset_on_logon: Option<String>,
    reset_on_logout: Option<String>,
    reset_on_disconnect: Option<String>,
    send_redundant_resend_requests: Option<String>,
    resend_session_level_rejects: Option<String>,
    time_stamp_precision: Option<String>,
    enable_last_msg_seq_num_processed: Option<String>,
    max_messages_in_resend_request: Option<String>,
    send_logout_before_disconnect_from_timeout: Option<String>,
    ignore_poss_dup_resend_requests: Option<String>,
    requires_orig_sending_time: Option<String>,

    // validation options
    use_data_dictionary: Option<String>,
    data_dictionary: Option<String>,
    transport_data_dictionary: Option<String>,
    app_data_dictionary: Option<String>,
    validate_fields_out_of_order: Option<String>,
    validate_fields_have_values: Option<String>,
    validate_user_defined_fields: Option<String>,
    validate_length_and_checksum: Option<String>,
    allow_unknown_msg_fields: Option<String>,
    check_latency: Option<String>,
    max_latency: Option<String>,

    // initiator options
    reconnect_interval: Option<String>,
    heart_bt_int: Option<String>,
    logon_timeout: Option<String>,
    logout_timeout: Option<String>,
    socket_connect_host: Option<String>,
    socket_connect_port: Option<String>,
    //TODO
    socket_connect_hosts: Option<String>, // initiator<n> failover
    socket_connect_ports: Option<String>, // initiator<n> failover

    // acceptor options
    socket_accept_host: Option<String>,
    socket_accept_port: Option<String>,

    // storage
    persist_messages: Option<String>,
    // store path
    file_store_path: Option<String>,

    // logging
    file_log_path: Option<String>,
    debug_file_log_path: Option<String>,

    // Socket options
    socket_nodelay: Option<String>,
    socket_send_buffer_size: Option<String>,
    socket_receive_buffer_size: Option<String>,
    socket_send_timeout: Option<String>,
    socket_receive_timeout: Option<String>,

    // SSL options
    ssl_enable: Option<String>,
    ssl_server_name: Option<String>,
    ssl_protocols: Option<String>,
    ssl_validate_certificates: Option<String>,
    ssl_check_certificate_revocation: Option<String>,
    ssl_certificate: Option<String>,
    ssl_certificate_password: Option<String>,
    ssl_require_client_certificate: Option<String>,
    ssl_ca_certificate: Option<String>,
}

impl SessionSetting {

    fn set(&mut self, option: Setting, value: &str) {
        match option {
            Setting::IsDynamic => self.is_dynamic = Some(value.into()),
            Setting::BeginString => self.begin_string = Some(value.into()),
            Setting::SenderCompID => self.sender_comp_id = Some(value.into()),
            Setting::SenderSubID => self.sender_sub_id = Some(value.into()),
            Setting::SenderLocationID => self.sender_location_id = Some(value.into()),
            Setting::TargetCompID => self.target_comp_id = Some(value.into()),
            Setting::TargetSubID => self.target_sub_id = Some(value.into()),
            Setting::TargetLocationID => self.target_location_id = Some(value.into()),
            Setting::SessionQualifier => self.session_qualifier = Some(value.into()),
            Setting::DefaultApplVerID => self.default_appl_ver_id = Some(value.into()),
            Setting::ConnectionType => self.connection_type = Some(value.into()),
            Setting::UseDataDictionary => self.use_data_dictionary = Some(value.into()),
            Setting::NonStopSession => self.non_stop_session = Some(value.into()),
            Setting::UseLocalTime => self.use_local_time = Some(value.into()),
            Setting::TimeZone => self.time_zone = Some(value.into()),
            Setting::StartDay => self.start_day = Some(value.into()),
            Setting::EndDay => self.end_day = Some(value.into()),
            Setting::StartTime => self.start_time = Some(value.into()),
            Setting::EndTime => self.end_time = Some(value.into()),
            Setting::HeartBtInt => self.heart_bt_int = Some(value.into()),
            Setting::SocketAcceptHost => self.socket_accept_host = Some(value.into()),
            Setting::SocketAcceptPort => self.socket_accept_port = Some(value.into()),
            Setting::SocketConnectHost => self.socket_connect_host = Some(value.into()),
            Setting::SocketConnectPort => self.socket_connect_port = Some(value.into()),
            Setting::ReconnectInterval => self.reconnect_interval = Some(value.into()),
            Setting::FileLogPath => self.file_log_path = Some(value.into()),
            Setting::DebugFileLogPath => self.debug_file_log_path = Some(value.into()),
            Setting::FileStorePath => self.file_store_path = Some(value.into()),
            Setting::RefreshOnLogon => self.refresh_on_logon = Some(value.into()),
            Setting::ResetOnLogon => self.reset_on_logon = Some(value.into()),
            Setting::ResetOnLogout => self.reset_on_logout = Some(value.into()),
            Setting::ResetOnDisconnect => self.reset_on_disconnect = Some(value.into()),
            Setting::ValidateFieldsOutOfOrder => self.validate_fields_out_of_order = Some(value.into()),
            Setting::ValidateFieldsHaveValues => self.validate_fields_have_values = Some(value.into()),
            Setting::ValidateUserDefinedFields => self.validate_user_defined_fields = Some(value.into()),
            Setting::ValidateLengthAndChecksum => self.validate_length_and_checksum = Some(value.into()),
            Setting::AllowUnknownMsgFields => self.allow_unknown_msg_fields = Some(value.into()),
            Setting::DataDictionary => self.data_dictionary = Some(value.into()),
            Setting::TransportDataDictionary => self.transport_data_dictionary = Some(value.into()),
            Setting::AppDataDictionary => self.app_data_dictionary = Some(value.into()),
            Setting::PersistMessages => self.persist_messages = Some(value.into()),
            Setting::LogonTimeout => self.logon_timeout = Some(value.into()),
            Setting::LogoutTimeout => self.logout_timeout = Some(value.into()),
            Setting::SendRedundantResendRequests => self.send_redundant_resend_requests = Some(value.into()),
            Setting::ResendSessionLevelRejects => self.resend_session_level_rejects = Some(value.into()),
            Setting::MillisecondsInTimeStamp => self.milliseconds_in_time_stamp = Some(value.into()),
            Setting::TimeStampPrecision => self.time_stamp_precision = Some(value.into()),
            Setting::EnableLastMsgSeqNumProcessed => self.enable_last_msg_seq_num_processed = Some(value.into()),
            Setting::MaxMessagesInResendRequest => self.max_messages_in_resend_request = Some(value.into()),
            Setting::SendLogoutBeforeDisconnectFromTimeout => self.send_logout_before_disconnect_from_timeout = Some(value.into()),
            Setting::SocketNodelay => self.socket_nodelay = Some(value.into()),
            Setting::SocketSendBufferSize => self.socket_send_buffer_size = Some(value.into()),
            Setting::SocketReceiveBufferSize => self.socket_receive_buffer_size = Some(value.into()),
            Setting::SocketSendTimeout => self.socket_send_timeout = Some(value.into()),
            Setting::SocketReceiveTimeout => self.socket_receive_timeout = Some(value.into()),
            Setting::IgnorePossDupResendRequests => self.ignore_poss_dup_resend_requests = Some(value.into()),
            Setting::RequiresOrigSendingTime => self.requires_orig_sending_time = Some(value.into()),
            Setting::CheckLatency => self.check_latency = Some(value.into()),
            Setting::MaxLatency => self.max_latency = Some(value.into()),
            Setting::SSLEnable => self.ssl_enable = Some(value.into()),
            Setting::SSLServerName => self.ssl_server_name = Some(value.into()),
            Setting::SSLProtocols => self.ssl_protocols = Some(value.into()),
            Setting::SSLValidateCertificates => self.ssl_validate_certificates = Some(value.into()),
            Setting::SSLCheckCertificateRevocation => self.ssl_check_certificate_revocation = Some(value.into()),
            Setting::SSLCertificate => self.ssl_certificate = Some(value.into()),
            Setting::SSLCertificatePassword => self.ssl_certificate_password = Some(value.into()),
            Setting::SSLRequireClientCertificate => self.ssl_require_client_certificate = Some(value.into()),
            Setting::SSLCACertificate => self.ssl_ca_certificate = Some(value.into()),
        }
    }

    fn set_from_line(&mut self, line: &str) -> Result<(), SessionSettingsError> {
        let setting: Vec<&str> = line.split('=').collect();
        Ok(if setting.len() > 2 {
            todo!()
        }else if setting.len() == 2 {
            let option: Setting = setting[0].try_into()?;
            let value = setting[1];
            self.set(option, value);
        })
    }

    fn merge(mut self, other: &Self) -> Self {
        self.begin_string = self.begin_string.or(other.begin_string.clone());
        self.sender_comp_id = self.sender_comp_id.or(other.sender_comp_id.clone());
        self.sender_sub_id = self.sender_sub_id.or(other.sender_sub_id.clone());
        self.sender_location_id = self.sender_location_id.or(other.sender_location_id.clone());
        self.target_comp_id = self.target_comp_id.or(other.target_comp_id.clone());
        self.target_sub_id = self.target_sub_id.or(other.target_sub_id.clone());
        self.target_location_id = self.target_location_id.or(other.target_location_id.clone());
        self.session_qualifier = self.session_qualifier.or(other.session_qualifier.clone());
        self.default_appl_ver_id = self.default_appl_ver_id.or(other.default_appl_ver_id.clone());
        self.connection_type = self.connection_type.or(other.connection_type.clone());
        self.non_stop_session = self.non_stop_session.or(other.non_stop_session.clone());
        self.use_local_time = self.use_local_time.or(other.use_local_time.clone());
        self.time_zone = self.time_zone.or(other.time_zone.clone());
        self.start_day = self.start_day.or(other.start_day.clone());
        self.end_day = self.end_day.or(other.end_day.clone());
        self.start_time = self.start_time.or(other.start_time.clone());
        self.end_time = self.end_time.or(other.end_time.clone());
        self.milliseconds_in_time_stamp = self.milliseconds_in_time_stamp.or(other.milliseconds_in_time_stamp.clone());
        self.refresh_on_logon = self.refresh_on_logon.or(other.refresh_on_logon.clone());
        self.reset_on_logon = self.reset_on_logon.or(other.reset_on_logon.clone());
        self.reset_on_logout = self.reset_on_logout.or(other.reset_on_logout.clone());
        self.reset_on_disconnect = self.reset_on_disconnect.or(other.reset_on_disconnect.clone());
        self.send_redundant_resend_requests = self.send_redundant_resend_requests.or(other.send_redundant_resend_requests.clone());
        self.resend_session_level_rejects = self.resend_session_level_rejects.or(other.resend_session_level_rejects.clone());
        self.time_stamp_precision = self.time_stamp_precision.or(other.time_stamp_precision.clone());
        self.enable_last_msg_seq_num_processed = self.enable_last_msg_seq_num_processed.or(other.enable_last_msg_seq_num_processed.clone());
        self.max_messages_in_resend_request = self.max_messages_in_resend_request.or(other.max_messages_in_resend_request.clone());
        self.send_logout_before_disconnect_from_timeout = self.send_logout_before_disconnect_from_timeout.or(other.send_logout_before_disconnect_from_timeout.clone());
        self.ignore_poss_dup_resend_requests = self.ignore_poss_dup_resend_requests.or(other.ignore_poss_dup_resend_requests.clone());
        self.requires_orig_sending_time = self.requires_orig_sending_time.or(other.requires_orig_sending_time.clone());

        // validation options
        self.use_data_dictionary = self.use_data_dictionary.or(other.use_data_dictionary.clone());
        self.data_dictionary = self.data_dictionary.or(other.data_dictionary.clone());
        self.transport_data_dictionary = self.transport_data_dictionary.or(other.transport_data_dictionary.clone());
        self.app_data_dictionary = self.app_data_dictionary.or(other.app_data_dictionary.clone());
        self.validate_fields_out_of_order = self.validate_fields_out_of_order.or(other.validate_fields_out_of_order.clone());
        self.validate_fields_have_values = self.validate_fields_have_values.or(other.validate_fields_have_values.clone());
        self.validate_user_defined_fields = self.validate_user_defined_fields.or(other.validate_user_defined_fields.clone());
        self.validate_length_and_checksum = self.validate_length_and_checksum.or(other.validate_length_and_checksum.clone());
        self.allow_unknown_msg_fields = self.allow_unknown_msg_fields.or(other.allow_unknown_msg_fields.clone());
        self.check_latency = self.check_latency.or(other.check_latency.clone());
        self.max_latency = self.max_latency.or(other.max_latency.clone());

        // initiator options
        self.reconnect_interval = self.reconnect_interval.or(other.reconnect_interval.clone());
        self.heart_bt_int = self.heart_bt_int.or(other.heart_bt_int.clone());
        self.logon_timeout = self.logon_timeout.or(other.logon_timeout.clone());
        self.logout_timeout = self.logout_timeout.or(other.logout_timeout.clone());
        self.socket_connect_host = self.socket_connect_host.or(other.socket_connect_host.clone());
        self.socket_connect_port = self.socket_connect_port.or(other.socket_connect_port.clone());
        self.socket_connect_hosts = self.socket_connect_hosts.or(other.socket_connect_hosts.clone());
        self.socket_connect_ports = self.socket_connect_ports.or(other.socket_connect_ports.clone());

        // acceptor options
        self.socket_accept_host = self.socket_accept_host.or(other.socket_accept_host.clone());
        self.socket_accept_port = self.socket_accept_port.or(other.socket_accept_port.clone());

        // storage
        self.persist_messages = self.persist_messages.or(other.persist_messages.clone());
        // store path
        self.file_store_path = self.file_store_path.or(other.file_store_path.clone());

        // logging
        self.file_log_path = self.file_log_path.or(other.file_log_path.clone());
        self.debug_file_log_path = self.debug_file_log_path.or(other.debug_file_log_path.clone());

        // Socket options
        self.socket_nodelay = self.socket_nodelay.or(other.socket_nodelay.clone());
        self.socket_send_buffer_size = self.socket_send_buffer_size.or(other.socket_send_buffer_size.clone());
        self.socket_receive_buffer_size = self.socket_receive_buffer_size.or(other.socket_receive_buffer_size.clone());
        self.socket_send_timeout = self.socket_send_timeout.or(other.socket_send_timeout.clone());
        self.socket_receive_timeout = self.socket_receive_timeout.or(other.socket_receive_timeout.clone());

        // SSL options
        self.ssl_enable = self.ssl_enable.or(other.ssl_enable.clone());
        self.ssl_server_name = self.ssl_server_name.or(other.ssl_server_name.clone());
        self.ssl_protocols = self.ssl_protocols.or(other.ssl_protocols.clone());
        self.ssl_validate_certificates = self.ssl_validate_certificates.or(other.ssl_validate_certificates.clone());
        self.ssl_check_certificate_revocation = self.ssl_check_certificate_revocation.or(other.ssl_check_certificate_revocation.clone());
        self.ssl_certificate = self.ssl_certificate.or(other.ssl_certificate.clone());
        self.ssl_certificate_password = self.ssl_certificate_password.or(other.ssl_certificate_password.clone());
        self.ssl_require_client_certificate = self.ssl_require_client_certificate.or(other.ssl_require_client_certificate.clone());
        self.ssl_ca_certificate = self.ssl_ca_certificate.or(other.ssl_ca_certificate.clone());
        self
    }

    fn score(&self, session_id: &SessionId) -> u16 {
        let mut score = 0;
        if let Some(sender_comp_id) = self.sender_comp_id.as_ref() {
            score += match sender_comp_id.as_str() {
                "*" => 6,
                value if value == session_id.sender_comp_id => 7,
                _ => 0,
            };
        }
        if let Some(sender_sub_id) = self.sender_sub_id.as_ref() {
            score += match sender_sub_id.as_str() {
                value if value == session_id.sender_sub_id => 1,
                _ => 0,
            };
        }
        if let Some(sender_loc_id) = self.sender_location_id.as_ref() {
            score += match sender_loc_id.as_str() {
                value if value == &session_id.sender_location_id => 1,
                _ => 0,
            };
        }
        if let Some(target_comp_id) = self.target_comp_id.as_ref() {
            score += match target_comp_id.as_str() {
                "*" => 6,
                value if value == session_id.target_comp_id => 7,
                _ => 0,
            };
        }
        if let Some(target_sub_id) = self.target_sub_id.as_ref() {
            score += match target_sub_id.as_str() {
                value if value == session_id.target_sub_id => 1,
                _ => 0,
            };
        }
        if let Some(target_loc_id) = self.target_location_id.as_ref() {
            score += match target_loc_id.as_str() {
                value if value == &session_id.target_location_id => 1,
                _ => 0,
            };
        }
        if score < 12 {
            0
        } else {
            score
        }
    }

    pub(crate) fn is_dynamic(&self) -> bool {
        matches!(self.is_dynamic.as_ref(), Some(id) if id == "Y")
            && (
                matches!(self.sender_comp_id.as_ref(), Some(id) if id == "*")
                ||
                matches!(self.target_comp_id.as_ref(), Some(id) if id == "*")
            )
    }

    pub(crate) fn socket_settings(&self) -> SocketSettings {
        let is_initiator = self.connection_type.as_ref().map(|t| t != "acceptor").unwrap_or(false);
        if is_initiator {
            println!("{:?}:{:?}", self.socket_connect_host.as_ref(), self.socket_connect_port.as_ref());
            let host = self.socket_connect_host.as_ref().expect("Some host");
            let port = self.socket_connect_port.as_ref().expect("Some port").parse().unwrap();
            SocketSettings::new(host.into(), port)
        } else {
            let host = self.socket_accept_host.as_ref().expect("Some host");
            let port = self.socket_accept_port.as_ref().expect("Some port").parse().unwrap();
            SocketSettings::new(host.into(), port)
        }
    }

    pub(crate) fn create(&self, app: Box<dyn Application>) -> Session {
        let is_initiator = self.connection_type.as_ref().unwrap() == "initiator";
        let session_id = self.session_id();
        let sender_default_appl_ver_id = self.default_appl_ver_id.as_ref().map(|v| v.as_str()).unwrap_or("");
        Session::builder(is_initiator, app, session_id, sender_default_appl_ver_id)
            .with_heartbeat_int(self.heart_bt_int.as_ref().map(|hb| hb.parse().ok()).flatten().unwrap_or(30))
            //TODO other settings
            .build()
    }

    fn session_id(&self) -> SessionId {
        SessionId::new(
            self.begin_string.as_ref().map(|v| v.as_str()).unwrap_or(""),
            self.sender_comp_id.as_ref().map(|v| v.as_str()).unwrap_or(""),
            self.sender_sub_id.as_ref().map(|v| v.as_str()).unwrap_or(""),
            self.sender_location_id.as_ref().map(|v| v.as_str()).unwrap_or(""),
            self.target_comp_id.as_ref().map(|v| v.as_str()).unwrap_or(""),
            self.target_sub_id.as_ref().map(|v| v.as_str()).unwrap_or(""),
            self.target_location_id.as_ref().map(|v| v.as_str()).unwrap_or(""),
        )
    }
}

#[derive(Debug, Default, Clone)]
pub struct SessionSettings {
    default: SessionSetting,
    sessions: Vec<SessionSetting>,
}

#[derive(Debug)]
pub enum SessionSettingsError {
    NoSuchSetting(String),
    IoError(std::io::Error),
    NoDefaultSection,
    DefaultSectionAlreadyDefined,
}

impl TryFrom<&str> for Setting {
    type Error = SessionSettingsError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "IsDynamic" => Ok(Self::IsDynamic),
            "BeginString" => Ok(Self::BeginString),
            "SenderCompID" => Ok(Self::SenderCompID),
            "SenderSubID" => Ok(Self::SenderSubID),
            "SenderLocationID" => Ok(Self::SenderLocationID),
            "TargetCompID" => Ok(Self::TargetCompID),
            "TargetSubID" => Ok(Self::TargetSubID),
            "TargetLocationID" => Ok(Self::TargetLocationID),
            "SessionQualifier" => Ok(Self::SessionQualifier),
            "DefaultApplVerID" => Ok(Self::DefaultApplVerID),
            "ConnectionType" => Ok(Self::ConnectionType),
            "UseDataDictionary" => Ok(Self::UseDataDictionary),
            "NonStopSession" => Ok(Self::NonStopSession),
            "UseLocalTime" => Ok(Self::UseLocalTime),
            "TimeZone" => Ok(Self::TimeZone),
            "StartDay" => Ok(Self::StartDay),
            "EndDay" => Ok(Self::EndDay),
            "StartTime" => Ok(Self::StartTime),
            "EndTime" => Ok(Self::EndTime),
            "HeartBtInt" => Ok(Self::HeartBtInt),
            "SocketAcceptHost" => Ok(Self::SocketAcceptHost),
            "SocketAcceptPort" => Ok(Self::SocketAcceptPort),
            "SocketConnectHost" => Ok(Self::SocketConnectHost),
            "SocketConnectPort" => Ok(Self::SocketConnectPort),
            "ReconnectInterval" => Ok(Self::ReconnectInterval),
            "FileLogPath" => Ok(Self::FileLogPath),
            "DebugFileLogPath" => Ok(Self::DebugFileLogPath),
            "FileStorePath" => Ok(Self::FileStorePath),
            "RefreshOnLogon" => Ok(Self::RefreshOnLogon),
            "ResetOnLogon" => Ok(Self::ResetOnLogon),
            "ResetOnLogout" => Ok(Self::ResetOnLogout),
            "ResetOnDisconnect" => Ok(Self::ResetOnDisconnect),
            "ValidateFieldsOutOfOrder" => Ok(Self::ValidateFieldsOutOfOrder),
            "ValidateFieldsHaveValues" => Ok(Self::ValidateFieldsHaveValues),
            "ValidateUserDefinedFields" => Ok(Self::ValidateUserDefinedFields),
            "ValidateLengthAndChecksum" => Ok(Self::ValidateLengthAndChecksum),
            "AllowUnknownMsgFields" => Ok(Self::AllowUnknownMsgFields),
            "DataDictionary" => Ok(Self::DataDictionary),
            "TransportDataDictionary" => Ok(Self::TransportDataDictionary),
            "AppDataDictionary" => Ok(Self::AppDataDictionary),
            "PersistMessages" => Ok(Self::PersistMessages),
            "LogonTimeout" => Ok(Self::LogonTimeout),
            "LogoutTimeout" => Ok(Self::LogoutTimeout),
            "SendRedundantResendRequests" => Ok(Self::SendRedundantResendRequests),
            "ResendSessionLevelRejects" => Ok(Self::ResendSessionLevelRejects),
            "MillisecondsInTimeStamp" => Ok(Self::MillisecondsInTimeStamp),
            "TimeStampPrecision" => Ok(Self::TimeStampPrecision),
            "EnableLastMsgSeqNumProcessed" => Ok(Self::EnableLastMsgSeqNumProcessed),
            "MaxMessagesInResendRequest" => Ok(Self::MaxMessagesInResendRequest),
            "SendLogoutBeforeDisconnectFromTimeout" => Ok(Self::SendLogoutBeforeDisconnectFromTimeout),
            "SocketNodelay" => Ok(Self::SocketNodelay),
            "SocketSendBufferSize" => Ok(Self::SocketSendBufferSize),
            "SocketReceiveBufferSize" => Ok(Self::SocketReceiveBufferSize),
            "SocketSendTimeout" => Ok(Self::SocketSendTimeout),
            "SocketReceiveTimeout" => Ok(Self::SocketReceiveTimeout),
            "IgnorePossDupResendRequests" => Ok(Self::IgnorePossDupResendRequests),
            "RequiresOrigSendingTime" => Ok(Self::RequiresOrigSendingTime),
            "CheckLatency" => Ok(Self::CheckLatency),
            "MaxLatency" => Ok(Self::MaxLatency),
            "SSLEnable" => Ok(Self::SSLEnable),
            "SSLServerName" => Ok(Self::SSLServerName),
            "SSLProtocols" => Ok(Self::SSLProtocols),
            "SSLValidateCertificates" => Ok(Self::SSLValidateCertificates),
            "SSLCheckCertificateRevocation" => Ok(Self::SSLCheckCertificateRevocation),
            "SSLCertificate" => Ok(Self::SSLCertificate),
            "SSLCertificatePassword" => Ok(Self::SSLCertificatePassword),
            "SSLRequireClientCertificate" => Ok(Self::SSLRequireClientCertificate),
            "SSLCACertificate" => Ok(Self::SSLCACertificate),
            _ => Err(Self::Error::NoSuchSetting(value.into()))
        }
    }
}

impl From<std::io::Error> for SessionSettingsError {
    fn from(error: std::io::Error) -> Self {
        SessionSettingsError::IoError(error)
    }
}

impl SessionSettings {

    pub fn from_file(filename: &str) -> Result<Self, SessionSettingsError> {
        let lines = std::fs::read_to_string(filename)?;
        Self::from_string(&lines)
    }

    pub fn from_string(string: &str) -> Result<Self, SessionSettingsError> {
        let delims: &[_] = &[ '[', ']' ];

        let mut default_started = false;
        let mut default_ended = false;
        let mut last_setting = None;

        let mut default = None;
        let mut settings = Vec::new();
        for line in string.lines() {
            //Comment
            if line.trim().starts_with('#') {
                continue
            }
            if !default_started && !default_ended && line.trim().trim_matches(delims).eq_ignore_ascii_case("default") {
                last_setting = Some(SessionSetting::default());
                default_started = true;
            } else if default_started && line.trim().trim_matches(delims).eq_ignore_ascii_case("default") {
                return Err(SessionSettingsError::DefaultSectionAlreadyDefined);
            } else if default_started && !default_ended && line.trim().trim_matches(delims).eq_ignore_ascii_case("session") {
                default = last_setting.replace(SessionSetting::default());
                default_ended = true;
            } else if default_started && default_ended && line.trim().trim_matches(delims).eq_ignore_ascii_case("session") {
                if let Some(mut value) = last_setting.replace(SessionSetting::default()) {
                    if let Some(default) = default.as_ref() {
                        settings.push(value.merge(default));
                    } else {
                        settings.push(value);
                    }
                }
            } else if let Some(setting) = last_setting.as_mut() {
                setting.set_from_line(line.trim())?;
            }
        }

        if default_started && default_ended {
            if let Some(mut value) = last_setting {
                if let Some(default) = default.as_ref() {
                    settings.push(value.merge(default));
                } else {
                    settings.push(value);
                }
            }
        }

        match (default, settings) {
            (None, _) => Err(SessionSettingsError::NoDefaultSection),
            (Some(default), v) => Ok(Self {
                default,
                sessions: v
            })
        }
    }

    pub(crate) fn default_settings(&self) -> &SessionSetting {
        &self.default
    }

    pub(crate) fn for_session_id(&self, session_id: &SessionId) -> Option<&SessionSetting> {
        let best_match = &self.sessions.iter()
                .map(|s| (s.score(session_id), s))
                .filter(|(score, _)| score > &0)
                .max_by(|(k1, _), (k2, _)| k1.cmp(k2))
                .map(|(k,v )| v);
        *best_match
    }

    pub(crate) fn sessions(&self) -> &Vec<SessionSetting> {
        self.sessions.as_ref()
    }

    pub(crate) fn sessions_by_address(&self) -> BTreeMap<SocketAddr,Vec<SessionSetting>> {
        let mut map = BTreeMap::new();
        for session in &self.sessions {
            let port = session.socket_settings().get_endpoint().unwrap();
            map.entry(port).or_insert(Vec::new()).push(session.clone());
        }
        map
    }

}

#[cfg(test)]
mod tests {
    use crate::session::{SessionSettingsError, SessionId};

    use super::SessionSettings;

    #[test]
    fn settings_test_one_sessions() {
        let data = r#"# Comment
[DEFAULT]
BeginString=TEST
SenderCompID=sender
[SESSION]
TargetCompID=target1
"#;
        let settings = SessionSettings::from_string(data);
        println!("{:?}", settings);
        assert!(settings.is_ok());
        let settings = settings.unwrap();
        assert!(settings.sessions.len() == 1);

        assert!(matches!(settings.default.begin_string, Some(v) if v.as_str() == "TEST"));
        assert!(matches!(&settings.sessions[0].begin_string, Some(v) if v.as_str() == "TEST"));

        assert!(matches!(settings.default.sender_comp_id, Some(v) if v.as_str() == "sender"));
        assert!(matches!(&settings.sessions[0].sender_comp_id, Some(v) if v.as_str() == "sender"));

        assert!(matches!(settings.default.target_comp_id, None));
        assert!(matches!(&settings.sessions[0].target_comp_id, Some(v) if v.as_str() == "target1"));
    }

    #[test]
    fn settings_test_two_sessions() {
        let data = r#"# Comment
[DEFAULT]
BeginString=TEST
SenderCompID=sender
[SESSION]
TargetCompID=target1
[SESSION]
TargetCompID=target2
[SESSION]
SenderCompID=sender_any
TargetCompID=*
"#;
        let settings = SessionSettings::from_string(data);
        println!("{:?}", settings);
        assert!(settings.is_ok());
        let settings = settings.unwrap();
        assert!(settings.sessions.len() == 3);

        assert!(matches!(&settings.default.begin_string, Some(v) if v.as_str() == "TEST"));
        assert!(matches!(&settings.sessions[0].begin_string, Some(v) if v.as_str() == "TEST"));
        assert!(matches!(&settings.sessions[1].begin_string, Some(v) if v.as_str() == "TEST"));

        assert!(matches!(&settings.default.sender_comp_id, Some(v) if v.as_str() == "sender"));
        assert!(matches!(&settings.sessions[0].sender_comp_id, Some(v) if v.as_str() == "sender"));
        assert!(matches!(&settings.sessions[1].sender_comp_id, Some(v) if v.as_str() == "sender"));

        assert!(matches!(&settings.default.target_comp_id, None));
        assert!(matches!(&settings.sessions[0].target_comp_id, Some(v) if v.as_str() == "target1"));
        assert!(matches!(&settings.sessions[1].target_comp_id, Some(v) if v.as_str() == "target2"));

        let session_id = SessionId::new("", "sender", "", "", "target1", "", "");
        assert_eq!(Some(&settings.sessions[0]), settings.for_session_id(&session_id));
        let session_id = SessionId::new("", "sender", "", "", "target2", "", "");
        assert_eq!(Some(&settings.sessions[1]), settings.for_session_id(&session_id));
        let session_id = SessionId::new("", "sender", "", "", "target3", "", "");
        assert_eq!(None, settings.for_session_id(&session_id));
        let session_id = SessionId::new("", "sender_any", "", "", "target_any_1", "", "");
        assert_eq!(Some(&settings.sessions[2]), settings.for_session_id(&session_id));
        let session_id = SessionId::new("", "sender_any", "", "", "target_any_2", "", "");
        assert_eq!(Some(&settings.sessions[2]), settings.for_session_id(&session_id));
    }

    #[test]
    fn settings_test_no_default() {
        let data = r#"# Comment
"#;
        let settings = SessionSettings::from_string(data);
        assert!(matches!(settings, Err(SessionSettingsError::NoDefaultSection)));
    }

    #[test]
    fn settings_test_double_default() {
        let data = r#"# Comment
[DEFAULT]
[SESSION]
[DEFAULT]
"#;
        let settings = SessionSettings::from_string(data);
        assert!(matches!(settings, Err(SessionSettingsError::DefaultSectionAlreadyDefined)));
    }

    #[test]
    fn settings_test_double_default_alt() {
        let data = r#"# Comment
[DEFAULT]
[DEFAULT]
[SESSION]
"#;
        let settings = SessionSettings::from_string(data);
        assert!(matches!(settings, Err(SessionSettingsError::DefaultSectionAlreadyDefined)));
    }

    #[test]
    fn settings_test_invalid_setting() {
        let data = r#"# Comment
[DEFAULT]
asdfasd=Y
"#;
        let settings = SessionSettings::from_string(data);
        assert!(matches!(settings, Err(SessionSettingsError::NoSuchSetting(_))));
    }
}
