use std::borrow::Cow;

use dfx_core::message::Message;
use crate::fix44::fields::*;

/// SecurityTypes
#[derive(Clone, Debug)]
pub struct SecurityTypes<'a> {
    inner: Cow<'a, Message>
}

impl<'a> SecurityTypes<'a> {
    //TODO implement
    
    pub fn encoded_text_len<'b: 'a>(&'b self) -> Option<EncodedTextLen<'b>> {
        self.inner.get_field(EncodedTextLen::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_encoded_text_len<'b: 'a>(&mut self, encoded_text_len: EncodedTextLen<'b>) {
        self.inner.to_mut().set_field(encoded_text_len);
    }
        
    pub fn encoded_text<'b: 'a>(&'b self) -> Option<EncodedText<'b>> {
        self.inner.get_field(EncodedText::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_encoded_text<'b: 'a>(&mut self, encoded_text: EncodedText<'b>) {
        self.inner.to_mut().set_field(encoded_text);
    }
        
    pub fn no_security_types<'b: 'a>(&'b self) -> Option<NoSecurityTypes<'b>> {
        self.inner.get_field(NoSecurityTypes::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_no_security_types<'b: 'a>(&mut self, no_security_types: NoSecurityTypes<'b>) {
        self.inner.to_mut().set_field(no_security_types);
    }
        
    pub fn last_fragment<'b: 'a>(&'b self) -> Option<LastFragment<'b>> {
        self.inner.get_field(LastFragment::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_last_fragment<'b: 'a>(&mut self, last_fragment: LastFragment<'b>) {
        self.inner.to_mut().set_field(last_fragment);
    }
        
    pub fn no_security_types_group(&self) -> Option<NoSecurityTypesGroup> {
        todo!()
    }
    pub fn set_no_security_types_group(&mut self, _no_security_types_group: NoSecurityTypesGroup) {
        todo!()
    }
        
}


pub struct NoSecurityTypesGroup {

}

