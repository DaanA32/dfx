use std::borrow::Cow;

use dfx_core::message::Message;

use super::super::fields::*;

/// UserRequest
#[derive(Clone, Debug)]
pub struct UserRequest<'a> {
    inner: Cow<'a, Message>
}

impl<'a> UserRequest<'a> {
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
        
    pub fn user_request_id<'b: 'a>(&'b self) -> Option<UserRequestId<'b>> {
        self.inner.get_field(UserRequestId::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_user_request_id<'b: 'a>(&mut self, user_request_id: UserRequestId<'b>) {
        self.inner.to_mut().set_field(user_request_id);
    }
        
    pub fn user_request_type<'b: 'a>(&'b self) -> Option<UserRequestType<'b>> {
        self.inner.get_field(UserRequestType::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_user_request_type<'b: 'a>(&mut self, user_request_type: UserRequestType<'b>) {
        self.inner.to_mut().set_field(user_request_type);
    }
        
    pub fn new_password<'b: 'a>(&'b self) -> Option<NewPassword<'b>> {
        self.inner.get_field(NewPassword::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_new_password<'b: 'a>(&mut self, new_password: NewPassword<'b>) {
        self.inner.to_mut().set_field(new_password);
    }
        
}


