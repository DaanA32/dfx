#![allow(dead_code)]
#![allow(unused)]

use dfx_testing::runner;
mod common;
use common::TestApplication;

use paste::paste;
use rusty_fork::rusty_fork_test;

fn cfg_from_version(version: &str) -> &str {
    match version {
        "fix40" => include_str!("./cfg/at_40.cfg"),
        "fix41" => include_str!("cfg/at_41.cfg"),
        "fix42" => include_str!("cfg/at_42.cfg"),
        "fix43" => include_str!("cfg/at_43.cfg"),
        "fix44" => include_str!("cfg/at_44.cfg"),
        "fix44noreset" => include_str!("cfg/at_44_noreset.cfg"),
        "fix50" => include_str!("cfg/at_50.cfg"),
        "fix50sp1" => include_str!("cfg/at_50_sp1.cfg"),
        "fix50sp2" => include_str!("cfg/at_50_sp2.cfg"),
        "fix50sp2" => include_str!("cfg/at_50_sp2.cfg"),
        "misc" => include_str!("cfg/at_42_misc.cfg"),
        "future" => include_str!("cfg/at_42.cfg"),
        v => panic!("cfg from version {v}"),
    }
}

fn path_from_version(version: &str) -> &str {
    match version {
        "fix40" => "tests/definitions/server-ext/fix40/",
        "fix41" => "tests/definitions/server-ext/fix41/",
        "fix42" => "tests/definitions/server-ext/fix42/",
        "fix43" => "tests/definitions/server-ext/fix43/",
        "fix44" => "tests/definitions/server-ext/fix44/",
        "fix44noreset" => "tests/definitions/server-ext/fix44noreset/",
        "fix50" => "tests/definitions/server-ext/fix50/",
        "fix50sp1" => "tests/definitions/server-ext/fix50sp1/",
        "fix50sp2" => "tests/definitions/server-ext/fix50sp2/",
        "misc" => "tests/definitions/server-ext/misc/",
        "future" => "tests/definitions/server-ext/future/",
        v => panic!("path from version {v}"),
    }
}

macro_rules! imports {
    () => {
        use std::{
            net::SocketAddr,
            time::{Duration, Instant}, path::Path, io::{BufReader, BufRead}, fs::File, borrow::BorrowMut, env,
        };

        use dfx::{
            connection::SocketAcceptor,
            session::{Session, SessionSettings}, message_store::DefaultStoreFactory, data_dictionary_provider::DefaultDataDictionaryProvider, logging::PrintlnLogFactory, message::DefaultMessageFactory,
        };
        use super::*;
    };
}

macro_rules! acceptor {
    ($cfg:ident -> $func:expr) => {
        paste! {
            rusty_fork_test!{
                #[test]
                fn [<test_ $func:snake>]() {
                    let app = TestApplication::new();
                    let cfg = cfg_from_version(stringify!($cfg));
                    let session_settings = SessionSettings::from_string(cfg).unwrap();
                    let mut acceptor = SocketAcceptor::new(
                        session_settings,
                        app,
                        DefaultStoreFactory::new(),
                        DefaultDataDictionaryProvider::new(),
                        PrintlnLogFactory::new(),
                        DefaultMessageFactory::new(),
                    );

                    println!("{:?}", std::env::current_dir());
                    let path = path_from_version(stringify!($cfg));
                    let steps = runner::steps(format!("{path}{}.def", $func).as_str());
                    acceptor.start();

                    while acceptor.endpoints().len() == 0 {
                        std::thread::sleep(Duration::from_millis(10));
                    }

                    let endpoint = acceptor.endpoints()[0];

                    let runner_thread = runner::create_thread(steps, endpoint.port().into(), path);
                    let start = Instant::now();
                    while !runner_thread.is_finished() {
                        if Instant::now() - start > Duration::from_secs(120) {
                            println!("ERROR: Timeout: {runner_thread:?}");
                            break;
                        }
                        std::thread::sleep(Duration::from_millis(10));
                    }
                    acceptor.stop();

                    if runner_thread.is_finished() {
                        match runner_thread.join() {
                            Ok(result) => match result {
                                Ok(()) => {},
                                Err(message) => panic!("Steps failed:\n{message}\n")
                            },
                            Err(_) => eprintln!("Failed to join thread."),
                        }
                    } else {
                        eprintln!("Runner did not finish in 120s {}::{}", stringify!($cfg), $func);
                    }
                }
            }
        }
    };
}

