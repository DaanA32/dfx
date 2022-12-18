use std::borrow::Cow;

use dfx_core::message::Message;

use super::super::fields::*;

/// BusinessMessageReject
#[derive(Clone, Debug)]
pub struct BusinessMessageReject<'a> {
    inner: Cow<'a, Message>
}

impl<'a> BusinessMessageReject<'a> {
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
        
    pub fn ref_msg_type<'b: 'a>(&'b self) -> Option<RefMsgType<'b>> {
        self.inner.get_field(RefMsgType::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_ref_msg_type<'b: 'a>(&mut self, ref_msg_type: RefMsgType<'b>) {
        self.inner.to_mut().set_field(ref_msg_type);
    }
        
    pub fn business_reject_ref_id<'b: 'a>(&'b self) -> Option<BusinessRejectRefId<'b>> {
        self.inner.get_field(BusinessRejectRefId::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_business_reject_ref_id<'b: 'a>(&mut self, business_reject_ref_id: BusinessRejectRefId<'b>) {
        self.inner.to_mut().set_field(business_reject_ref_id);
    }
        
    pub fn business_reject_reason<'b: 'a>(&'b self) -> Option<BusinessRejectReason<'b>> {
        self.inner.get_field(BusinessRejectReason::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_business_reject_reason<'b: 'a>(&mut self, business_reject_reason: BusinessRejectReason<'b>) {
        self.inner.to_mut().set_field(business_reject_reason);
    }
        
}


