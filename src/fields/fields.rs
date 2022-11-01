use crate::field_map::FieldBase;
use crate::field_map::Tag;
use crate::tags;

use std::ops::Deref;

pub struct MsgType(FieldBase);
impl MsgType {
    pub const TAG: Tag = tags::MsgType;
    pub const LOGON: &'static str = "A";
}

#[derive(Debug, Clone)]
pub struct EncryptMethod(FieldBase);
impl EncryptMethod {
    pub const TAG: Tag = tags::EncryptMethod;
    pub fn new(val: u32) -> Self {
        Self(FieldBase::new(EncryptMethod::TAG, EncryptMethod::string_value()))
    }
    pub fn string_value() -> String {
        todo!()
    }
    pub const NONE: u32 = 0;
}
impl Deref for EncryptMethod {
    type Target = FieldBase;
    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct HeartBtInt(FieldBase);
impl HeartBtInt {
    pub const TAG: Tag = tags::HeartBtInt;
    pub fn new(val: u32) -> Self {
        Self(FieldBase::new(HeartBtInt::TAG, HeartBtInt::string_value()))
    }
    pub fn string_value() -> String {
        todo!()
    }
    pub const NONE: u32 = 0;
}
impl Deref for HeartBtInt {
    type Target = FieldBase;
    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct DefaultApplVerID(FieldBase);
impl DefaultApplVerID {
    pub const TAG: Tag = tags::DefaultApplVerID;
    pub fn new(val: String) -> Self {
        Self(FieldBase::new(DefaultApplVerID::TAG, val))
    }
}
impl Deref for DefaultApplVerID {
    type Target = FieldBase;
    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct ResetSeqNumFlag(FieldBase);
impl ResetSeqNumFlag {
    pub const TAG: Tag = tags::ResetSeqNumFlag;
    pub fn new(val: bool) -> Self {
        Self(FieldBase::new(ResetSeqNumFlag::TAG, ResetSeqNumFlag::string_value()))
    }
    pub fn string_value() -> String {
        todo!()
    }
    pub const NONE: u32 = 0;
}
impl Deref for ResetSeqNumFlag {
    type Target = FieldBase;
    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}
// TODO: DerefMut
// TODO: Into/From FieldBase?
// TODO: codegen