mod fix40 {
    imports!();
    acceptor!(fix40 -> "10_MsgSeqNumEqual");
    acceptor!(fix40 -> "10_MsgSeqNumGreater");
    acceptor!(fix40 -> "10_MsgSeqNumLess");
    acceptor!(fix40 -> "11a_NewSeqNoGreater");
    acceptor!(fix40 -> "11b_NewSeqNoEqual");
    acceptor!(fix40 -> "11c_NewSeqNoLess");
    acceptor!(fix40 -> "13b_UnsolicitedLogoutMessage");
    acceptor!(fix40 -> "14a_BadField");
    acceptor!(fix40 -> "14b_RequiredFieldMissing");
    acceptor!(fix40 -> "14c_TagNotDefinedForMsgType");
    acceptor!(fix40 -> "14d_TagSpecifiedWithoutValue");
    acceptor!(fix40 -> "14e_IncorrectEnumValue");
    acceptor!(fix40 -> "14f_IncorrectDataFormat");
    acceptor!(fix40 -> "14g_HeaderBodyTrailerFieldsOutOfOrder");
    acceptor!(fix40 -> "14h_RepeatedTag");
    acceptor!(fix40 -> "15_HeaderAndBodyFieldsOrderedDifferently");
    acceptor!(fix40 -> "19a_PossResendMessageThatHAsAlreadyBeenSent");
    acceptor!(fix40 -> "19b_PossResendMessageThatHasNotBeenSent");
    acceptor!(fix40 -> "1a_ValidLogonMsgSeqNumTooHigh");
    acceptor!(fix40 -> "1a_ValidLogonWithCorrectMsgSeqNum");
    acceptor!(fix40 -> "1b_DuplicateIdentity");
    acceptor!(fix40 -> "1c_InvalidSenderCompID");
    acceptor!(fix40 -> "1c_InvalidTargetCompID");
    acceptor!(fix40 -> "1d_InvalidLogonBadSendingTime");
    acceptor!(fix40 -> "1d_InvalidLogonLengthInvalid");
    acceptor!(fix40 -> "1d_InvalidLogonWrongBeginString");
    acceptor!(fix40 -> "1e_NotLogonMessage");
    acceptor!(fix40 -> "20_SimultaneousResendRequest");
    acceptor!(fix40 -> "2a_MsgSeqNumCorrect");
    acceptor!(fix40 -> "2b_MsgSeqNumTooHigh");
    acceptor!(fix40 -> "2c_MsgSeqNumTooLow");
    acceptor!(fix40 -> "2d_GarbledMessage");
    acceptor!(fix40 -> "2e_PossDupAlreadyReceived");
    acceptor!(fix40 -> "2e_PossDupNotReceived");
    acceptor!(fix40 -> "2f_PossDupOrigSendingTimeTooHigh");
    acceptor!(fix40 -> "2g_PossDupNoOrigSendingTime");
    acceptor!(fix40 -> "2i_BeginStringValueUnexpected");
    acceptor!(fix40 -> "2k_CompIDDoesNotMatchProfile");
    acceptor!(fix40 -> "2m_BodyLengthValueNotCorrect");
    acceptor!(fix40 -> "2o_SendingTimeValueOutOfRange");
    acceptor!(fix40 -> "2q_MsgTypeNotValid");
    acceptor!(fix40 -> "2r_UnregisteredMsgType");
    acceptor!(fix40 -> "2t_FirstThreeFieldsOutOfOrder");
    acceptor!(fix40 -> "3b_InvalidChecksum");
    acceptor!(fix40 -> "3c_GarbledMessage");
    acceptor!(fix40 -> "4a_NoDataSentDuringHeartBtInt");
    acceptor!(fix40 -> "4b_ReceivedTestRequest");
    acceptor!(fix40 -> "6_SendTestRequest");
    acceptor!(fix40 -> "7_ReceiveRejectMessage");
    acceptor!(fix40 -> "8_AdminAndApplicationMessages");
    acceptor!(fix40 -> "8_OnlyAdminMessages");
    acceptor!(fix40 -> "8_OnlyApplicationMessages");
    acceptor!(fix40 -> "AlreadyLoggedOn");
    acceptor!(fix40 -> "RejectResentMessage");
    acceptor!(fix40 -> "ReverseRoute");
    acceptor!(fix40 -> "ReverseRouteWithEmptyRoutingTags");
}

