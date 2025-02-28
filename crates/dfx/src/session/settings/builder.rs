use std::{fs::File, io::Read, net::ToSocketAddrs};

use chrono::NaiveTime;

use crate::session::SessionSchedule;
use dfx_base::fields::converters::datetime::DateTimeFormat;
use dfx_base::session_id::SessionId;
use native_tls::{Certificate, Identity, Protocol, TlsAcceptor, TlsConnector};

use super::{
    ConnectionType, LoggingOptions, Persistence, SessionSetting, SessionSettingsError,
    SettingOption, SettingsConnection, SocketOptions, SslOptions, ValidationOptions,
};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(crate) struct DynamicSessionSettingBuilder {
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
    ssl_min_protocol: Option<String>,
    ssl_max_protocol: Option<String>,
    ssl_use_sni: Option<String>,
    ssl_accept_invalid_certs: Option<String>,
    ssl_accept_invalid_hostnames: Option<String>,
    ssl_disable_built_in_roots: Option<String>,
    ssl_certificate: Option<String>,
    ssl_certificate_password: Option<String>,
    ssl_require_client_certificate: Option<String>,
    ssl_ca_certificate: Option<String>,
}

pub(crate) struct Validated(DynamicSessionSettingBuilder);
impl Validated {
    pub(crate) fn build(self) -> Result<SessionSetting, SessionSettingsError> {
        self.0.build()
    }
}

