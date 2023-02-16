use std::borrow::Cow;

use dfx_core::message::Message;
use crate::fix44::fields::*;

/// SecurityList
#[derive(Clone, Debug)]
pub struct SecurityList<'a> {
    inner: Cow<'a, Message>
}

impl<'a> SecurityList<'a> {
    //TODO implement
    
    pub fn no_related_sym<'b: 'a>(&'b self) -> Option<NoRelatedSym<'b>> {
        self.inner.get_field(NoRelatedSym::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_no_related_sym<'b: 'a>(&mut self, no_related_sym: NoRelatedSym<'b>) {
        self.inner.to_mut().set_field(no_related_sym);
    }
        
    pub fn last_fragment<'b: 'a>(&'b self) -> Option<LastFragment<'b>> {
        self.inner.get_field(LastFragment::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_last_fragment<'b: 'a>(&mut self, last_fragment: LastFragment<'b>) {
        self.inner.to_mut().set_field(last_fragment);
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