mod fix41 {
    imports!();
    acceptor!(fix41 -> "10_MsgSeqNumEqual");
    acceptor!(fix41 -> "10_MsgSeqNumGreater");
    acceptor!(fix41 -> "10_MsgSeqNumLess");
    acceptor!(fix41 -> "11a_NewSeqNoGreater");
    acceptor!(fix41 -> "11b_NewSeqNoEqual");
    acceptor!(fix41 -> "11c_NewSeqNoLess");
    acceptor!(fix41 -> "13b_UnsolicitedLogoutMessage");
    acceptor!(fix41 -> "14a_BadField");
    acceptor!(fix41 -> "14b_RequiredFieldMissing");
    acceptor!(fix41 -> "14c_TagNotDefinedForMsgType");
    acceptor!(fix41 -> "14d_TagSpecifiedWithoutValue");
    acceptor!(fix41 -> "14e_IncorrectEnumValue");
    acceptor!(fix41 -> "14f_IncorrectDataFormat");
    acceptor!(fix41 -> "14g_HeaderBodyTrailerFieldsOutOfOrder");
    acceptor!(fix41 -> "14h_RepeatedTag");
    acceptor!(fix41 -> "15_HeaderAndBodyFieldsOrderedDifferently");
    acceptor!(fix41 -> "19a_PossResendMessageThatHAsAlreadyBeenSent");
    acceptor!(fix41 -> "19b_PossResendMessageThatHasNotBeenSent");
    acceptor!(fix41 -> "1a_ValidLogonMsgSeqNumTooHigh");
    acceptor!(fix41 -> "1a_ValidLogonWithCorrectMsgSeqNum");
    acceptor!(fix41 -> "1b_DuplicateIdentity");
    acceptor!(fix41 -> "1c_InvalidSenderCompID");
    acceptor!(fix41 -> "1c_InvalidTargetCompID");
    acceptor!(fix41 -> "1d_InvalidLogonBadSendingTime");
    acceptor!(fix41 -> "1d_InvalidLogonLengthInvalid");
    acceptor!(fix41 -> "1d_InvalidLogonWrongBeginString");
    acceptor!(fix41 -> "1e_NotLogonMessage");
    acceptor!(fix41 -> "20_SimultaneousResendRequest");
    acceptor!(fix41 -> "2a_MsgSeqNumCorrect");
    acceptor!(fix41 -> "2b_MsgSeqNumTooHigh");
    acceptor!(fix41 -> "2c_MsgSeqNumTooLow");
    acceptor!(fix41 -> "2d_GarbledMessage");
    acceptor!(fix41 -> "2e_PossDupAlreadyReceived");
    acceptor!(fix41 -> "2e_PossDupNotReceived");
    acceptor!(fix41 -> "2f_PossDupOrigSendingTimeTooHigh");
    acceptor!(fix41 -> "2g_PossDupNoOrigSendingTime");
    acceptor!(fix41 -> "2i_BeginStringValueUnexpected");
    acceptor!(fix41 -> "2k_CompIDDoesNotMatchProfile");
    acceptor!(fix41 -> "2m_BodyLengthValueNotCorrect");
    acceptor!(fix41 -> "2o_SendingTimeValueOutOfRange");
    acceptor!(fix41 -> "2q_MsgTypeNotValid");
    acceptor!(fix41 -> "2r_UnregisteredMsgType");
    acceptor!(fix41 -> "2t_FirstThreeFieldsOutOfOrder");
    acceptor!(fix41 -> "3b_InvalidChecksum");
    acceptor!(fix41 -> "3c_GarbledMessage");
    acceptor!(fix41 -> "4a_NoDataSentDuringHeartBtInt");
    acceptor!(fix41 -> "4b_ReceivedTestRequest");
    acceptor!(fix41 -> "6_SendTestRequest");
    acceptor!(fix41 -> "7_ReceiveRejectMessage");
    acceptor!(fix41 -> "8_AdminAndApplicationMessages");
    acceptor!(fix41 -> "8_OnlyAdminMessages");
    acceptor!(fix41 -> "8_OnlyApplicationMessages");
    acceptor!(fix41 -> "AlreadyLoggedOn");
    acceptor!(fix41 -> "RejectResentMessage");
    acceptor!(fix41 -> "ReverseRoute");
    acceptor!(fix41 -> "ReverseRouteWithEmptyRoutingTags");
}

mod fix42 {
    imports!();
    acceptor!(fix42 -> "10_MsgSeqNumEqual");
    acceptor!(fix42 -> "10_MsgSeqNumGreater");
    acceptor!(fix42 -> "10_MsgSeqNumLess");
    acceptor!(fix42 -> "11a_NewSeqNoGreater");
    acceptor!(fix42 -> "11b_NewSeqNoEqual");
    acceptor!(fix42 -> "11c_NewSeqNoLess");
    acceptor!(fix42 -> "13b_UnsolicitedLogoutMessage");
    acceptor!(fix42 -> "14a_BadField");
    acceptor!(fix42 -> "14b_RequiredFieldMissing");
    acceptor!(fix42 -> "14c_TagNotDefinedForMsgType");
    acceptor!(fix42 -> "14d_TagSpecifiedWithoutValue");
    acceptor!(fix42 -> "14e_IncorrectEnumValue");
    acceptor!(fix42 -> "14f_IncorrectDataFormat");
    acceptor!(fix42 -> "14g_HeaderBodyTrailerFieldsOutOfOrder");
    acceptor!(fix42 -> "14h_RepeatedTag");
    acceptor!(fix42 -> "14i_RepeatingGroupCountNotEqual");
    acceptor!(fix42 -> "15_HeaderAndBodyFieldsOrderedDifferently");
    acceptor!(fix42 -> "19a_PossResendMessageThatHAsAlreadyBeenSent");
    acceptor!(fix42 -> "19b_PossResendMessageThatHasNotBeenSent");
    acceptor!(fix42 -> "1a_ValidLogonMsgSeqNumTooHigh");
    acceptor!(fix42 -> "1a_ValidLogonWithCorrectMsgSeqNum");
    acceptor!(fix42 -> "1b_DuplicateIdentity");
    acceptor!(fix42 -> "1c_InvalidSenderCompID");
    acceptor!(fix42 -> "1c_InvalidTargetCompID");
    acceptor!(fix42 -> "1d_InvalidLogonBadSendingTime");
    acceptor!(fix42 -> "1d_InvalidLogonLengthInvalid");
    acceptor!(fix42 -> "1d_InvalidLogonWrongBeginString");
    acceptor!(fix42 -> "1e_NotLogonMessage");
    acceptor!(fix42 -> "20_SimultaneousResendRequest");
    acceptor!(fix42 -> "21_RepeatingGroupSpecifierWithValueOfZero");
    acceptor!(fix42 -> "2a_MsgSeqNumCorrect");
    acceptor!(fix42 -> "2b_MsgSeqNumTooHigh");
    acceptor!(fix42 -> "2c_MsgSeqNumTooLow");
    acceptor!(fix42 -> "2d_GarbledMessage");
    acceptor!(fix42 -> "2e_PossDupAlreadyReceived");
    acceptor!(fix42 -> "2e_PossDupNotReceived");
    acceptor!(fix42 -> "2f_PossDupOrigSendingTimeTooHigh");
    acceptor!(fix42 -> "2g_PossDupNoOrigSendingTime");
    acceptor!(fix42 -> "2i_BeginStringValueUnexpected");
    acceptor!(fix42 -> "2k_CompIDDoesNotMatchProfile");
    acceptor!(fix42 -> "2m_BodyLengthValueNotCorrect");
    acceptor!(fix42 -> "2o_SendingTimeValueOutOfRange");
    acceptor!(fix42 -> "2q_MsgTypeNotValid");
    acceptor!(fix42 -> "2r_UnregisteredMsgType");
    acceptor!(fix42 -> "2t_FirstThreeFieldsOutOfOrder");
    acceptor!(fix42 -> "3b_InvalidChecksum");
    acceptor!(fix42 -> "3c_GarbledMessage");
    acceptor!(fix42 -> "4a_NoDataSentDuringHeartBtInt");
    acceptor!(fix42 -> "4b_ReceivedTestRequest");
    acceptor!(fix42 -> "6_SendTestRequest");
    acceptor!(fix42 -> "7_ReceiveRejectMessage");
    acceptor!(fix42 -> "8_AdminAndApplicationMessages");
    acceptor!(fix42 -> "8_OnlyAdminMessages");
    acceptor!(fix42 -> "8_OnlyApplicationMessages");
    acceptor!(fix42 -> "AlreadyLoggedOn");
    acceptor!(fix42 -> "RejectResentMessage");
    acceptor!(fix42 -> "ReverseRoute");
    acceptor!(fix42 -> "ReverseRouteWithEmptyRoutingTags");
}

