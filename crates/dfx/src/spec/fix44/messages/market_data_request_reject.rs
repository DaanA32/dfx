use std::borrow::Cow;

use dfx_core::message::Message;

use super::super::fields::*;

/// MarketDataRequestReject
#[derive(Clone, Debug)]
pub struct MarketDataRequestReject<'a> {
    inner: Cow<'a, Message>
}

impl<'a> MarketDataRequestReject<'a> {
    //TODO implement
    
    pub fn md_req_id<'b: 'a>(&'b self) -> Option<MdReqId<'b>> {
        self.inner.get_field(MdReqId::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_md_req_id<'b: 'a>(&mut self, md_req_id: MdReqId<'b>) {
        self.inner.to_mut().set_field(md_req_id);
    }
        
    pub fn md_req_rej_reason<'b: 'a>(&'b self) -> Option<MdReqRejReason<'b>> {
        self.inner.get_field(MdReqRejReason::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_md_req_rej_reason<'b: 'a>(&mut self, md_req_rej_reason: MdReqRejReason<'b>) {
        self.inner.to_mut().set_field(md_req_rej_reason);
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
        
    pub fn no_alt_md_source<'b: 'a>(&'b self) -> Option<NoAltMdSource<'b>> {
        self.inner.get_field(NoAltMdSource::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_no_alt_md_source<'b: 'a>(&mut self, no_alt_md_source: NoAltMdSource<'b>) {
        self.inner.to_mut().set_field(no_alt_md_source);
    }
        
    pub fn no_alt_md_source_group(&self) -> Option<NoAltMdSourceGroup> {
        todo!()
    }
    pub fn set_no_alt_md_source_group(&mut self, _no_alt_md_source_group: NoAltMdSourceGroup) {
        todo!()
    }
        
}


pub struct NoAltMdSourceGroup {

}

