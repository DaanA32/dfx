use std::{fs::File, io::{BufReader, BufRead}, collections::BTreeMap, net::SocketAddr};
use crate::connection::SocketSettings;
use super::{SessionId, Session, Application};

mod builder;
use builder::*;
mod setting;
pub use setting::*;

#[derive(Debug)]
pub enum SettingOption {
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
    InvalidValue { setting: SettingOption, value: String },
    LineParseError { line: String, reason: String, line_number: usize },
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

        let mut n = 0;
        for line in string.lines() {
            n+=1;
            //Comment
            if line.trim().starts_with('#') {
                continue
            }
            if !default_started && !default_ended && line.trim().trim_matches(delims).eq_ignore_ascii_case("default") {
                last_setting = Some(SessionSettingBuilder::default());
                default_started = true;
            } else if default_started && line.trim().trim_matches(delims).eq_ignore_ascii_case("default") {
                return Err(SessionSettingsError::DefaultSectionAlreadyDefined);
            } else if default_started && !default_ended && line.trim().trim_matches(delims).eq_ignore_ascii_case("session") {
                default = last_setting.replace(SessionSettingBuilder::default());
                default_ended = true;
            } else if default_started && default_ended && line.trim().trim_matches(delims).eq_ignore_ascii_case("session") {
                if let Some(mut value) = last_setting.replace(SessionSettingBuilder::default()) {
                    if let Some(default) = default.as_ref() {
                        settings.push(value.merge(default).validate()?.build());
                    } else {
                        settings.push(value.validate()?.build());
                    }
                }
            } else if let Some(setting) = last_setting.as_mut() {
                setting.set_from_line(n, line.trim())?;
            }
        }

        if default_started && default_ended {
            if let Some(mut value) = last_setting {
                if let Some(default) = default.as_ref() {
                    settings.push(value.merge(default).validate()?.build());
                } else {
                    settings.push(value.validate()?.build());
                }
            }
        }

        match (default, settings) {
            (None, _) => Err(SessionSettingsError::NoDefaultSection),
            (Some(default), v) => Ok(Self {
                // default,
                sessions: v
            })
        }
    }

    // pub(crate) fn default_settings(&self) -> &SessionSettingBuilder {
    //     &self.default
    // }

    pub(crate) fn for_session_id(&self, session_id: &SessionId) -> Option<&SessionSetting> {
        let best_match = self.sessions.iter()
                .map(|s| (s.score(session_id), s))
                .filter(|(score, _)| score > &0)
                .max_by(|(k1, _), (k2, _)| k1.cmp(k2))
                .map(|(k,v )| v);
        best_match
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
ConnectionType=acceptor
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

        assert_eq!(&settings.sessions[0].begin_string, "TEST");

        assert_eq!(&settings.sessions[0].sender_comp_id, "sender");

        assert_eq!(&settings.sessions[0].target_comp_id, "target1");
    }

    #[test]
    fn settings_test_two_sessions() {
        let data = r#"# Comment
[DEFAULT]
ConnectionType=acceptor
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

        assert_eq!(settings.sessions[0].begin_string, "TEST");
        assert_eq!(settings.sessions[1].begin_string, "TEST");

        assert_eq!(settings.sessions[0].sender_comp_id, "sender");
        assert_eq!(settings.sessions[1].sender_comp_id, "sender");

        assert_eq!(settings.sessions[0].target_comp_id, "target1");
        assert_eq!(settings.sessions[1].target_comp_id, "target2");

        let session_id = SessionId::new("", "sender", "", "", "target1", "", "");
        assert_eq!(Some(&settings.sessions[0].target_comp_id), settings.for_session_id(&session_id).map(|s| &s.target_comp_id));
        let session_id = SessionId::new("", "sender", "", "", "target2", "", "");
        assert_eq!(Some(&settings.sessions[1].target_comp_id), settings.for_session_id(&session_id).map(|s| &s.target_comp_id));
        let session_id = SessionId::new("", "sender", "", "", "target3", "", "");
        assert_eq!(None, settings.for_session_id(&session_id));
        let session_id = SessionId::new("", "sender_any", "", "", "target_any_1", "", "");
        assert_eq!(Some(&settings.sessions[2].target_comp_id), settings.for_session_id(&session_id).map(|s| &s.target_comp_id));
        let session_id = SessionId::new("", "sender_any", "", "", "target_any_2", "", "");
        assert_eq!(Some(&settings.sessions[2].target_comp_id), settings.for_session_id(&session_id).map(|s| &s.target_comp_id));
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
