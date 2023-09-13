#![allow(dead_code)]
use std::{
    net::SocketAddr,
    path::PathBuf,
};

use derive_builder::Builder;

use dfx_base::fields::converters::datetime::DateTimeFormat;
use native_tls::{TlsAcceptor, TlsConnector};
use crate::{
    connection::SocketSettings,
    session::SessionSchedule,
};

use dfx_base::session_id::SessionId;

use super::{SessionSettingsError, SettingOption};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ConnectionType {
    Acceptor,
    Initiator,
}

impl TryFrom<&str> for ConnectionType {
    type Error = SessionSettingsError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "initiator" => Ok(Self::Initiator),
            "acceptor" => Ok(Self::Acceptor),
            e => Err(SessionSettingsError::InvalidValue {
                setting: SettingOption::ConnectionType.into(),
                value: e.into(),
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SettingsConnection {
    Acceptor {
        is_dynamic: bool,
        session_qualifier: Option<String>,
        accept_addr: SocketAddr,
        logon_timeout: u32,
        logout_timeout: u32,
    },
    Initiator {
        connect_addr: SocketAddr,

        // reconnect options
        reconnect_interval: u32,
        heart_bt_int: u32,
        logon_timeout: u32,
        logout_timeout: u32,
    },
}

impl SettingsConnection {
    /// Returns `true` if the settings connection is [`Initiator`].
    ///
    /// [`Initiator`]: SettingsConnection::Initiator
    #[must_use]
    pub(crate) fn is_initiator(&self) -> bool {
        matches!(self, Self::Initiator { .. })
    }

    /// Returns `true` if the settings connection is [`Acceptor`].
    ///
    /// [`Acceptor`]: SettingsConnection::Acceptor
    #[must_use]
    pub(crate) fn is_acceptor(&self) -> bool {
        matches!(self, Self::Acceptor { .. })
    }

    pub(crate) fn socket_addr(&self) -> &SocketAddr {
        match self {
            SettingsConnection::Acceptor { accept_addr, .. } => accept_addr,
            SettingsConnection::Initiator { connect_addr, .. } => connect_addr,
        }
    }

    pub(crate) fn heart_bt_int(&self) -> Option<u32> {
        match self {
            SettingsConnection::Acceptor { .. } => None,
            SettingsConnection::Initiator { heart_bt_int, .. } => Some(*heart_bt_int),
        }
    }

    pub(crate) fn logon_timeout(&self) -> u32 {
        match self {
            SettingsConnection::Acceptor { logon_timeout, .. } => *logon_timeout,
            SettingsConnection::Initiator { logon_timeout, .. } => *logon_timeout,
        }
    }

    pub(crate) fn logout_timeout(&self) -> u32 {
        match self {
            SettingsConnection::Acceptor { logout_timeout, .. } => *logout_timeout,
            SettingsConnection::Initiator { logout_timeout, .. } => *logout_timeout,
        }
    }
}

#[derive(Builder, Clone, Debug, PartialEq, Eq)]
pub(crate) struct SocketOptions {
    no_delay: bool,
    send_buffer_size: usize,
    receive_buffer_size: usize,
    send_timeout: u64,
    receive_timeout: u64,
}

impl SocketOptions {

    pub(crate) fn builder() -> SocketOptionsBuilder {
        SocketOptionsBuilder::create_empty()
    }

    pub(crate) fn no_delay(&self) -> bool {
        self.no_delay
    }

    // pub(crate) fn send_buffer_size(&self) -> usize {
    //     self.send_buffer_size
    // }

    // pub(crate) fn receive_buffer_size(&self) -> usize {
    //     self.receive_buffer_size
    // }

    pub(crate) fn send_timeout(&self) -> u64 {
        self.send_timeout
    }

    pub(crate) fn receive_timeout(&self) -> u64 {
        self.receive_timeout
    }
}

#[derive(Clone)]
pub(crate) enum SslOptions {
    Acceptor { acceptor: TlsAcceptor },
    Initiator { initiator: TlsConnector, domain: String }
}

impl std::fmt::Debug for SslOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Acceptor { acceptor: _ } => f.debug_struct("Acceptor").finish(),
            Self::Initiator { initiator, domain } => f.debug_struct("Initiator").field("initiator", initiator).field("domain", domain).finish(),
        }
    }
}

