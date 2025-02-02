use std::{
    collections::BTreeMap,
    net::SocketAddr,
};

mod builder;
use builder::*;
mod setting;
pub(crate) use setting::*;

use dfx_base::session_id::SessionId;

#[derive(Debug)]
pub(crate) enum SettingOption {
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
    // TODO add feature ssl
    SSLEnable,
    SSLServerName,
    SSLMinProtocol,
    SSLMaxProtocol,
    SSLUseSNI,
    SSLAcceptInvalidCerts,
    SSLAcceptInvalidHostnames,
    SSLDisableBuiltInRoots,
    // SSLValidateCertificates,
    // SSLCheckCertificateRevocation,
    SSLCertificate,
    SSLCertificatePassword,
    SSLRequireClientCertificate,
    SSLCACertificate,
}

#[derive(Debug, Default, Clone)]
pub struct SessionSettings {
    // default: SessionSettingBuilder,
    sessions: Vec<SessionSetting>,
}

#[derive(Debug)]
pub enum SessionSettingsError {
    NoSuchSetting(String),
    IoError(std::io::Error),
    NoDefaultSection,
    DefaultSectionAlreadyDefined,
    ValidationErrors(Vec<String>),
    InvalidValue {
        setting: String,
        value: String,
    },
    LineParseError {
        line: String,
        reason: String,
        line_number: usize,
    },
}

impl TryFrom<&str> for SettingOption {
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
            "SendLogoutBeforeDisconnectFromTimeout" => {
                Ok(Self::SendLogoutBeforeDisconnectFromTimeout)
            }
            "SocketNodelay" => Ok(Self::SocketNodelay),
            "SocketSendBufferSize" => Ok(Self::SocketSendBufferSize),
            "SocketReceiveBufferSize" => Ok(Self::SocketReceiveBufferSize),
            "SocketSendTimeout" => Ok(Self::SocketSendTimeout),
            "SocketReceiveTimeout" => Ok(Self::SocketReceiveTimeout),
            "IgnorePossDupResendRequests" => Ok(Self::IgnorePossDupResendRequests),
            "RequiresOrigSendingTime" => Ok(Self::RequiresOrigSendingTime),
            "CheckLatency" => Ok(Self::CheckLatency),
            "MaxLatency" => Ok(Self::MaxLatency),
            // TODO add feature ssl
            "SSLEnable" => Ok(Self::SSLEnable),
            "SSLServerName" => Ok(Self::SSLServerName),
            "SSLMinProtocol" => Ok(Self::SSLMinProtocol),
            "SSLMaxProtocol" => Ok(Self::SSLMaxProtocol),
            "SSLUseSNI" => Ok(Self::SSLUseSNI),
            "SSLAcceptInvalidCerts" => Ok(Self::SSLAcceptInvalidCerts),
            "SSLAcceptInvalidHostnames" => Ok(Self::SSLAcceptInvalidHostnames),
            "SSLDisableBuiltInRoots" => Ok(Self::SSLDisableBuiltInRoots),
            // "SSLValidateCertificates" => Ok(Self::SSLValidateCertificates),
            // "SSLCheckCertificateRevocation" => Ok(Self::SSLCheckCertificateRevocation),
            "SSLCertificate" => Ok(Self::SSLCertificate),
            "SSLCertificatePassword" => Ok(Self::SSLCertificatePassword),
            "SSLRequireClientCertificate" => Ok(Self::SSLRequireClientCertificate),
            "SSLCACertificate" => Ok(Self::SSLCACertificate),
            _ => Err(Self::Error::NoSuchSetting(value.into())),
        }
    }
}

impl From<SettingOption> for String {
    fn from(val: SettingOption) -> Self {
        let ref_str: &'static str = val.into();
        ref_str.into()
    }
}

