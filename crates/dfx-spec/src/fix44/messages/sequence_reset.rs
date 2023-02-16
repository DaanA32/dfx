use std::borrow::Cow;

use dfx_core::message::Message;
use crate::fix44::fields::*;

/// SequenceReset
#[derive(Clone, Debug)]
pub struct SequenceReset<'a> {
    inner: Cow<'a, Message>
}

impl<'a> SequenceReset<'a> {
    //TODO implement
    
    pub fn new_seq_no<'b: 'a>(&'b self) -> Option<NewSeqNo<'b>> {
        self.inner.get_field(NewSeqNo::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_new_seq_no<'b: 'a>(&mut self, new_seq_no: NewSeqNo<'b>) {
        self.inner.to_mut().set_field(new_seq_no);
    }
        
    pub fn gap_fill_flag<'b: 'a>(&'b self) -> Option<GapFillFlag<'b>> {
        self.inner.get_field(GapFillFlag::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_gap_fill_flag<'b: 'a>(&mut self, gap_fill_flag: GapFillFlag<'b>) {
        self.inner.to_mut().set_field(gap_fill_flag);
    }
        
}