mod fix43 {
    imports!();
    acceptor!(fix43 -> "10_MsgSeqNumEqual");
    acceptor!(fix43 -> "10_MsgSeqNumGreater");
    acceptor!(fix43 -> "10_MsgSeqNumLess");
    acceptor!(fix43 -> "11a_NewSeqNoGreater");
    acceptor!(fix43 -> "11b_NewSeqNoEqual");
    acceptor!(fix43 -> "11c_NewSeqNoLess");
    acceptor!(fix43 -> "13b_UnsolicitedLogoutMessage");
    acceptor!(fix43 -> "14a_BadField");
    acceptor!(fix43 -> "14b_RequiredFieldMissing");
    acceptor!(fix43 -> "14c_TagNotDefinedForMsgType");
    acceptor!(fix43 -> "14d_TagSpecifiedWithoutValue");
    acceptor!(fix43 -> "14e_IncorrectEnumValue");
    acceptor!(fix43 -> "14f_IncorrectDataFormat");
    acceptor!(fix43 -> "14g_HeaderBodyTrailerFieldsOutOfOrder");
    acceptor!(fix43 -> "14h_RepeatedTag");
    acceptor!(fix43 -> "14i_RepeatingGroupCountNotEqual");
    acceptor!(fix43 -> "15_HeaderAndBodyFieldsOrderedDifferently");
    acceptor!(fix43 -> "19a_PossResendMessageThatHAsAlreadyBeenSent");
    acceptor!(fix43 -> "19b_PossResendMessageThatHasNotBeenSent");
    acceptor!(fix43 -> "1a_ValidLogonMsgSeqNumTooHigh");
    acceptor!(fix43 -> "1a_ValidLogonWithCorrectMsgSeqNum");
    acceptor!(fix43 -> "1b_DuplicateIdentity");
    acceptor!(fix43 -> "1c_InvalidSenderCompID");
    acceptor!(fix43 -> "1c_InvalidTargetCompID");
    acceptor!(fix43 -> "1d_InvalidLogonBadSendingTime");
    acceptor!(fix43 -> "1d_InvalidLogonLengthInvalid");
    acceptor!(fix43 -> "1d_InvalidLogonWrongBeginString");
    acceptor!(fix43 -> "1e_NotLogonMessage");
    acceptor!(fix43 -> "20_SimultaneousResendRequest");
    acceptor!(fix43 -> "21_RepeatingGroupSpecifierWithValueOfZero");
    acceptor!(fix43 -> "2a_MsgSeqNumCorrect");
    acceptor!(fix43 -> "2b_MsgSeqNumTooHigh");
    acceptor!(fix43 -> "2c_MsgSeqNumTooLow");
    acceptor!(fix43 -> "2d_GarbledMessage");
    acceptor!(fix43 -> "2e_PossDupAlreadyReceived");
    acceptor!(fix43 -> "2e_PossDupNotReceived");
    acceptor!(fix43 -> "2f_PossDupOrigSendingTimeTooHigh");
    acceptor!(fix43 -> "2g_PossDupNoOrigSendingTime");
    acceptor!(fix43 -> "2i_BeginStringValueUnexpected");
    acceptor!(fix43 -> "2k_CompIDDoesNotMatchProfile");
    acceptor!(fix43 -> "2m_BodyLengthValueNotCorrect");
    acceptor!(fix43 -> "2o_SendingTimeValueOutOfRange");
    acceptor!(fix43 -> "2q_MsgTypeNotValid");
    acceptor!(fix43 -> "2r_UnregisteredMsgType");
    acceptor!(fix43 -> "2t_FirstThreeFieldsOutOfOrder");
    acceptor!(fix43 -> "3b_InvalidChecksum");
    acceptor!(fix43 -> "3c_GarbledMessage");
    acceptor!(fix43 -> "4a_NoDataSentDuringHeartBtInt");
    acceptor!(fix43 -> "4b_ReceivedTestRequest");
    acceptor!(fix43 -> "6_SendTestRequest");
    acceptor!(fix43 -> "7_ReceiveRejectMessage");
    acceptor!(fix43 -> "8_AdminAndApplicationMessages");
    acceptor!(fix43 -> "8_OnlyAdminMessages");
    acceptor!(fix43 -> "8_OnlyApplicationMessages");
    acceptor!(fix43 -> "AlreadyLoggedOn");
    acceptor!(fix43 -> "RejectResentMessage");
    acceptor!(fix43 -> "ReverseRoute");
    acceptor!(fix43 -> "ReverseRouteWithEmptyRoutingTags");
}