impl From<SettingOption> for &'static str {

    fn from(val: SettingOption) -> Self {
        match val {
            SettingOption::IsDynamic => "IsDynamic",
            SettingOption::BeginString => "BeginString",
            SettingOption::SenderCompID => "SenderCompID",
            SettingOption::SenderSubID => "SenderSubID",
            SettingOption::SenderLocationID => "SenderLocationID",
            SettingOption::TargetCompID => "TargetCompID",
            SettingOption::TargetSubID => "TargetSubID",
            SettingOption::TargetLocationID => "TargetLocationID",
            SettingOption::SessionQualifier => "SessionQualifier",
            SettingOption::DefaultApplVerID => "DefaultApplVerID",
            SettingOption::ConnectionType => "ConnectionType",
            SettingOption::UseDataDictionary => "UseDataDictionary",
            SettingOption::NonStopSession => "NonStopSession",
            SettingOption::UseLocalTime => "UseLocalTime",
            SettingOption::TimeZone => "TimeZone",
            SettingOption::StartDay => "StartDay",
            SettingOption::EndDay => "EndDay",
            SettingOption::StartTime => "StartTime",
            SettingOption::EndTime => "EndTime",
            SettingOption::HeartBtInt => "HeartBtInt",
            SettingOption::SocketAcceptHost => "SocketAcceptHost",
            SettingOption::SocketAcceptPort => "SocketAcceptPort",
            SettingOption::SocketConnectHost => "SocketConnectHost",
            SettingOption::SocketConnectPort => "SocketConnectPort",
            SettingOption::ReconnectInterval => "ReconnectInterval",
            SettingOption::FileLogPath => "FileLogPath",
            SettingOption::DebugFileLogPath => "DebugFileLogPath",
            SettingOption::FileStorePath => "FileStorePath",
            SettingOption::RefreshOnLogon => "RefreshOnLogon",
            SettingOption::ResetOnLogon => "ResetOnLogon",
            SettingOption::ResetOnLogout => "ResetOnLogout",
            SettingOption::ResetOnDisconnect => "ResetOnDisconnect",
            SettingOption::ValidateFieldsOutOfOrder => "ValidateFieldsOutOfOrder",
            SettingOption::ValidateFieldsHaveValues => "ValidateFieldsHaveValues",
            SettingOption::ValidateUserDefinedFields => "ValidateUserDefinedFields",
            SettingOption::ValidateLengthAndChecksum => "ValidateLengthAndChecksum",
            SettingOption::AllowUnknownMsgFields => "AllowUnknownMsgFields",
            SettingOption::DataDictionary => "DataDictionary",
            SettingOption::TransportDataDictionary => "TransportDataDictionary",
            SettingOption::AppDataDictionary => "AppDataDictionary",
            SettingOption::PersistMessages => "PersistMessages",
            SettingOption::LogonTimeout => "LogonTimeout",
            SettingOption::LogoutTimeout => "LogoutTimeout",
            SettingOption::SendRedundantResendRequests => "SendRedundantResendRequests",
            SettingOption::ResendSessionLevelRejects => "ResendSessionLevelRejects",
            SettingOption::MillisecondsInTimeStamp => "MillisecondsInTimeStamp",
            SettingOption::TimeStampPrecision => "TimeStampPrecision",
            SettingOption::EnableLastMsgSeqNumProcessed => "EnableLastMsgSeqNumProcessed",
            SettingOption::MaxMessagesInResendRequest => "MaxMessagesInResendRequest",
            SettingOption::SendLogoutBeforeDisconnectFromTimeout => "SendLogoutBeforeDisconnectFromTimeout",
            SettingOption::SocketNodelay => "SocketNodelay",
            SettingOption::SocketSendBufferSize => "SocketSendBufferSize",
            SettingOption::SocketReceiveBufferSize => "SocketReceiveBufferSize",
            SettingOption::SocketSendTimeout => "SocketSendTimeout",
            SettingOption::SocketReceiveTimeout => "SocketReceiveTimeout",
            SettingOption::IgnorePossDupResendRequests => "IgnorePossDupResendRequests",
            SettingOption::RequiresOrigSendingTime => "RequiresOrigSendingTime",
            SettingOption::CheckLatency => "CheckLatency",
            SettingOption::MaxLatency => "MaxLatency",
            // TODO add feature ssl
            SettingOption::SSLEnable => "SSLEnable",
            SettingOption::SSLServerName => "SSLServerName",
            SettingOption::SSLMinProtocol => "SSLMinProtocol",
            SettingOption::SSLMaxProtocol => "SSLMaxProtocol",
            SettingOption::SSLUseSNI => "SSLUseSNI",
            SettingOption::SSLAcceptInvalidCerts => "SSLAcceptInvalidCerts",
            SettingOption::SSLAcceptInvalidHostnames => "SSLAcceptInvalidHostnames",
            SettingOption::SSLDisableBuiltInRoots => "SSLDisableBuiltInRoots",
            // Self::SSLValidateCertificates => "SSLValidateCertificates",
            // Self::SSLCheckCertificateRevocation => "SSLCheckCertificateRevocation",
            SettingOption::SSLCertificate => "SSLCertificate",
            SettingOption::SSLCertificatePassword => "SSLCertificatePassword",
            SettingOption::SSLRequireClientCertificate => "SSLRequireClientCertificate",
            SettingOption::SSLCACertificate => "SSLCACertificate",
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
        let delims: &[_] = &['[', ']'];

        let mut default_started = false;
        let mut default_ended = false;
        let mut last_setting = None;

        let mut default = None;
        let mut settings = Vec::new();

        let mut n = 0;
        for line in string.lines() {
            n += 1;
            //Comment
            // println!("Read Line: {}", line.trim());
            if line.trim().starts_with('#') {
                continue;
            }
            if !default_started
                && !default_ended
                && line
                    .trim()
                    .trim_matches(delims)
                    .eq_ignore_ascii_case("default")
            {
                last_setting = Some(DynamicSessionSettingBuilder::default());
                default_started = true;
            } else if default_started
                && line
                    .trim()
                    .trim_matches(delims)
                    .eq_ignore_ascii_case("default")
            {
                return Err(SessionSettingsError::DefaultSectionAlreadyDefined);
            } else if default_started
                && !default_ended
                && line
                    .trim()
                    .trim_matches(delims)
                    .eq_ignore_ascii_case("session")
            {
                default = last_setting.replace(DynamicSessionSettingBuilder::default());
                default_ended = true;
            } else if default_started
                && default_ended
                && line
                    .trim()
                    .trim_matches(delims)
                    .eq_ignore_ascii_case("session")
            {
                if let Some(value) =
                    last_setting.replace(DynamicSessionSettingBuilder::default())
                {
                    if let Some(default) = default.as_ref() {
                        settings.push(value.merge(default).validate()?.build()?);
                    } else {
                        settings.push(value.validate()?.build()?);
                    }
                }
            } else if let Some(setting) = last_setting.as_mut() {
                setting.set_from_line(n, line.trim())?;
            }
        }

        if default_started && default_ended {
            if let Some(value) = last_setting {
                if let Some(default) = default.as_ref() {
                    settings.push(value.merge(default).validate()?.build()?);
                } else {
                    settings.push(value.validate()?.build()?);
                }
            }
        }

        match (default, settings) {
            (None, _) => Err(SessionSettingsError::NoDefaultSection),
            (Some(_default), v) => Ok(Self {
                // default,
                sessions: v,
            }),
        }
    }

    // pub(crate) fn default_settings(&self) -> &SessionSettingBuilder {
    //     &self.default
    // }

    pub(crate) fn for_session_id(&self, session_id: &SessionId) -> Option<&SessionSetting> {
        let best_match = self
            .sessions
            .iter()
            .map(|s| (s.score(session_id), s))
            .filter(|(score, _)| score > &0)
            .max_by(|(k1, _), (k2, _)| k1.cmp(k2))
            .map(|(_, v)| v);
        best_match
    }

    pub(crate) fn sessions(&self) -> &Vec<SessionSetting> {
        self.sessions.as_ref()
    }

    pub(crate) fn sessions_by_address(&self) -> BTreeMap<SocketAddr, Vec<SessionSetting>> {
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
    use dfx_base::session_id::SessionId;
    use crate::session::SessionSettingsError;

    use super::SessionSettings;

    #[test]
    fn settings_test_one_sessions() {
        let data = r#"# Comment
[DEFAULT]
ConnectionType=acceptor
BeginString=TEST
SenderCompID=sender
SocketAcceptHost=127.0.0.1
SocketAcceptPort=5000
[SESSION]
TargetCompID=target1
"#;
        let settings = SessionSettings::from_string(data);
        println!("{:?}", settings);
        assert!(settings.is_ok());
        let settings = settings.unwrap();
        assert!(settings.sessions.len() == 1);

        assert_eq!(settings.sessions[0].session_id().begin_string(), "TEST");

        assert_eq!(settings.sessions[0].session_id().sender_comp_id(), "sender");

        assert_eq!(settings.sessions[0].session_id().target_comp_id(), "target1");
    }

    #[test]
    fn settings_test_two_sessions() {
        let data = r#"# Comment
[DEFAULT]
ConnectionType=acceptor
BeginString=TEST
SenderCompID=sender
SocketAcceptHost=127.0.0.1
SocketAcceptPort=5000

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

        assert_eq!(settings.sessions[0].session_id().begin_string(), "TEST");
        assert_eq!(settings.sessions[1].session_id().begin_string(), "TEST");

        assert_eq!(settings.sessions[0].session_id().sender_comp_id(), "sender");
        assert_eq!(settings.sessions[1].session_id().sender_comp_id(), "sender");

        assert_eq!(settings.sessions[0].session_id().target_comp_id(), "target1");
        assert_eq!(settings.sessions[1].session_id().target_comp_id(), "target2");

        let session_id = SessionId::new("", "sender", "", "", "target1", "", "");
        assert_eq!(
            Some(settings.sessions[0].session_id().target_comp_id()),
            settings
                .for_session_id(&session_id)
                .map(|s| s.session_id().target_comp_id())
        );
        let session_id = SessionId::new("", "sender", "", "", "target2", "", "");
        assert_eq!(
            Some(settings.sessions[1].session_id().target_comp_id()),
            settings
                .for_session_id(&session_id)
                .map(|s| s.session_id().target_comp_id())
        );
        let session_id = SessionId::new("", "sender", "", "", "target3", "", "");
        //assert_eq!(None, settings.for_session_id(&session_id));
        let session_id = SessionId::new("", "sender_any", "", "", "target_any_1", "", "");
        assert_eq!(
            Some(settings.sessions[2].session_id().target_comp_id()),
            settings
                .for_session_id(&session_id)
                .map(|s| s.session_id().target_comp_id())
        );
        let session_id = SessionId::new("", "sender_any", "", "", "target_any_2", "", "");
        assert_eq!(
            Some(settings.sessions[2].session_id().target_comp_id()),
            settings
                .for_session_id(&session_id)
                .map(|s| s.session_id().target_comp_id())
        );
    }

    #[test]
    fn settings_test_no_default() {
        let data = r#"# Comment
"#;
        let settings = SessionSettings::from_string(data);
        assert!(matches!(
            settings,
            Err(SessionSettingsError::NoDefaultSection)
        ));
    }

    #[test]
    fn settings_test_double_default() {
        let data = r#"# Comment
[DEFAULT]
[SESSION]
[DEFAULT]
"#;
        let settings = SessionSettings::from_string(data);
        assert!(matches!(
            settings,
            Err(SessionSettingsError::DefaultSectionAlreadyDefined)
        ));
    }

    #[test]
    fn settings_test_double_default_alt() {
        let data = r#"# Comment
[DEFAULT]
[DEFAULT]
[SESSION]
"#;
        let settings = SessionSettings::from_string(data);
        assert!(matches!(
            settings,
            Err(SessionSettingsError::DefaultSectionAlreadyDefined)
        ));
    }

    #[test]
    fn settings_test_invalid_setting() {
        let data = r#"# Comment
[DEFAULT]
asdfasd=Y
"#;
        let settings = SessionSettings::from_string(data);
        assert!(matches!(
            settings,
            Err(SessionSettingsError::NoSuchSetting(_))
        ));
    }
}
