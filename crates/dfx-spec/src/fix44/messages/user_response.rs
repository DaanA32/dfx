use std::borrow::Cow;

use dfx_core::message::Message;
use crate::fix44::fields::*;

/// UserResponse
#[derive(Clone, Debug)]
pub struct UserResponse<'a> {
    inner: Cow<'a, Message>
}

impl<'a> UserResponse<'a> {
    //TODO implement
    
    pub fn username<'b: 'a>(&'b self) -> Option<Username<'b>> {
        self.inner.get_field(Username::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_username<'b: 'a>(&mut self, username: Username<'b>) {
        self.inner.to_mut().set_field(username);
    }
        
    pub fn user_request_id<'b: 'a>(&'b self) -> Option<UserRequestId<'b>> {
        self.inner.get_field(UserRequestId::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_user_request_id<'b: 'a>(&mut self, user_request_id: UserRequestId<'b>) {
        self.inner.to_mut().set_field(user_request_id);
    }
        
    pub fn user_status<'b: 'a>(&'b self) -> Option<UserStatus<'b>> {
        self.inner.get_field(UserStatus::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_user_status<'b: 'a>(&mut self, user_status: UserStatus<'b>) {
        self.inner.to_mut().set_field(user_status);
    }
        
    pub fn user_status_text<'b: 'a>(&'b self) -> Option<UserStatusText<'b>> {
        self.inner.get_field(UserStatusText::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_user_status_text<'b: 'a>(&mut self, user_status_text: UserStatusText<'b>) {
        self.inner.to_mut().set_field(user_status_text);
    }
        
}