mod fix44 {
    imports!();
    acceptor!(fix44 -> "10_MsgSeqNumEqual");
    acceptor!(fix44 -> "10_MsgSeqNumGreater");
    acceptor!(fix44 -> "10_MsgSeqNumLess");
    acceptor!(fix44 -> "11a_NewSeqNoGreater");
    acceptor!(fix44 -> "11b_NewSeqNoEqual");
    acceptor!(fix44 -> "11c_NewSeqNoLess");
    acceptor!(fix44 -> "13b_UnsolicitedLogoutMessage");
    acceptor!(fix44 -> "14a_BadField");
    acceptor!(fix44 -> "14b_RequiredFieldMissing");
    acceptor!(fix44 -> "14c_TagNotDefinedForMsgType");
    acceptor!(fix44 -> "14d_TagSpecifiedWithoutValue");
    acceptor!(fix44 -> "14e_IncorrectEnumValue");
    acceptor!(fix44 -> "14f_IncorrectDataFormat");
    acceptor!(fix44 -> "14g_HeaderBodyTrailerFieldsOutOfOrder");
    acceptor!(fix44 -> "14h_RepeatedTag");
    acceptor!(fix44 -> "14i_RepeatingGroupCountNotEqual");
    acceptor!(fix44 -> "15_HeaderAndBodyFieldsOrderedDifferently");
    acceptor!(fix44 -> "19a_PossResendMessageThatHAsAlreadyBeenSent");
    acceptor!(fix44 -> "19b_PossResendMessageThatHasNotBeenSent");
    acceptor!(fix44 -> "1a_ValidLogonMsgSeqNumTooHigh");
    acceptor!(fix44 -> "1a_ValidLogonWithCorrectMsgSeqNum");
    acceptor!(fix44 -> "1b_DuplicateIdentity");
    acceptor!(fix44 -> "1c_InvalidSenderCompID");
    acceptor!(fix44 -> "1c_InvalidTargetCompID");
    acceptor!(fix44 -> "1d_InvalidLogonBadSendingTime");
    acceptor!(fix44 -> "1d_InvalidLogonLengthInvalid");
    acceptor!(fix44 -> "1d_InvalidLogonWrongBeginString");
    acceptor!(fix44 -> "1e_NotLogonMessage");
    acceptor!(fix44 -> "20_SimultaneousResendRequest");
    acceptor!(fix44 -> "21_RepeatingGroupSpecifierWithValueOfZero");
    acceptor!(fix44 -> "2a_MsgSeqNumCorrect");
    acceptor!(fix44 -> "2b_MsgSeqNumTooHigh");
    acceptor!(fix44 -> "2c_MsgSeqNumTooLow");
    acceptor!(fix44 -> "2d_GarbledMessage");
    acceptor!(fix44 -> "2e_PossDupAlreadyReceived");
    acceptor!(fix44 -> "2e_PossDupNotReceived");
    acceptor!(fix44 -> "2f_PossDupOrigSendingTimeTooHigh");
    acceptor!(fix44 -> "2g_PossDupNoOrigSendingTime");
    acceptor!(fix44 -> "2i_BeginStringValueUnexpected");
    acceptor!(fix44 -> "2k_CompIDDoesNotMatchProfile");
    acceptor!(fix44 -> "2m_BodyLengthValueNotCorrect");
    acceptor!(fix44 -> "2o_SendingTimeValueOutOfRange");
    acceptor!(fix44 -> "2q_MsgTypeNotValid");
    acceptor!(fix44 -> "2r_UnregisteredMsgType");
    acceptor!(fix44 -> "2t_FirstThreeFieldsOutOfOrder");
    acceptor!(fix44 -> "3b_InvalidChecksum");
    acceptor!(fix44 -> "3c_GarbledMessage");
    acceptor!(fix44 -> "4a_NoDataSentDuringHeartBtInt");
    acceptor!(fix44 -> "4b_ReceivedTestRequest");
    acceptor!(fix44 -> "6_SendTestRequest");
    acceptor!(fix44 -> "7_ReceiveRejectMessage");
    acceptor!(fix44 -> "8_AdminAndApplicationMessages");
    acceptor!(fix44 -> "8_OnlyAdminMessages");
    acceptor!(fix44 -> "8_OnlyApplicationMessages");
    acceptor!(fix44 -> "AlreadyLoggedOn");
    acceptor!(fix44 -> "ComponentFieldRequired");
    acceptor!(fix44 -> "FieldTypeError_NestedGroup");
    acceptor!(fix44 -> "FieldTypeError_RepeatingGroup");
    acceptor!(fix44 -> "FieldTypeError_Top");
    acceptor!(fix44 -> "InternationalCharacters");
    acceptor!(fix44 -> "RejectResentMessage");
    acceptor!(fix44 -> "ResendRepeatingGroup");
    acceptor!(fix44 -> "ReverseRoute");
    acceptor!(fix44 -> "ReverseRouteWithEmptyRoutingTags");
    acceptor!(fix44 -> "SessionReset");
    acceptor!(fix44 -> "issue146_MissingGroupDelimiter");
    acceptor!(fix44noreset -> "SessionResetAfterDisconnect");
}

