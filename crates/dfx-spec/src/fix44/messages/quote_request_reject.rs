use std::borrow::Cow;

use dfx_core::message::Message;
use crate::fix44::fields::*;

/// QuoteRequestReject
#[derive(Clone, Debug)]
pub struct QuoteRequestReject<'a> {
    inner: Cow<'a, Message>
}

impl<'a> QuoteRequestReject<'a> {
    //TODO implement
    
    pub fn no_related_sym<'b: 'a>(&'b self) -> Option<NoRelatedSym<'b>> {
        self.inner.get_field(NoRelatedSym::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_no_related_sym<'b: 'a>(&mut self, no_related_sym: NoRelatedSym<'b>) {
        self.inner.to_mut().set_field(no_related_sym);
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
        
    pub fn no_related_sym_group(&self) -> Option<NoRelatedSymGroup> {
        todo!()
    }
    pub fn set_no_related_sym_group(&mut self, _no_related_sym_group: NoRelatedSymGroup) {
        todo!()
    }
        
}


pub struct NoRelatedSymGroup {

}

