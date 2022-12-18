use std::borrow::Cow;

use dfx_core::message::Message;

use super::super::fields::*;

/// ConfirmationAck
#[derive(Clone, Debug)]
pub struct ConfirmationAck<'a> {
    inner: Cow<'a, Message>
}

impl<'a> ConfirmationAck<'a> {
    //TODO implement
    
    pub fn text<'b: 'a>(&'b self) -> Option<Text<'b>> {
        self.inner.get_field(Text::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_text<'b: 'a>(&mut self, text: Text<'b>) {
        self.inner.to_mut().set_field(text);
    }
        
    pub fn transact_time<'b: 'a>(&'b self) -> Option<TransactTime<'b>> {
        self.inner.get_field(TransactTime::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_transact_time<'b: 'a>(&mut self, transact_time: TransactTime<'b>) {
        self.inner.to_mut().set_field(transact_time);
    }
        
    pub fn trade_date<'b: 'a>(&'b self) -> Option<TradeDate<'b>> {
        self.inner.get_field(TradeDate::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_trade_date<'b: 'a>(&mut self, trade_date: TradeDate<'b>) {
        self.inner.to_mut().set_field(trade_date);
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
        
    pub fn match_status<'b: 'a>(&'b self) -> Option<MatchStatus<'b>> {
        self.inner.get_field(MatchStatus::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_match_status<'b: 'a>(&mut self, match_status: MatchStatus<'b>) {
        self.inner.to_mut().set_field(match_status);
    }
        
    pub fn confirm_id<'b: 'a>(&'b self) -> Option<ConfirmId<'b>> {
        self.inner.get_field(ConfirmId::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_confirm_id<'b: 'a>(&mut self, confirm_id: ConfirmId<'b>) {
        self.inner.to_mut().set_field(confirm_id);
    }
        
    pub fn confirm_rej_reason<'b: 'a>(&'b self) -> Option<ConfirmRejReason<'b>> {
        self.inner.get_field(ConfirmRejReason::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_confirm_rej_reason<'b: 'a>(&mut self, confirm_rej_reason: ConfirmRejReason<'b>) {
        self.inner.to_mut().set_field(confirm_rej_reason);
    }
        
    pub fn affirm_status<'b: 'a>(&'b self) -> Option<AffirmStatus<'b>> {
        self.inner.get_field(AffirmStatus::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_affirm_status<'b: 'a>(&mut self, affirm_status: AffirmStatus<'b>) {
        self.inner.to_mut().set_field(affirm_status);
    }
        
}


