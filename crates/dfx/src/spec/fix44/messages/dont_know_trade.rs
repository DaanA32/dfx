use std::borrow::Cow;

use dfx_core::message::Message;

use super::super::fields::*;

/// DontKnowTrade
#[derive(Clone, Debug)]
pub struct DontKnowTrade<'a> {
    inner: Cow<'a, Message>
}

impl<'a> DontKnowTrade<'a> {
    //TODO implement
    
    pub fn exec_id<'b: 'a>(&'b self) -> Option<ExecId<'b>> {
        self.inner.get_field(ExecId::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_exec_id<'b: 'a>(&mut self, exec_id: ExecId<'b>) {
        self.inner.to_mut().set_field(exec_id);
    }
        
    pub fn dk_reason<'b: 'a>(&'b self) -> Option<DkReason<'b>> {
        self.inner.get_field(DkReason::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_dk_reason<'b: 'a>(&mut self, dk_reason: DkReason<'b>) {
        self.inner.to_mut().set_field(dk_reason);
    }
        
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