impl DynamicSessionSettingBuilder {
    fn set(&mut self, option: SettingOption, value: &str) {
        match option {
            SettingOption::IsDynamic => self.is_dynamic = Some(value.into()),
            SettingOption::BeginString => self.begin_string = Some(value.into()),
            SettingOption::SenderCompID => self.sender_comp_id = Some(value.into()),
            SettingOption::SenderSubID => self.sender_sub_id = Some(value.into()),
            SettingOption::SenderLocationID => self.sender_location_id = Some(value.into()),
            SettingOption::TargetCompID => self.target_comp_id = Some(value.into()),
            SettingOption::TargetSubID => self.target_sub_id = Some(value.into()),
            SettingOption::TargetLocationID => self.target_location_id = Some(value.into()),
            SettingOption::SessionQualifier => self.session_qualifier = Some(value.into()),
            SettingOption::DefaultApplVerID => self.default_appl_ver_id = Some(value.into()),
            SettingOption::ConnectionType => self.connection_type = Some(value.into()),
            SettingOption::UseDataDictionary => self.use_data_dictionary = Some(value.into()),
            SettingOption::NonStopSession => self.non_stop_session = Some(value.into()),
            SettingOption::UseLocalTime => self.use_local_time = Some(value.into()),
            SettingOption::TimeZone => self.time_zone = Some(value.into()),
            SettingOption::StartDay => self.start_day = Some(value.into()),
            SettingOption::EndDay => self.end_day = Some(value.into()),
            SettingOption::StartTime => self.start_time = Some(value.into()),
            SettingOption::EndTime => self.end_time = Some(value.into()),
            SettingOption::HeartBtInt => self.heart_bt_int = Some(value.into()),
            SettingOption::SocketAcceptHost => self.socket_accept_host = Some(value.into()),
            SettingOption::SocketAcceptPort => self.socket_accept_port = Some(value.into()),
            SettingOption::SocketConnectHost => self.socket_connect_host = Some(value.into()),
            SettingOption::SocketConnectPort => self.socket_connect_port = Some(value.into()),
            SettingOption::ReconnectInterval => self.reconnect_interval = Some(value.into()),
            SettingOption::FileLogPath => self.file_log_path = Some(value.into()),
            SettingOption::DebugFileLogPath => self.debug_file_log_path = Some(value.into()),
            SettingOption::FileStorePath => self.file_store_path = Some(value.into()),
            SettingOption::RefreshOnLogon => self.refresh_on_logon = Some(value.into()),
            SettingOption::ResetOnLogon => self.reset_on_logon = Some(value.into()),
            SettingOption::ResetOnLogout => self.reset_on_logout = Some(value.into()),
            SettingOption::ResetOnDisconnect => self.reset_on_disconnect = Some(value.into()),
            SettingOption::ValidateFieldsOutOfOrder => {
                self.validate_fields_out_of_order = Some(value.into());
            }
            SettingOption::ValidateFieldsHaveValues => {
                self.validate_fields_have_values = Some(value.into());
            }
            SettingOption::ValidateUserDefinedFields => {
                self.validate_user_defined_fields = Some(value.into());
            }
            SettingOption::ValidateLengthAndChecksum => {
                self.validate_length_and_checksum = Some(value.into());
            }
            SettingOption::AllowUnknownMsgFields => {
                self.allow_unknown_msg_fields = Some(value.into());
            }
            SettingOption::DataDictionary => self.data_dictionary = Some(value.into()),
            SettingOption::TransportDataDictionary => {
                self.transport_data_dictionary = Some(value.into());
            }
            SettingOption::AppDataDictionary => self.app_data_dictionary = Some(value.into()),
            SettingOption::PersistMessages => self.persist_messages = Some(value.into()),
            SettingOption::LogonTimeout => self.logon_timeout = Some(value.into()),
            SettingOption::LogoutTimeout => self.logout_timeout = Some(value.into()),
            SettingOption::SendRedundantResendRequests => {
                self.send_redundant_resend_requests = Some(value.into());
            }
            SettingOption::ResendSessionLevelRejects => {
                self.resend_session_level_rejects = Some(value.into());
            }
            SettingOption::MillisecondsInTimeStamp => {
                self.milliseconds_in_time_stamp = Some(value.into());
            }
            SettingOption::TimeStampPrecision => self.time_stamp_precision = Some(value.into()),
            SettingOption::EnableLastMsgSeqNumProcessed => {
                self.enable_last_msg_seq_num_processed = Some(value.into());
            }
            SettingOption::MaxMessagesInResendRequest => {
                self.max_messages_in_resend_request = Some(value.into());
            }
            SettingOption::SendLogoutBeforeDisconnectFromTimeout => {
                self.send_logout_before_disconnect_from_timeout = Some(value.into());
            }
            SettingOption::SocketNodelay => self.socket_nodelay = Some(value.into()),
            SettingOption::SocketSendBufferSize => {
                self.socket_send_buffer_size = Some(value.into());
            }
            SettingOption::SocketReceiveBufferSize => {
                self.socket_receive_buffer_size = Some(value.into());
            }
            SettingOption::SocketSendTimeout => self.socket_send_timeout = Some(value.into()),
            SettingOption::SocketReceiveTimeout => self.socket_receive_timeout = Some(value.into()),
            SettingOption::IgnorePossDupResendRequests => {
                self.ignore_poss_dup_resend_requests = Some(value.into());
            }
            SettingOption::RequiresOrigSendingTime => {
                self.requires_orig_sending_time = Some(value.into());
            }
            SettingOption::CheckLatency => self.check_latency = Some(value.into()),
            SettingOption::MaxLatency => self.max_latency = Some(value.into()),
            SettingOption::SSLEnable => self.ssl_enable = Some(value.into()),
            SettingOption::SSLServerName => self.ssl_server_name = Some(value.into()),
            SettingOption::SSLMinProtocol => self.ssl_min_protocol = Some(value.into()),
            SettingOption::SSLMaxProtocol => self.ssl_max_protocol = Some(value.into()),
            SettingOption::SSLUseSNI => self.ssl_use_sni = Some(value.into()),
            SettingOption::SSLAcceptInvalidCerts => {
                self.ssl_accept_invalid_certs = Some(value.into());
            }
            SettingOption::SSLAcceptInvalidHostnames => {
                self.ssl_accept_invalid_hostnames = Some(value.into());
            }
            SettingOption::SSLDisableBuiltInRoots => {
                self.ssl_disable_built_in_roots = Some(value.into());
            }
            SettingOption::SSLCertificate => self.ssl_certificate = Some(value.into()),
            SettingOption::SSLCertificatePassword => {
                self.ssl_certificate_password = Some(value.into());
            }
            SettingOption::SSLRequireClientCertificate => {
                self.ssl_require_client_certificate = Some(value.into());
            }
            SettingOption::SSLCACertificate => self.ssl_ca_certificate = Some(value.into()),
        }
    }

