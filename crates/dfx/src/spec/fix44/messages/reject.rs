use std::borrow::Cow;

use dfx_core::message::Message;

use super::super::fields::*;

/// Reject
#[derive(Clone, Debug)]
pub struct Reject<'a> {
    inner: Cow<'a, Message>
}

impl<'a> Reject<'a> {
    //TODO implement
    
    pub fn ref_seq_num<'b: 'a>(&'b self) -> Option<RefSeqNum<'b>> {
        self.inner.get_field(RefSeqNum::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_ref_seq_num<'b: 'a>(&mut self, ref_seq_num: RefSeqNum<'b>) {
        self.inner.to_mut().set_field(ref_seq_num);
    }
        
    pub fn text<'b: 'a>(&'b self) -> Option<Text<'b>> {
        self.inner.get_field(Text::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_text<'b: 'a>(&mut self, text: Text<'b>) {
        self.inner.to_mut().set_field(text);
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
        
    pub fn ref_tag_id<'b: 'a>(&'b self) -> Option<RefTagId<'b>> {
        self.inner.get_field(RefTagId::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_ref_tag_id<'b: 'a>(&mut self, ref_tag_id: RefTagId<'b>) {
        self.inner.to_mut().set_field(ref_tag_id);
    }
        
    pub fn ref_msg_type<'b: 'a>(&'b self) -> Option<RefMsgType<'b>> {
        self.inner.get_field(RefMsgType::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_ref_msg_type<'b: 'a>(&mut self, ref_msg_type: RefMsgType<'b>) {
        self.inner.to_mut().set_field(ref_msg_type);
    }
        
    pub fn session_reject_reason<'b: 'a>(&'b self) -> Option<SessionRejectReason<'b>> {
        self.inner.get_field(SessionRejectReason::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_session_reject_reason<'b: 'a>(&mut self, session_reject_reason: SessionRejectReason<'b>) {
        self.inner.to_mut().set_field(session_reject_reason);
    }
        
}


