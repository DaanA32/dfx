use std::borrow::Cow;

use dfx_core::message::Message;
use crate::fix44::fields::*;

/// ResendRequest
#[derive(Clone, Debug)]
pub struct ResendRequest<'a> {
    inner: Cow<'a, Message>
}

impl<'a> ResendRequest<'a> {
    //TODO implement
    
    pub fn begin_seq_no<'b: 'a>(&'b self) -> Option<BeginSeqNo<'b>> {
        self.inner.get_field(BeginSeqNo::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_begin_seq_no<'b: 'a>(&mut self, begin_seq_no: BeginSeqNo<'b>) {
        self.inner.to_mut().set_field(begin_seq_no);
    }
        
    pub fn end_seq_no<'b: 'a>(&'b self) -> Option<EndSeqNo<'b>> {
        self.inner.get_field(EndSeqNo::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_end_seq_no<'b: 'a>(&mut self, end_seq_no: EndSeqNo<'b>) {
        self.inner.to_mut().set_field(end_seq_no);
    }
        
}