    pub(crate) fn set_from_line(
        &mut self,
        line_num: usize,
        line: &str,
    ) -> Result<(), SessionSettingsError> {
        if line.is_empty() {
            return Ok(());
        }
        let setting: Vec<&str> = line.split('=').collect();
        if setting.len() > 2 {
            Err(SessionSettingsError::LineParseError {
                line_number: line_num,
                line: line.into(),
                reason: "Too many '=' in line.".into(),
            })
        } else if setting.len() == 2 {
            let option: SettingOption = setting[0].try_into()?;
            let value = setting[1];
            self.set(option, value);
            Ok(())
        } else {
            Err(SessionSettingsError::LineParseError {
                line_number: line_num,
                line: line.into(),
                reason: "No value specified.".into(),
            })
        }
    }

    pub(crate) fn merge(mut self, other: &Self) -> Self {
        self.is_dynamic = self.is_dynamic.or(other.is_dynamic.clone());
        self.begin_string = self.begin_string.or(other.begin_string.clone());
        self.sender_comp_id = self.sender_comp_id.or(other.sender_comp_id.clone());
        self.sender_sub_id = self.sender_sub_id.or(other.sender_sub_id.clone());
        self.sender_location_id = self.sender_location_id.or(other.sender_location_id.clone());
        self.target_comp_id = self.target_comp_id.or(other.target_comp_id.clone());
        self.target_sub_id = self.target_sub_id.or(other.target_sub_id.clone());
        self.target_location_id = self.target_location_id.or(other.target_location_id.clone());
        self.session_qualifier = self.session_qualifier.or(other.session_qualifier.clone());
        self.default_appl_ver_id = self
            .default_appl_ver_id
            .or(other.default_appl_ver_id.clone());
        self.connection_type = self.connection_type.or(other.connection_type.clone());
        self.non_stop_session = self.non_stop_session.or(other.non_stop_session.clone());
        self.use_local_time = self.use_local_time.or(other.use_local_time.clone());
        self.time_zone = self.time_zone.or(other.time_zone.clone());
        self.start_day = self.start_day.or(other.start_day.clone());
        self.end_day = self.end_day.or(other.end_day.clone());
        self.start_time = self.start_time.or(other.start_time.clone());
        self.end_time = self.end_time.or(other.end_time.clone());
        self.milliseconds_in_time_stamp = self
            .milliseconds_in_time_stamp
            .or(other.milliseconds_in_time_stamp.clone());
        self.refresh_on_logon = self.refresh_on_logon.or(other.refresh_on_logon.clone());
        self.reset_on_logon = self.reset_on_logon.or(other.reset_on_logon.clone());
        self.reset_on_logout = self.reset_on_logout.or(other.reset_on_logout.clone());
        self.reset_on_disconnect = self
            .reset_on_disconnect
            .or(other.reset_on_disconnect.clone());
        self.send_redundant_resend_requests = self
            .send_redundant_resend_requests
            .or(other.send_redundant_resend_requests.clone());
        self.resend_session_level_rejects = self
            .resend_session_level_rejects
            .or(other.resend_session_level_rejects.clone());
        self.time_stamp_precision = self
            .time_stamp_precision
            .or(other.time_stamp_precision.clone());
        self.enable_last_msg_seq_num_processed = self
            .enable_last_msg_seq_num_processed
            .or(other.enable_last_msg_seq_num_processed.clone());
        self.max_messages_in_resend_request = self
            .max_messages_in_resend_request
            .or(other.max_messages_in_resend_request.clone());
        self.send_logout_before_disconnect_from_timeout = self
            .send_logout_before_disconnect_from_timeout
            .or(other.send_logout_before_disconnect_from_timeout.clone());
        self.ignore_poss_dup_resend_requests = self
            .ignore_poss_dup_resend_requests
            .or(other.ignore_poss_dup_resend_requests.clone());
        self.requires_orig_sending_time = self
            .requires_orig_sending_time
            .or(other.requires_orig_sending_time.clone());

        // validation options
        self.use_data_dictionary = self
            .use_data_dictionary
            .or(other.use_data_dictionary.clone());
        self.data_dictionary = self.data_dictionary.or(other.data_dictionary.clone());
        self.transport_data_dictionary = self
            .transport_data_dictionary
            .or(other.transport_data_dictionary.clone());
        self.app_data_dictionary = self
            .app_data_dictionary
            .or(other.app_data_dictionary.clone());
        self.validate_fields_out_of_order = self
            .validate_fields_out_of_order
            .or(other.validate_fields_out_of_order.clone());
        self.validate_fields_have_values = self
            .validate_fields_have_values
            .or(other.validate_fields_have_values.clone());
        self.validate_user_defined_fields = self
            .validate_user_defined_fields
            .or(other.validate_user_defined_fields.clone());
        self.validate_length_and_checksum = self
            .validate_length_and_checksum
            .or(other.validate_length_and_checksum.clone());
        self.allow_unknown_msg_fields = self
            .allow_unknown_msg_fields
            .or(other.allow_unknown_msg_fields.clone());
        self.check_latency = self.check_latency.or(other.check_latency.clone());
        self.max_latency = self.max_latency.or(other.max_latency.clone());

        // initiator options
        self.reconnect_interval = self.reconnect_interval.or(other.reconnect_interval.clone());
        self.heart_bt_int = self.heart_bt_int.or(other.heart_bt_int.clone());
        self.logon_timeout = self.logon_timeout.or(other.logon_timeout.clone());
        self.logout_timeout = self.logout_timeout.or(other.logout_timeout.clone());
        self.socket_connect_host = self
            .socket_connect_host
            .or(other.socket_connect_host.clone());
        self.socket_connect_port = self
            .socket_connect_port
            .or(other.socket_connect_port.clone());
        self.socket_connect_hosts = self
            .socket_connect_hosts
            .or(other.socket_connect_hosts.clone());
        self.socket_connect_ports = self
            .socket_connect_ports
            .or(other.socket_connect_ports.clone());

        // acceptor options
        self.socket_accept_host = self.socket_accept_host.or(other.socket_accept_host.clone());
        self.socket_accept_port = self.socket_accept_port.or(other.socket_accept_port.clone());

        // storage
        self.persist_messages = self.persist_messages.or(other.persist_messages.clone());
        // store path
        self.file_store_path = self.file_store_path.or(other.file_store_path.clone());

        // logging
        self.file_log_path = self.file_log_path.or(other.file_log_path.clone());
        self.debug_file_log_path = self
            .debug_file_log_path
            .or(other.debug_file_log_path.clone());

        // Socket options
        self.socket_nodelay = self.socket_nodelay.or(other.socket_nodelay.clone());
        self.socket_send_buffer_size = self
            .socket_send_buffer_size
            .or(other.socket_send_buffer_size.clone());
        self.socket_receive_buffer_size = self
            .socket_receive_buffer_size
            .or(other.socket_receive_buffer_size.clone());
        self.socket_send_timeout = self
            .socket_send_timeout
            .or(other.socket_send_timeout.clone());
        self.socket_receive_timeout = self
            .socket_receive_timeout
            .or(other.socket_receive_timeout.clone());

        // SSL options

        self.ssl_server_name = self.ssl_server_name.or(other.ssl_server_name.clone());
        self.ssl_min_protocol = self.ssl_min_protocol.or(other.ssl_min_protocol.clone());
        self.ssl_max_protocol = self.ssl_max_protocol.or(other.ssl_max_protocol.clone());
        self.ssl_use_sni = self.ssl_use_sni.or(other.ssl_use_sni.clone());
        self.ssl_accept_invalid_certs = self
            .ssl_accept_invalid_certs
            .or(other.ssl_accept_invalid_certs.clone());
        self.ssl_accept_invalid_hostnames = self
            .ssl_accept_invalid_hostnames
            .or(other.ssl_accept_invalid_hostnames.clone());
        self.ssl_disable_built_in_roots = self
            .ssl_disable_built_in_roots
            .or(other.ssl_disable_built_in_roots.clone());
        self.ssl_certificate = self.ssl_certificate.or(other.ssl_certificate.clone());
        self.ssl_certificate_password = self
            .ssl_certificate_password
            .or(other.ssl_certificate_password.clone());
        self.ssl_require_client_certificate = self
            .ssl_require_client_certificate
            .or(other.ssl_require_client_certificate.clone());
        self.ssl_ca_certificate = self.ssl_ca_certificate.or(other.ssl_ca_certificate.clone());
        self
    }