mod fix50 {
    imports!();
    acceptor!(fix50 -> "10_MsgSeqNumEqual");
    acceptor!(fix50 -> "10_MsgSeqNumGreater");
    acceptor!(fix50 -> "10_MsgSeqNumLess");
    acceptor!(fix50 -> "11a_NewSeqNoGreater");
    acceptor!(fix50 -> "11b_NewSeqNoEqual");
    acceptor!(fix50 -> "11c_NewSeqNoLess");
    acceptor!(fix50 -> "13b_UnsolicitedLogoutMessage");
    acceptor!(fix50 -> "14a_BadField");
    acceptor!(fix50 -> "14b_RequiredFieldMissing");
    acceptor!(fix50 -> "14c_TagNotDefinedForMsgType");
    acceptor!(fix50 -> "14d_TagSpecifiedWithoutValue");
    acceptor!(fix50 -> "14e_IncorrectEnumValue");
    acceptor!(fix50 -> "14f_IncorrectDataFormat");
    acceptor!(fix50 -> "14g_HeaderBodyTrailerFieldsOutOfOrder");
    acceptor!(fix50 -> "14h_RepeatedTag");
    acceptor!(fix50 -> "14i_RepeatingGroupCountNotEqual");
    acceptor!(fix50 -> "15_HeaderAndBodyFieldsOrderedDifferently");
    acceptor!(fix50 -> "19a_PossResendMessageThatHAsAlreadyBeenSent");
    acceptor!(fix50 -> "19b_PossResendMessageThatHasNotBeenSent");
    acceptor!(fix50 -> "1a_ValidLogonMsgSeqNumTooHigh");
    acceptor!(fix50 -> "1a_ValidLogonWithCorrectMsgSeqNum");
    acceptor!(fix50 -> "1b_DuplicateIdentity");
    acceptor!(fix50 -> "1c_InvalidSenderCompID");
    acceptor!(fix50 -> "1c_InvalidTargetCompID");
    acceptor!(fix50 -> "1d_InvalidLogonBadSendingTime");
    acceptor!(fix50 -> "1d_InvalidLogonLengthInvalid");
    acceptor!(fix50 -> "1d_InvalidLogonNoDefaultApplVerID");
    acceptor!(fix50 -> "1d_InvalidLogonWrongBeginString");
    acceptor!(fix50 -> "1e_NotLogonMessage");
    acceptor!(fix50 -> "20_SimultaneousResendRequest");
    acceptor!(fix50 -> "21_RepeatingGroupSpecifierWithValueOfZero");
    acceptor!(fix50 -> "2a_MsgSeqNumCorrect");
    acceptor!(fix50 -> "2b_MsgSeqNumTooHigh");
    acceptor!(fix50 -> "2c_MsgSeqNumTooLow");
    acceptor!(fix50 -> "2d_GarbledMessage");
    acceptor!(fix50 -> "2e_PossDupAlreadyReceived");
    acceptor!(fix50 -> "2e_PossDupNotReceived");
    acceptor!(fix50 -> "2f_PossDupOrigSendingTimeTooHigh");
    acceptor!(fix50 -> "2g_PossDupNoOrigSendingTime");
    acceptor!(fix50 -> "2i_BeginStringValueUnexpected");
    acceptor!(fix50 -> "2k_CompIDDoesNotMatchProfile");
    acceptor!(fix50 -> "2m_BodyLengthValueNotCorrect");
    acceptor!(fix50 -> "2o_SendingTimeValueOutOfRange");
    acceptor!(fix50 -> "2q_MsgTypeNotValid");
    acceptor!(fix50 -> "2r_UnregisteredMsgType");
    acceptor!(fix50 -> "2t_FirstThreeFieldsOutOfOrder");
    acceptor!(fix50 -> "3b_InvalidChecksum");
    acceptor!(fix50 -> "3c_GarbledMessage");
    acceptor!(fix50 -> "4a_NoDataSentDuringHeartBtInt");
    acceptor!(fix50 -> "4b_ReceivedTestRequest");
    acceptor!(fix50 -> "6_SendTestRequest");
    acceptor!(fix50 -> "7_ReceiveRejectMessage");
    acceptor!(fix50 -> "8_AdminAndApplicationMessages");
    acceptor!(fix50 -> "8_OnlyAdminMessages");
    acceptor!(fix50 -> "8_OnlyApplicationMessages");
    acceptor!(fix50 -> "AlreadyLoggedOn");
    acceptor!(fix50 -> "BeginString");
    acceptor!(fix50 -> "RejectResentMessage");
    acceptor!(fix50 -> "ReverseRoute");
    acceptor!(fix50 -> "ReverseRouteWithEmptyRoutingTags");
    acceptor!(fix50 -> "SessionReset");
}