#[derive(Builder, Clone, Debug, PartialEq, Eq)]
pub struct LoggingOptions {
    file_log_path: Option<String>,
    debug_file_log_path: Option<String>,
}

impl LoggingOptions {
    pub(crate) fn builder() -> LoggingOptionsBuilder {
        LoggingOptionsBuilder::create_empty()
    }

    pub(crate) fn file_log_path(&self) -> Option<&String> {
        self.file_log_path.as_ref()
    }

    pub(crate) fn debug_file_log_path(&self) -> Option<&String> {
        self.debug_file_log_path.as_ref()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Persistence {
    FileStore { path: PathBuf },
    Memory,
    None,
}

#[derive(Builder, Clone, Debug, PartialEq, Eq)]
pub(crate) struct ValidationOptions {
    milliseconds_in_time_stamp: bool,
    refresh_on_logon: bool,
    reset_on_logon: bool,
    reset_on_logout: bool,
    reset_on_disconnect: bool,
    send_redundant_resend_requests: bool,
    resend_session_level_rejects: bool,
    time_stamp_precision: DateTimeFormat,
    enable_last_msg_seq_num_processed: bool,
    max_messages_in_resend_request: u32,
    send_logout_before_disconnect_from_timeout: bool,
    ignore_poss_dup_resend_requests: bool,
    requires_orig_sending_time: bool,

    // validation options
    use_data_dictionary: bool,
    data_dictionary: Option<String>,
    transport_data_dictionary: Option<String>,
    app_data_dictionary: Option<String>,
    validate_fields_out_of_order: bool,
    validate_fields_have_values: bool,
    validate_user_defined_fields: bool,
    validate_length_and_checksum: bool,
    allow_unknown_msg_fields: bool,
    check_latency: bool,
    max_latency: u32,
}

impl ValidationOptions {
    pub(crate) fn builder() -> ValidationOptionsBuilder {
        ValidationOptionsBuilder::create_empty()
    }

    pub(crate) fn milliseconds_in_time_stamp(&self) -> bool {
        self.milliseconds_in_time_stamp
    }

    pub(crate) fn refresh_on_logon(&self) -> bool {
        self.refresh_on_logon
    }

    pub(crate) fn reset_on_logon(&self) -> bool {
        self.reset_on_logon
    }

    pub(crate) fn reset_on_logout(&self) -> bool {
        self.reset_on_logout
    }

    pub(crate) fn reset_on_disconnect(&self) -> bool {
        self.reset_on_disconnect
    }

    pub(crate) fn send_redundant_resend_requests(&self) -> bool {
        self.send_redundant_resend_requests
    }

    pub(crate) fn resend_session_level_rejects(&self) -> bool {
        self.resend_session_level_rejects
    }

    pub(crate) fn time_stamp_precision(&self) -> &DateTimeFormat {
        &self.time_stamp_precision
    }

    pub(crate) fn enable_last_msg_seq_num_processed(&self) -> bool {
        self.enable_last_msg_seq_num_processed
    }

    pub(crate) fn max_messages_in_resend_request(&self) -> u32 {
        self.max_messages_in_resend_request
    }

    pub(crate) fn send_logout_before_disconnect_from_timeout(&self) -> bool {
        self.send_logout_before_disconnect_from_timeout
    }

    pub(crate) fn ignore_poss_dup_resend_requests(&self) -> bool {
        self.ignore_poss_dup_resend_requests
    }

    pub(crate) fn requires_orig_sending_time(&self) -> bool {
        self.requires_orig_sending_time
    }

    pub(crate) fn use_data_dictionary(&self) -> bool {
        self.use_data_dictionary
    }

    pub(crate) fn data_dictionary(&self) -> Option<&String> {
        self.data_dictionary.as_ref()
    }

    pub(crate) fn transport_data_dictionary(&self) -> Option<&String> {
        self.transport_data_dictionary.as_ref()
    }

    pub(crate) fn app_data_dictionary(&self) -> Option<&String> {
        self.app_data_dictionary.as_ref()
    }

    pub(crate) fn validate_fields_out_of_order(&self) -> bool {
        self.validate_fields_out_of_order
    }

    pub(crate) fn validate_fields_have_values(&self) -> bool {
        self.validate_fields_have_values
    }

    pub(crate) fn validate_user_defined_fields(&self) -> bool {
        self.validate_user_defined_fields
    }

    pub(crate) fn validate_length_and_checksum(&self) -> bool {
        self.validate_length_and_checksum
    }

    pub(crate) fn allow_unknown_msg_fields(&self) -> bool {
        self.allow_unknown_msg_fields
    }

    pub(crate) fn check_latency(&self) -> bool {
        self.check_latency
    }

    pub(crate) fn max_latency(&self) -> u32 {
        self.max_latency
    }
}

#[derive(Builder, Clone, Debug)]
pub(crate) struct SessionSetting {
    session_id: SessionId,
    connection: SettingsConnection,
    socket_options: SocketOptions,
    ssl_options: Option<SslOptions>,
    logging: LoggingOptions,
    persistence: Persistence,
    default_appl_ver_id: Option<String>,
    schedule: SessionSchedule,
    validation_options: ValidationOptions,
}

impl SessionSetting {
    pub(crate) fn builder() -> SessionSettingBuilder {
        SessionSettingBuilder::create_empty()
    }

    pub(crate) fn session_id(&self) -> &SessionId {
        &self.session_id
    }

    pub(crate) fn connection(&self) -> &SettingsConnection {
        &self.connection
    }

    pub(crate) fn socket_options(&self) -> &SocketOptions {
        &self.socket_options
    }

    pub(crate) fn ssl_options(&self) -> Option<&SslOptions> {
        self.ssl_options.as_ref()
    }

    pub(crate) fn logging(&self) -> &LoggingOptions {
        &self.logging
    }

    pub(crate) fn persistence(&self) -> &Persistence {
        &self.persistence
    }

    pub(crate) fn default_appl_ver_id(&self) -> Option<&String> {
        self.default_appl_ver_id.as_ref()
    }

    pub(crate) fn schedule(&self) -> &SessionSchedule {
        &self.schedule
    }

    pub(crate) fn validation_options(&self) -> &ValidationOptions {
        &self.validation_options
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SessionSettingOld {
    pub(crate) connection_type: ConnectionType,
    pub(crate) is_dynamic: bool,

    pub(crate) begin_string: String,
    pub(crate) sender_comp_id: String,
    pub(crate) sender_sub_id: Option<String>,
    pub(crate) sender_location_id: Option<String>,
    pub(crate) target_comp_id: String,
    pub(crate) target_sub_id: Option<String>,
    pub(crate) target_location_id: Option<String>,

    pub(crate) session_qualifier: Option<String>,
    pub(crate) default_appl_ver_id: Option<String>,

    pub(crate) non_stop_session: bool,
    pub(crate) use_local_time: bool,
    pub(crate) time_zone: Option<String>,
    pub(crate) start_day: Option<String>,
    pub(crate) end_day: Option<String>,
    pub(crate) start_time: Option<String>,
    pub(crate) end_time: Option<String>,

    pub(crate) milliseconds_in_time_stamp: bool,
    pub(crate) refresh_on_logon: bool,
    pub(crate) reset_on_logon: bool,
    pub(crate) reset_on_logout: bool,
    pub(crate) reset_on_disconnect: bool,
    pub(crate) send_redundant_resend_requests: bool,
    pub(crate) resend_session_level_rejects: bool,
    pub(crate) time_stamp_precision: Option<String>,
    pub(crate) enable_last_msg_seq_num_processed: bool,
    pub(crate) max_messages_in_resend_request: u32,
    pub(crate) send_logout_before_disconnect_from_timeout: bool,
    pub(crate) ignore_poss_dup_resend_requests: bool,
    pub(crate) requires_orig_sending_time: bool,

    // validation options
    pub(crate) use_data_dictionary: bool,
    pub(crate) data_dictionary: Option<String>,
    pub(crate) transport_data_dictionary: Option<String>,
    pub(crate) app_data_dictionary: Option<String>,
    pub(crate) validate_fields_out_of_order: bool,
    pub(crate) validate_fields_have_values: bool,
    pub(crate) validate_user_defined_fields: bool,
    pub(crate) validate_length_and_checksum: bool,
    pub(crate) allow_unknown_msg_fields: bool,
    pub(crate) check_latency: bool,
    pub(crate) max_latency: u32,

    pub(crate) reconnect_interval: u32,
    pub(crate) heart_bt_int: Option<u32>, //initiator only
    pub(crate) logon_timeout: u32,
    pub(crate) logout_timeout: u32,

    // TODO move this into ConnectionType
    // initiator options
    pub(crate) socket_connect_host: Option<String>,
    pub(crate) socket_connect_port: Option<u32>,
    // TODO
    // pub(crate) socket_connect_hosts: Option<String>, // initiator<n> failover
    // pub(crate) socket_connect_ports: Option<String>, // initiator<n> failover

    // acceptor options
    // TODO move this into ConnectionType
    pub(crate) socket_accept_host: Option<String>,
    pub(crate) socket_accept_port: Option<u32>,

    // storage
    pub(crate) persist_messages: bool,
    // store path
    pub(crate) file_store_path: Option<String>,

    // logging
    pub(crate) file_log_path: Option<String>,
    pub(crate) debug_file_log_path: Option<String>,

    // Socket options
    pub(crate) socket_nodelay: bool,
    pub(crate) socket_send_buffer_size: Option<String>,
    pub(crate) socket_receive_buffer_size: Option<String>,
    pub(crate) socket_send_timeout: Option<String>,
    pub(crate) socket_receive_timeout: Option<String>,

    // SSL options
    pub(crate) ssl_enable: bool,
    pub(crate) ssl_server_name: Option<String>,
    pub(crate) ssl_protocols: Option<String>,
    pub(crate) ssl_validate_certificates: Option<String>,
    pub(crate) ssl_check_certificate_revocation: Option<String>,
    pub(crate) ssl_certificate: Option<String>,
    pub(crate) ssl_certificate_password: Option<String>,
    pub(crate) ssl_require_client_certificate: Option<String>,
    pub(crate) ssl_ca_certificate: Option<String>,
}

impl SessionSetting {
    pub(crate) fn score(&self, session_id: &SessionId) -> u16 {
        let mut score = 0;
        score += match self.session_id().sender_comp_id() {
            "*" => 6,
            value if value == session_id.sender_comp_id() => 7,
            _ => 0,
        };
        score += match self.session_id().sender_sub_id() {
            value if value == session_id.sender_sub_id() => 1,
            _ => 0,
        };
        score += match self.session_id().sender_location_id() {
            value if value == session_id.sender_location_id() => 1,
            _ => 0,
        };
        score += match self.session_id().target_comp_id() {
            "*" => 6,
            value if value == session_id.target_comp_id() => 7,
            _ => 0,
        };
        score += match self.session_id().target_sub_id() {
            value if value == session_id.target_sub_id() => 1,
            _ => 0,
        };
        score += match self.session_id().target_location_id() {
            value if value == session_id.target_location_id() => 1,
            _ => 0,
        };
        if score < 12 {
            0
        } else {
            score
        }
    }

    pub(crate) fn is_dynamic(&self) -> bool {
        match self.connection {
            SettingsConnection::Acceptor { is_dynamic, .. } => {
                is_dynamic
                    && (self.session_id.sender_comp_id() == "*"
                        || self.session_id.target_comp_id() == "*")
            }
            SettingsConnection::Initiator { .. } => false,
        }
    }

    pub(crate) fn socket_settings(&self) -> SocketSettings {
        SocketSettings::new(self.connection.socket_addr().clone(), self.socket_options.clone(), self.ssl_options.clone())
    }

    pub(crate) fn reconnect_interval(&self) -> Option<u32> {
        match self.connection {
            SettingsConnection::Acceptor { .. } => None,
            SettingsConnection::Initiator { reconnect_interval, .. } => Some(reconnect_interval),
        }
    }

    pub(crate) fn accepts(&self, session_id: &SessionId) -> bool {
        if self.is_dynamic() {
            let sender_comp_ok = match self.session_id.sender_comp_id() {
                "*" => true,
                s => s == session_id.sender_comp_id(),
            };
            let target_comp_ok = match self.session_id.target_comp_id() {
                "*" => true,
                s => s == session_id.target_comp_id(),
            };
            sender_comp_ok && target_comp_ok
        } else {
            self.session_id() == session_id
        }
    }
}