    //TODO validate + log defaults
    pub(crate) fn validate(self) -> Result<Validated, SessionSettingsError> {
        let mut errors = Vec::new();

        let conn_type: Option<Result<ConnectionType, _>> =
            self.connection_type.as_ref().map(|v| v.as_str().try_into());
        match conn_type {
            Some(Ok(_)) => (),
            _ => errors
                .push("ConnectionType must be set to either 'acceptor' or 'initiator'.".into()),
        }

        if self.begin_string.is_none() {
            //TODO check valid begin strings
            errors.push("BeginString must be set.".into());
        }

        if self.sender_comp_id.is_none() {
            errors.push("SenderCompID must be set.".into());
        }

        if self.target_comp_id.is_none() {
            errors.push("TargetCompID must be set.".into());
        }

        if !errors.is_empty() {
            Err(SessionSettingsError::ValidationErrors(errors))
        } else {
            Ok(Validated(self))
        }
    }

    // TODO check if these are the correct default values
    fn build(self) -> Result<SessionSetting, SessionSettingsError> {
        let mut builder = SessionSetting::builder();

        let session_id = SessionId::new(
            self.begin_string.unwrap_or_default(),
            self.sender_comp_id.unwrap_or_default(),
            self.sender_sub_id.unwrap_or_default(),
            self.sender_location_id.unwrap_or_default(),
            self.target_comp_id.unwrap_or_default(),
            self.target_sub_id.unwrap_or_default(),
            self.target_location_id.unwrap_or_default(),
        );
        builder.session_id(session_id);

        let connection = match self.connection_type.unwrap().as_str() {
            "acceptor" => SettingsConnection::Acceptor {
                is_dynamic: self.is_dynamic.is_some_and(|v| v == "Y"),
                session_qualifier: self.session_qualifier,
                accept_addr: format!(
                    "{}:{}",
                    self.socket_accept_host
                        .unwrap_or_else(|| "127.0.0.1".to_string()),
                    self.socket_accept_port.unwrap()
                )
                .parse()
                .unwrap(),
                logon_timeout: self
                    .logon_timeout
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(10),
                logout_timeout: self
                    .logout_timeout
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(2),
            },
            "initiator" => SettingsConnection::Initiator {
                connect_addr: format!(
                    "{}:{}",
                    self.socket_connect_host.as_ref().unwrap(),
                    self.socket_connect_port.unwrap()
                )
                .to_socket_addrs()
                .unwrap()
                .next()
                .unwrap(),
                reconnect_interval: self
                    .reconnect_interval
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(30),
                heart_bt_int: self.heart_bt_int.and_then(|v| v.parse().ok()).unwrap_or(30),
                logon_timeout: self
                    .logon_timeout
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(10),
                logout_timeout: self
                    .logout_timeout
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(2),
            },
            _ => unreachable!(),
        };
        let is_initiator = connection.is_initiator();
        builder.connection(connection);

        let mut socket_options = SocketOptions::builder();
        socket_options.no_delay(self.socket_nodelay.map_or(true, |v| v == "Y"));
        socket_options.receive_buffer_size(
            self.socket_receive_buffer_size
                .and_then(|v| v.parse().ok())
                .unwrap_or(8192),
        );
        socket_options.send_buffer_size(
            self.socket_send_buffer_size
                .and_then(|v| v.parse().ok())
                .unwrap_or(8192),
        );
        socket_options.receive_timeout(
            self.socket_receive_timeout
                .and_then(|v| v.parse().ok())
                .unwrap_or(0),
        );
        socket_options.send_timeout(
            self.socket_send_timeout
                .and_then(|v| v.parse().ok())
                .unwrap_or(0),
        );

        builder.socket_options(socket_options.build().unwrap());

        let logging = LoggingOptions::builder()
            .file_log_path(self.file_log_path.clone())
            .debug_file_log_path(self.file_log_path.clone())
            .build()
            .unwrap();

        builder.logging(logging);

        let persistence = if self.persist_messages.map_or(true, |v| v == "Y") {
            match self.file_store_path {
                Some(value) => Persistence::FileStore { path: value.into() },
                //TODO default to path log?
                None => Persistence::Memory,
            }
        } else {
            Persistence::None
        };
        builder.persistence(persistence);
        builder.default_appl_ver_id(self.default_appl_ver_id);

        let any_set = self
            .start_day
            .as_ref()
            .or(self.end_day.as_ref())
            .or(self.end_time.as_ref())
            .or(self.start_time.as_ref())
            .is_some();
        let schedule = if self.non_stop_session.map_or(true, |v| v == "Y") && !any_set {
            SessionSchedule::NonStop
        // } else if self.even_minutes_session.map(|v| v == "Y").unwrap_or(true) && !any_set {
        //     #[cfg(test)]
        //     SessionSchedule::EvenMinutes
        } else {
            match (
                self.start_day,
                self.end_day,
                self.start_time,
                self.end_time,
                self.time_zone,
            ) {
                (Some(start_day), Some(end_day), Some(start_time), Some(end_time), timezone) => {
                    let start_day = start_day.parse().unwrap();
                    let end_day = end_day.parse().unwrap();
                    let start_time = NaiveTime::parse_from_str(&start_time, "%H:%M:%S").unwrap();
                    let end_time = NaiveTime::parse_from_str(&end_time, "%H:%M:%S").unwrap();
                    let use_localtime: bool = self.use_local_time.is_some_and(|v| v == "Y");
                    let timezone = timezone.and_then(|tz| tz.parse().ok());
                    SessionSchedule::Weekly {
                        start_day,
                        end_day,
                        start_time,
                        end_time,
                        timezone,
                        use_localtime,
                    }
                }
                (None, None, Some(start_time), Some(end_time), timezone) => {
                    let start_time = NaiveTime::parse_from_str(&start_time, "%H:%M:%S").unwrap();
                    let end_time = NaiveTime::parse_from_str(&end_time, "%H:%M:%S").unwrap();
                    let use_localtime: bool = self.use_local_time.is_some_and(|v| v == "Y");
                    let timezone = timezone.and_then(|tz| tz.parse().ok());
                    SessionSchedule::Daily {
                        start_time,
                        end_time,
                        timezone,
                        use_localtime,
                    }
                }
                (None, None, None, None, None) => SessionSchedule::NonStop,
                _ => unreachable!(),
            }
        };
        builder.schedule(schedule);

        let validation_options = ValidationOptions::builder()
            .milliseconds_in_time_stamp(self.milliseconds_in_time_stamp.is_some_and(|v| v == "Y"))
            .refresh_on_logon(self.refresh_on_logon.is_some_and(|v| v == "Y"))
            .reset_on_logon(self.reset_on_logon.is_some_and(|v| v == "Y"))
            .reset_on_logout(self.reset_on_logout.is_some_and(|v| v == "Y"))
            .reset_on_disconnect(self.reset_on_disconnect.is_some_and(|v| v == "Y"))
            .send_redundant_resend_requests(
                self.send_redundant_resend_requests
                    .is_some_and(|v| v == "Y"),
            )
            .resend_session_level_rejects(
                self.resend_session_level_rejects.is_some_and(|v| v == "Y"),
            )
            .time_stamp_precision(
                self.time_stamp_precision
                    .and_then(|v| v.try_into().ok())
                    .unwrap_or(DateTimeFormat::Seconds),
            )
            .enable_last_msg_seq_num_processed(
                self.enable_last_msg_seq_num_processed
                    .is_some_and(|v| v == "Y"),
            )
            .max_messages_in_resend_request(
                self.max_messages_in_resend_request
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(0),
            )
            .send_logout_before_disconnect_from_timeout(
                self.send_logout_before_disconnect_from_timeout
                    .is_some_and(|v| v == "Y"),
            )
            .ignore_poss_dup_resend_requests(
                self.ignore_poss_dup_resend_requests
                    .is_some_and(|v| v == "Y"),
            )
            .requires_orig_sending_time(self.requires_orig_sending_time.map_or(true, |v| v == "Y"))
            .use_data_dictionary(self.use_data_dictionary.is_some_and(|v| v == "Y"))
            .data_dictionary(self.data_dictionary)
            .transport_data_dictionary(self.transport_data_dictionary)
            .app_data_dictionary(self.app_data_dictionary)
            .validate_fields_out_of_order(
                self.validate_fields_out_of_order.map_or(true, |v| v == "Y"),
            )
            .validate_fields_have_values(
                self.validate_fields_have_values.map_or(true, |v| v == "Y"),
            )
            .validate_user_defined_fields(
                self.validate_user_defined_fields.map_or(true, |v| v == "Y"),
            )
            .validate_length_and_checksum(
                self.validate_length_and_checksum.map_or(true, |v| v == "Y"),
            )
            .allow_unknown_msg_fields(self.allow_unknown_msg_fields.is_some_and(|v| v == "Y"))
            .check_latency(self.check_latency.map_or(true, |v| v == "Y"))
            .max_latency(self.max_latency.and_then(|v| v.parse().ok()).unwrap_or(120))
            .build()
            .unwrap();
        builder.validation_options(validation_options);

        let ssl_options = match (self.ssl_enable.as_deref(), is_initiator) {
            (Some(_x @ "Y"), true) => {
                let mut builder = TlsConnector::builder();
                if let Some(certificate_file) = self.ssl_certificate {
                    let mut file = File::open(certificate_file).unwrap();
                    let mut buf = vec![];
                    file.read_to_end(&mut buf).unwrap();
                    let certificate = Certificate::from_pem(&buf).unwrap();
                    let identity = Identity::from_pkcs12(
                        &certificate.to_der().unwrap(),
                        self.ssl_certificate_password.as_ref().unwrap(),
                    )
                    .unwrap();
                    builder.identity(identity);
                }
                builder
                    .min_protocol_version(self.ssl_min_protocol.as_ref().and_then(|r| protocol(r)));
                builder
                    .max_protocol_version(self.ssl_max_protocol.as_ref().and_then(|r| protocol(r)));
                builder.use_sni(self.ssl_use_sni.as_ref().map_or(true, |v| v == "Y"));
                builder.danger_accept_invalid_certs(
                    self.ssl_accept_invalid_certs
                        .as_ref()
                        .is_some_and(|v| v == "Y"),
                );
                builder.danger_accept_invalid_hostnames(
                    self.ssl_accept_invalid_hostnames
                        .as_ref()
                        .is_some_and(|v| v == "Y"),
                );
                builder.disable_built_in_roots(
                    self.ssl_disable_built_in_roots
                        .as_ref()
                        .is_some_and(|v| v == "Y"),
                );

                // root_certificates: vec![],
                // #[cfg(feature = "alpn")]
                // alpn: vec![],

                let host = self.socket_connect_host.clone();
                let domain = self.ssl_server_name.clone().or(host).unwrap_or_default();
                let initiator = builder.build().unwrap();
                Some(SslOptions::Initiator { initiator, domain })
            }
            (Some(_x @ "Y"), false) => {
                let mut file = File::open(self.ssl_certificate.unwrap()).unwrap();
                let mut buf = vec![];
                file.read_to_end(&mut buf).unwrap();
                let certificate = Certificate::from_pem(&buf).unwrap();
                let identity = Identity::from_pkcs12(
                    &certificate.to_der().unwrap(),
                    self.ssl_certificate_password.as_ref().unwrap(),
                )
                .unwrap();
                let mut builder = TlsAcceptor::builder(identity);
                builder
                    .min_protocol_version(self.ssl_min_protocol.as_ref().and_then(|r| protocol(r)));
                builder
                    .max_protocol_version(self.ssl_max_protocol.as_ref().and_then(|r| protocol(r)));
                let acceptor = builder.build().unwrap();
                Some(SslOptions::Acceptor { acceptor })
            }
            _ => None,
        };
        builder.ssl_options(ssl_options);
        Ok(builder.build().ok().unwrap())
    }
}

fn protocol(protocol: &str) -> Option<Protocol> {
    match protocol {
        "TLSv1.0" => Some(Protocol::Tlsv10),
        "TLSv1.1" => Some(Protocol::Tlsv11),
        "TLSv1.2" => Some(Protocol::Tlsv12),
        "SSLv3" => Some(Protocol::Sslv3),
        _ => None,
    }
}
