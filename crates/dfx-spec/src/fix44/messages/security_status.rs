use std::borrow::Cow;

use dfx_core::message::Message;
use crate::fix44::fields::*;

/// SecurityStatus
#[derive(Clone, Debug)]
pub struct SecurityStatus<'a> {
    inner: Cow<'a, Message>
}

impl<'a> SecurityStatus<'a> {
    //TODO implement
    
    pub fn currency<'b: 'a>(&'b self) -> Option<Currency<'b>> {
        self.inner.get_field(Currency::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_currency<'b: 'a>(&mut self, currency: Currency<'b>) {
        self.inner.to_mut().set_field(currency);
    }
        
    pub fn financial_status<'b: 'a>(&'b self) -> Option<FinancialStatus<'b>> {
        self.inner.get_field(FinancialStatus::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_financial_status<'b: 'a>(&mut self, financial_status: FinancialStatus<'b>) {
        self.inner.to_mut().set_field(financial_status);
    }
        
    pub fn corporate_action<'b: 'a>(&'b self) -> Option<CorporateAction<'b>> {
        self.inner.get_field(CorporateAction::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_corporate_action<'b: 'a>(&mut self, corporate_action: CorporateAction<'b>) {
        self.inner.to_mut().set_field(corporate_action);
    }
        
    pub fn halt_reason_char<'b: 'a>(&'b self) -> Option<HaltReasonChar<'b>> {
        self.inner.get_field(HaltReasonChar::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_halt_reason_char<'b: 'a>(&mut self, halt_reason_char: HaltReasonChar<'b>) {
        self.inner.to_mut().set_field(halt_reason_char);
    }
        
    pub fn in_view_of_common<'b: 'a>(&'b self) -> Option<InViewOfCommon<'b>> {
        self.inner.get_field(InViewOfCommon::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_in_view_of_common<'b: 'a>(&mut self, in_view_of_common: InViewOfCommon<'b>) {
        self.inner.to_mut().set_field(in_view_of_common);
    }
        
    pub fn due_to_related<'b: 'a>(&'b self) -> Option<DueToRelated<'b>> {
        self.inner.get_field(DueToRelated::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_due_to_related<'b: 'a>(&mut self, due_to_related: DueToRelated<'b>) {
        self.inner.to_mut().set_field(due_to_related);
    }
        
    pub fn buy_volume<'b: 'a>(&'b self) -> Option<BuyVolume<'b>> {
        self.inner.get_field(BuyVolume::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_buy_volume<'b: 'a>(&mut self, buy_volume: BuyVolume<'b>) {
        self.inner.to_mut().set_field(buy_volume);
    }
        
    pub fn high_px<'b: 'a>(&'b self) -> Option<HighPx<'b>> {
        self.inner.get_field(HighPx::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_high_px<'b: 'a>(&mut self, high_px: HighPx<'b>) {
        self.inner.to_mut().set_field(high_px);
    }
        
    pub fn adjustment<'b: 'a>(&'b self) -> Option<Adjustment<'b>> {
        self.inner.get_field(Adjustment::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_adjustment<'b: 'a>(&mut self, adjustment: Adjustment<'b>) {
        self.inner.to_mut().set_field(adjustment);
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

