use dfx_core::field_map::Field;
use dfx_core::field_map::Tag;
use dfx_core::tags;

use std::ops::Deref;

pub struct MsgType(Field);
impl MsgType {
    pub const TAG: Tag = tags::MsgType;

    pub const HEARTBEAT: &'static str = "0"; //TODO
    pub const TEST_REQUEST: &'static str = "1"; //TODO
    pub const RESEND_REQUEST: &'static str = "2"; //TODO
    pub const REJECT: &'static str = "3"; //TODO
    pub const SEQUENCE_RESET: &'static str = "4"; //TODO
    pub const LOGOUT: &'static str = "5"; //TODO
    pub const LOGON: &'static str = "A";
    pub const BUSINESS_MESSAGE_REJECT: &'static str = "j";

    pub fn new(msg_type: &str) -> Self {
        Self(Field::new(
            Self::TAG,
            msg_type
        ))
    }
}

#[derive(Debug, Clone)]
pub struct EncryptMethod(Field);
impl EncryptMethod {
    pub const TAG: Tag = tags::EncryptMethod;
    pub fn new(val: u32) -> Self {
        Self(Field::new(
            EncryptMethod::TAG,
            EncryptMethod::string_value(val),
        ))
    }
    pub fn string_value(val: u32) -> String {
        format!("{}", val)
    }
    pub const NONE: u32 = 0;
}
impl Deref for EncryptMethod {
    type Target = Field;
    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct HeartBtInt(Field);
impl HeartBtInt {
    pub const TAG: Tag = tags::HeartBtInt;
    pub fn new(val: u32) -> Self {
        Self(Field::new(
            HeartBtInt::TAG,
            HeartBtInt::string_value(val),
        ))
    }
    pub fn string_value(val: u32) -> String {
        format!("{}", val)
    }
    pub const NONE: u32 = 0;
}
impl Deref for HeartBtInt {
    type Target = Field;
    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct DefaultApplVerID(Field);
impl DefaultApplVerID {
    pub const TAG: Tag = tags::DefaultApplVerID;
    pub fn new(val: String) -> Self {
        Self(Field::new(DefaultApplVerID::TAG, val))
    }
}
impl Deref for DefaultApplVerID {
    type Target = Field;
    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct ResetSeqNumFlag(Field);
impl ResetSeqNumFlag {
    pub const TAG: Tag = tags::ResetSeqNumFlag;
    pub fn new(val: bool) -> Self {
        Self(Field::new(
            ResetSeqNumFlag::TAG,
            ResetSeqNumFlag::string_value(val),
        ))
    }
    pub fn string_value(val: bool) -> String {
        if val {
            "Y".into()
        } else {
            "N".into()
        }
    }
    pub const NONE: u32 = 0;
}
impl Deref for ResetSeqNumFlag {
    type Target = Field;
    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}
// TODO: DerefMut
// TODO: Into/From Field?
// TODO: codegen

// pub struct ApplVerID;
// impl ApplVerID {
//     pub const FIX27: u32 = 0;
//     pub const FIX30: u32 = 1;
//     pub const FIX40: u32 = 2;
//     pub const FIX41: u32 = 3;
//     pub const FIX42: u32 = 4;
//     pub const FIX43: u32 = 5;
//     pub const FIX44: u32 = 6;
//     pub const FIX50: u32 = 7;
//     pub const FIX50SP1: u32 = 8;
//     pub const FIX50SP2: u32 = 9;
// }

// pub struct SessionRejectReason;
// impl SessionRejectReason {
//     pub const VALUE_IS_INCORRECT: &str = "Value incorrect";
//     pub const SENDING_TIME_ACCURACY_PROBLEM: &str = "Sending time";
//     pub const COMPID_PROBLEM: &str = "Comp ID";
//     pub const INVALID_MSGTYPE: &str = "Invalid MSG Type";
//     pub const INVALID_TAG_NUMBER: &str = "Invalid Tag";
//     pub const REQUIRED_TAG_MISSING: &str = "Required Tag";
// }
