use std::borrow::Cow;

use dfx_core::message::Message;

use super::super::fields::*;

/// ListCancelRequest
#[derive(Clone, Debug)]
pub struct ListCancelRequest<'a> {
    inner: Cow<'a, Message>
}

impl<'a> ListCancelRequest<'a> {
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
        
    pub fn list_id<'b: 'a>(&'b self) -> Option<ListId<'b>> {
        self.inner.get_field(ListId::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_list_id<'b: 'a>(&mut self, list_id: ListId<'b>) {
        self.inner.to_mut().set_field(list_id);
    }
        
    pub fn trade_date<'b: 'a>(&'b self) -> Option<TradeDate<'b>> {
        self.inner.get_field(TradeDate::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_trade_date<'b: 'a>(&mut self, trade_date: TradeDate<'b>) {
        self.inner.to_mut().set_field(trade_date);
    }
        
    pub fn trade_origination_date<'b: 'a>(&'b self) -> Option<TradeOriginationDate<'b>> {
        self.inner.get_field(TradeOriginationDate::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_trade_origination_date<'b: 'a>(&mut self, trade_origination_date: TradeOriginationDate<'b>) {
        self.inner.to_mut().set_field(trade_origination_date);
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
        
}


