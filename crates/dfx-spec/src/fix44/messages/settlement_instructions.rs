use std::borrow::Cow;

use dfx_core::message::Message;
use crate::fix44::fields::*;

/// SettlementInstructions
#[derive(Clone, Debug)]
pub struct SettlementInstructions<'a> {
    inner: Cow<'a, Message>
}

impl<'a> SettlementInstructions<'a> {
    //TODO implement
    
    pub fn cl_ord_id<'b: 'a>(&'b self) -> Option<ClOrdId<'b>> {
        self.inner.get_field(ClOrdId::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_cl_ord_id<'b: 'a>(&mut self, cl_ord_id: ClOrdId<'b>) {
        self.inner.to_mut().set_field(cl_ord_id);
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
        
    pub fn no_settl_inst<'b: 'a>(&'b self) -> Option<NoSettlInst<'b>> {
        self.inner.get_field(NoSettlInst::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_no_settl_inst<'b: 'a>(&mut self, no_settl_inst: NoSettlInst<'b>) {
        self.inner.to_mut().set_field(no_settl_inst);
    }
        
    pub fn no_settl_inst_group(&self) -> Option<NoSettlInstGroup> {
        todo!()
    }
    pub fn set_no_settl_inst_group(&mut self, _no_settl_inst_group: NoSettlInstGroup) {
        todo!()
    }
        
}


pub struct NoSettlInstGroup {

}

