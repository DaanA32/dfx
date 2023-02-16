use std::borrow::Cow;

use dfx_core::message::Message;
use crate::fix44::fields::*;

/// SecurityStatusRequest
#[derive(Clone, Debug)]
pub struct SecurityStatusRequest<'a> {
    inner: Cow<'a, Message>
}

impl<'a> SecurityStatusRequest<'a> {
    //TODO implement
    
    pub fn currency<'b: 'a>(&'b self) -> Option<Currency<'b>> {
        self.inner.get_field(Currency::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_currency<'b: 'a>(&mut self, currency: Currency<'b>) {
        self.inner.to_mut().set_field(currency);
    }
        
    pub fn no_legs<'b: 'a>(&'b self) -> Option<NoLegs<'b>> {
        self.inner.get_field(NoLegs::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_no_legs<'b: 'a>(&mut self, no_legs: NoLegs<'b>) {
        self.inner.to_mut().set_field(no_legs);
    }
        
    pub fn no_legs_group(&self) -> Option<NoLegsGroup> {
        todo!()
    }
    pub fn set_no_legs_group(&mut self, _no_legs_group: NoLegsGroup) {
        todo!()
    }
        
}


pub struct NoLegsGroup {

}