mod fix50sp1 {
    imports!();
    acceptor!(fix50sp1 -> "10_MsgSeqNumEqual");
    acceptor!(fix50sp1 -> "10_MsgSeqNumGreater");
    acceptor!(fix50sp1 -> "10_MsgSeqNumLess");
    acceptor!(fix50sp1 -> "11a_NewSeqNoGreater");
    acceptor!(fix50sp1 -> "11b_NewSeqNoEqual");
    acceptor!(fix50sp1 -> "11c_NewSeqNoLess");
    acceptor!(fix50sp1 -> "13b_UnsolicitedLogoutMessage");
    acceptor!(fix50sp1 -> "14a_BadField");
    acceptor!(fix50sp1 -> "14b_RequiredFieldMissing");
    acceptor!(fix50sp1 -> "14c_TagNotDefinedForMsgType");
    acceptor!(fix50sp1 -> "14d_TagSpecifiedWithoutValue");
    acceptor!(fix50sp1 -> "14e_IncorrectEnumValue");
    acceptor!(fix50sp1 -> "14f_IncorrectDataFormat");
    acceptor!(fix50sp1 -> "14g_HeaderBodyTrailerFieldsOutOfOrder");
    acceptor!(fix50sp1 -> "14h_RepeatedTag");
    acceptor!(fix50sp1 -> "14i_RepeatingGroupCountNotEqual");
    acceptor!(fix50sp1 -> "15_HeaderAndBodyFieldsOrderedDifferently");
    acceptor!(fix50sp1 -> "19a_PossResendMessageThatHAsAlreadyBeenSent");
    acceptor!(fix50sp1 -> "19b_PossResendMessageThatHasNotBeenSent");
    acceptor!(fix50sp1 -> "1a_ValidLogonMsgSeqNumTooHigh");
    acceptor!(fix50sp1 -> "1a_ValidLogonWithCorrectMsgSeqNum");
    acceptor!(fix50sp1 -> "1b_DuplicateIdentity");
    acceptor!(fix50sp1 -> "1c_InvalidSenderCompID");
    acceptor!(fix50sp1 -> "1c_InvalidTargetCompID");
    acceptor!(fix50sp1 -> "1d_InvalidLogonBadSendingTime");
    acceptor!(fix50sp1 -> "1d_InvalidLogonLengthInvalid");
    acceptor!(fix50sp1 -> "1d_InvalidLogonNoDefaultApplVerID");
    acceptor!(fix50sp1 -> "1d_InvalidLogonWrongBeginString");
    acceptor!(fix50sp1 -> "1e_NotLogonMessage");
    acceptor!(fix50sp1 -> "20_SimultaneousResendRequest");
    acceptor!(fix50sp1 -> "21_RepeatingGroupSpecifierWithValueOfZero");
    acceptor!(fix50sp1 -> "2a_MsgSeqNumCorrect");
    acceptor!(fix50sp1 -> "2b_MsgSeqNumTooHigh");
    acceptor!(fix50sp1 -> "2c_MsgSeqNumTooLow");
    acceptor!(fix50sp1 -> "2d_GarbledMessage");
    acceptor!(fix50sp1 -> "2e_PossDupAlreadyReceived");
    acceptor!(fix50sp1 -> "2e_PossDupNotReceived");
    acceptor!(fix50sp1 -> "2f_PossDupOrigSendingTimeTooHigh");
    acceptor!(fix50sp1 -> "2g_PossDupNoOrigSendingTime");
    acceptor!(fix50sp1 -> "2i_BeginStringValueUnexpected");
    acceptor!(fix50sp1 -> "2k_CompIDDoesNotMatchProfile");
    acceptor!(fix50sp1 -> "2m_BodyLengthValueNotCorrect");
    acceptor!(fix50sp1 -> "2o_SendingTimeValueOutOfRange");
    acceptor!(fix50sp1 -> "2q_MsgTypeNotValid");
    acceptor!(fix50sp1 -> "2r_UnregisteredMsgType");
    acceptor!(fix50sp1 -> "2t_FirstThreeFieldsOutOfOrder");
    acceptor!(fix50sp1 -> "3b_InvalidChecksum");
    acceptor!(fix50sp1 -> "3c_GarbledMessage");
    acceptor!(fix50sp1 -> "4a_NoDataSentDuringHeartBtInt");
    acceptor!(fix50sp1 -> "4b_ReceivedTestRequest");
    acceptor!(fix50sp1 -> "6_SendTestRequest");
    acceptor!(fix50sp1 -> "7_ReceiveRejectMessage");
    acceptor!(fix50sp1 -> "8_AdminAndApplicationMessages");
    acceptor!(fix50sp1 -> "8_OnlyAdminMessages");
    acceptor!(fix50sp1 -> "8_OnlyApplicationMessages");
    acceptor!(fix50sp1 -> "AlreadyLoggedOn");
    acceptor!(fix50sp1 -> "RejectResentMessage");
    acceptor!(fix50sp1 -> "ReverseRoute");
    acceptor!(fix50sp1 -> "ReverseRouteWithEmptyRoutingTags");
    acceptor!(fix50sp1 -> "SessionReset");
}
mod fix50sp2 {
    imports!();
    acceptor!(fix50sp2 -> "10_MsgSeqNumEqual");
    acceptor!(fix50sp2 -> "10_MsgSeqNumGreater");
    acceptor!(fix50sp2 -> "10_MsgSeqNumLess");
    acceptor!(fix50sp2 -> "11a_NewSeqNoGreater");
    acceptor!(fix50sp2 -> "11b_NewSeqNoEqual");
    acceptor!(fix50sp2 -> "11c_NewSeqNoLess");
    acceptor!(fix50sp2 -> "13b_UnsolicitedLogoutMessage");
    acceptor!(fix50sp2 -> "14a_BadField");
    acceptor!(fix50sp2 -> "14b_RequiredFieldMissing");
    acceptor!(fix50sp2 -> "14c_TagNotDefinedForMsgType");
    acceptor!(fix50sp2 -> "14d_TagSpecifiedWithoutValue");
    acceptor!(fix50sp2 -> "14e_IncorrectEnumValue");
    acceptor!(fix50sp2 -> "14f_IncorrectDataFormat");
    acceptor!(fix50sp2 -> "14g_HeaderBodyTrailerFieldsOutOfOrder");
    acceptor!(fix50sp2 -> "14h_RepeatedTag");
    acceptor!(fix50sp2 -> "14i_RepeatingGroupCountNotEqual");
    acceptor!(fix50sp2 -> "15_HeaderAndBodyFieldsOrderedDifferently");
    acceptor!(fix50sp2 -> "19a_PossResendMessageThatHAsAlreadyBeenSent");
    acceptor!(fix50sp2 -> "19b_PossResendMessageThatHasNotBeenSent");
    acceptor!(fix50sp2 -> "1a_ValidLogonMsgSeqNumTooHigh");
    acceptor!(fix50sp2 -> "1a_ValidLogonWithCorrectMsgSeqNum");
    acceptor!(fix50sp2 -> "1b_DuplicateIdentity");
    acceptor!(fix50sp2 -> "1c_InvalidSenderCompID");
    acceptor!(fix50sp2 -> "1c_InvalidTargetCompID");
    acceptor!(fix50sp2 -> "1d_InvalidLogonBadSendingTime");
    acceptor!(fix50sp2 -> "1d_InvalidLogonLengthInvalid");
    acceptor!(fix50sp2 -> "1d_InvalidLogonNoDefaultApplVerID");
    acceptor!(fix50sp2 -> "1d_InvalidLogonWrongBeginString");
    acceptor!(fix50sp2 -> "1e_NotLogonMessage");
    acceptor!(fix50sp2 -> "20_SimultaneousResendRequest");
    acceptor!(fix50sp2 -> "21_RepeatingGroupSpecifierWithValueOfZero");
    acceptor!(fix50sp2 -> "2a_MsgSeqNumCorrect");
    acceptor!(fix50sp2 -> "2b_MsgSeqNumTooHigh");
    acceptor!(fix50sp2 -> "2c_MsgSeqNumTooLow");
    acceptor!(fix50sp2 -> "2d_GarbledMessage");
    acceptor!(fix50sp2 -> "2e_PossDupAlreadyReceived");
    acceptor!(fix50sp2 -> "2e_PossDupNotReceived");
    acceptor!(fix50sp2 -> "2f_PossDupOrigSendingTimeTooHigh");
    acceptor!(fix50sp2 -> "2g_PossDupNoOrigSendingTime");
    acceptor!(fix50sp2 -> "2i_BeginStringValueUnexpected");
    acceptor!(fix50sp2 -> "2k_CompIDDoesNotMatchProfile");
    acceptor!(fix50sp2 -> "2m_BodyLengthValueNotCorrect");
    acceptor!(fix50sp2 -> "2o_SendingTimeValueOutOfRange");
    acceptor!(fix50sp2 -> "2q_MsgTypeNotValid");
    acceptor!(fix50sp2 -> "2r_UnregisteredMsgType");
    acceptor!(fix50sp2 -> "2t_FirstThreeFieldsOutOfOrder");
    acceptor!(fix50sp2 -> "3b_InvalidChecksum");
    acceptor!(fix50sp2 -> "3c_GarbledMessage");
    acceptor!(fix50sp2 -> "4a_NoDataSentDuringHeartBtInt");
    acceptor!(fix50sp2 -> "4b_ReceivedTestRequest");
    acceptor!(fix50sp2 -> "6_SendTestRequest");
    acceptor!(fix50sp2 -> "7_ReceiveRejectMessage");
    acceptor!(fix50sp2 -> "8_AdminAndApplicationMessages");
    acceptor!(fix50sp2 -> "8_OnlyAdminMessages");
    acceptor!(fix50sp2 -> "8_OnlyApplicationMessages");
    acceptor!(fix50sp2 -> "AlreadyLoggedOn");
    acceptor!(fix50sp2 -> "RejectResentMessage");
    acceptor!(fix50sp2 -> "ReverseRoute");
    acceptor!(fix50sp2 -> "ReverseRouteWithEmptyRoutingTags");
    acceptor!(fix50sp2 -> "SessionReset");
}

mod future {
    imports!();
    acceptor!(future -> "14j_OutOfRepeatingGroupMembers");
    acceptor!(future -> "14k_EmbeddedSOH");
}

mod misc {
    imports!();
    acceptor!(misc -> "FIX42LastMsgSeqNumProcessed");
    acceptor!(misc -> "FIX42MaxMessagesInResend");
    acceptor!(misc -> "FIX42Subs");
    acceptor!(misc -> "FIX42TestRequest");
}
