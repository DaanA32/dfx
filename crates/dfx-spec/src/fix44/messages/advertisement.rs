use std::borrow::Cow;

use dfx_core::message::Message;
use crate::fix44::fields::*;

/// Advertisement
#[derive(Clone, Debug)]
pub struct Advertisement<'a> {
    inner: Cow<'a, Message>
}

impl<'a> Advertisement<'a> {
    //TODO implement
    
    pub fn adv_id<'b: 'a>(&'b self) -> Option<AdvId<'b>> {
        self.inner.get_field(AdvId::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_adv_id<'b: 'a>(&mut self, adv_id: AdvId<'b>) {
        self.inner.to_mut().set_field(adv_id);
    }
        
    pub fn adv_ref_id<'b: 'a>(&'b self) -> Option<AdvRefId<'b>> {
        self.inner.get_field(AdvRefId::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_adv_ref_id<'b: 'a>(&mut self, adv_ref_id: AdvRefId<'b>) {
        self.inner.to_mut().set_field(adv_ref_id);
    }
        
    pub fn adv_side<'b: 'a>(&'b self) -> Option<AdvSide<'b>> {
        self.inner.get_field(AdvSide::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_adv_side<'b: 'a>(&mut self, adv_side: AdvSide<'b>) {
        self.inner.to_mut().set_field(adv_side);
    }
        
    pub fn adv_trans_type<'b: 'a>(&'b self) -> Option<AdvTransType<'b>> {
        self.inner.get_field(AdvTransType::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_adv_trans_type<'b: 'a>(&mut self, adv_trans_type: AdvTransType<'b>) {
        self.inner.to_mut().set_field(adv_trans_type);
    }
        
    pub fn currency<'b: 'a>(&'b self) -> Option<Currency<'b>> {
        self.inner.get_field(Currency::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_currency<'b: 'a>(&mut self, currency: Currency<'b>) {
        self.inner.to_mut().set_field(currency);
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

