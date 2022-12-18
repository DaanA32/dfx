use std::borrow::Cow;

use dfx_core::message::Message;

use super::super::fields::*;

/// SecurityTypeRequest
#[derive(Clone, Debug)]
pub struct SecurityTypeRequest<'a> {
    inner: Cow<'a, Message>
}

impl<'a> SecurityTypeRequest<'a> {
    //TODO implement
    
    pub fn text<'b: 'a>(&'b self) -> Option<Text<'b>> {
        self.inner.get_field(Text::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_text<'b: 'a>(&mut self, text: Text<'b>) {
        self.inner.to_mut().set_field(text);
    }
        
    pub fn security_type<'b: 'a>(&'b self) -> Option<SecurityType<'b>> {
        self.inner.get_field(SecurityType::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_security_type<'b: 'a>(&mut self, security_type: SecurityType<'b>) {
        self.inner.to_mut().set_field(security_type);
    }
        
    pub fn security_req_id<'b: 'a>(&'b self) -> Option<SecurityReqId<'b>> {
        self.inner.get_field(SecurityReqId::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_security_req_id<'b: 'a>(&mut self, security_req_id: SecurityReqId<'b>) {
        self.inner.to_mut().set_field(security_req_id);
    }
        
    pub fn trading_session_id<'b: 'a>(&'b self) -> Option<TradingSessionId<'b>> {
        self.inner.get_field(TradingSessionId::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_trading_session_id<'b: 'a>(&mut self, trading_session_id: TradingSessionId<'b>) {
        self.inner.to_mut().set_field(trading_session_id);
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
        
    pub fn product<'b: 'a>(&'b self) -> Option<Product<'b>> {
        self.inner.get_field(Product::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_product<'b: 'a>(&mut self, product: Product<'b>) {
        self.inner.to_mut().set_field(product);
    }
        
    pub fn trading_session_sub_id<'b: 'a>(&'b self) -> Option<TradingSessionSubId<'b>> {
        self.inner.get_field(TradingSessionSubId::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_trading_session_sub_id<'b: 'a>(&mut self, trading_session_sub_id: TradingSessionSubId<'b>) {
        self.inner.to_mut().set_field(trading_session_sub_id);
    }
        
    pub fn security_sub_type<'b: 'a>(&'b self) -> Option<SecuritySubType<'b>> {
        self.inner.get_field(SecuritySubType::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_security_sub_type<'b: 'a>(&mut self, security_sub_type: SecuritySubType<'b>) {
        self.inner.to_mut().set_field(security_sub_type);
    }
        
}


