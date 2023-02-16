use std::borrow::Cow;

use dfx_core::message::Message;
use crate::fix44::fields::*;

/// News
#[derive(Clone, Debug)]
pub struct News<'a> {
    inner: Cow<'a, Message>
}

impl<'a> News<'a> {
    //TODO implement
    
    pub fn no_related_sym<'b: 'a>(&'b self) -> Option<NoRelatedSym<'b>> {
        self.inner.get_field(NoRelatedSym::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_no_related_sym<'b: 'a>(&mut self, no_related_sym: NoRelatedSym<'b>) {
        self.inner.to_mut().set_field(no_related_sym);
    }
        
    pub fn headline<'b: 'a>(&'b self) -> Option<Headline<'b>> {
        self.inner.get_field(Headline::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_headline<'b: 'a>(&mut self, headline: Headline<'b>) {
        self.inner.to_mut().set_field(headline);
    }
        
    pub fn encoded_headline_len<'b: 'a>(&'b self) -> Option<EncodedHeadlineLen<'b>> {
        self.inner.get_field(EncodedHeadlineLen::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_encoded_headline_len<'b: 'a>(&mut self, encoded_headline_len: EncodedHeadlineLen<'b>) {
        self.inner.to_mut().set_field(encoded_headline_len);
    }
        
    pub fn encoded_headline<'b: 'a>(&'b self) -> Option<EncodedHeadline<'b>> {
        self.inner.get_field(EncodedHeadline::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_encoded_headline<'b: 'a>(&mut self, encoded_headline: EncodedHeadline<'b>) {
        self.inner.to_mut().set_field(encoded_headline);
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

