use std::borrow::Cow;

use dfx_core::message::Message;
use crate::fix44::fields::*;

/// Logon
#[derive(Clone, Debug)]
pub struct Logon<'a> {
    inner: Cow<'a, Message>
}

impl<'a> Logon<'a> {
    //TODO implement
    
    pub fn raw_data_length<'b: 'a>(&'b self) -> Option<RawDataLength<'b>> {
        self.inner.get_field(RawDataLength::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_raw_data_length<'b: 'a>(&mut self, raw_data_length: RawDataLength<'b>) {
        self.inner.to_mut().set_field(raw_data_length);
    }
        
    pub fn raw_data<'b: 'a>(&'b self) -> Option<RawData<'b>> {
        self.inner.get_field(RawData::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_raw_data<'b: 'a>(&mut self, raw_data: RawData<'b>) {
        self.inner.to_mut().set_field(raw_data);
    }
        
    pub fn encrypt_method<'b: 'a>(&'b self) -> Option<EncryptMethod<'b>> {
        self.inner.get_field(EncryptMethod::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_encrypt_method<'b: 'a>(&mut self, encrypt_method: EncryptMethod<'b>) {
        self.inner.to_mut().set_field(encrypt_method);
    }
        
    pub fn heart_bt_int<'b: 'a>(&'b self) -> Option<HeartBtInt<'b>> {
        self.inner.get_field(HeartBtInt::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_heart_bt_int<'b: 'a>(&mut self, heart_bt_int: HeartBtInt<'b>) {
        self.inner.to_mut().set_field(heart_bt_int);
    }
        
    pub fn reset_seq_num_flag<'b: 'a>(&'b self) -> Option<ResetSeqNumFlag<'b>> {
        self.inner.get_field(ResetSeqNumFlag::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_reset_seq_num_flag<'b: 'a>(&mut self, reset_seq_num_flag: ResetSeqNumFlag<'b>) {
        self.inner.to_mut().set_field(reset_seq_num_flag);
    }
        
    pub fn max_message_size<'b: 'a>(&'b self) -> Option<MaxMessageSize<'b>> {
        self.inner.get_field(MaxMessageSize::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_max_message_size<'b: 'a>(&mut self, max_message_size: MaxMessageSize<'b>) {
        self.inner.to_mut().set_field(max_message_size);
    }
        
    pub fn no_msg_types<'b: 'a>(&'b self) -> Option<NoMsgTypes<'b>> {
        self.inner.get_field(NoMsgTypes::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_no_msg_types<'b: 'a>(&mut self, no_msg_types: NoMsgTypes<'b>) {
        self.inner.to_mut().set_field(no_msg_types);
    }
        
    pub fn test_message_indicator<'b: 'a>(&'b self) -> Option<TestMessageIndicator<'b>> {
        self.inner.get_field(TestMessageIndicator::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_test_message_indicator<'b: 'a>(&mut self, test_message_indicator: TestMessageIndicator<'b>) {
        self.inner.to_mut().set_field(test_message_indicator);
    }
        
    pub fn username<'b: 'a>(&'b self) -> Option<Username<'b>> {
        self.inner.get_field(Username::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_username<'b: 'a>(&mut self, username: Username<'b>) {
        self.inner.to_mut().set_field(username);
    }
        
    pub fn password<'b: 'a>(&'b self) -> Option<Password<'b>> {
        self.inner.get_field(Password::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_password<'b: 'a>(&mut self, password: Password<'b>) {
        self.inner.to_mut().set_field(password);
    }
        
    pub fn next_expected_msg_seq_num<'b: 'a>(&'b self) -> Option<NextExpectedMsgSeqNum<'b>> {
        self.inner.get_field(NextExpectedMsgSeqNum::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_next_expected_msg_seq_num<'b: 'a>(&mut self, next_expected_msg_seq_num: NextExpectedMsgSeqNum<'b>) {
        self.inner.to_mut().set_field(next_expected_msg_seq_num);
    }
        
    pub fn no_msg_types_group(&self) -> Option<NoMsgTypesGroup> {
        todo!()
    }
    pub fn set_no_msg_types_group(&mut self, _no_msg_types_group: NoMsgTypesGroup) {
        todo!()
    }
        
}


pub struct NoMsgTypesGroup {

}

