use std::borrow::Cow;

use dfx_core::message::Message;
use crate::fix44::fields::*;

/// DerivativeSecurityListRequest
#[derive(Clone, Debug)]
pub struct DerivativeSecurityListRequest<'a> {
    inner: Cow<'a, Message>
}

impl<'a> DerivativeSecurityListRequest<'a> {
    //TODO implement
    
    pub fn currency<'b: 'a>(&'b self) -> Option<Currency<'b>> {
        self.inner.get_field(Currency::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_currency<'b: 'a>(&mut self, currency: Currency<'b>) {
        self.inner.to_mut().set_field(currency);
    }
        
    pub fn text<'b: 'a>(&'b self) -> Option<Text<'b>> {
        self.inner.get_field(Text::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_text<'b: 'a>(&mut self, text: Text<'b>) {
        self.inner.to_mut().set_field(text);
    }
        
    pub fn subscription_request_type<'b: 'a>(&'b self) -> Option<SubscriptionRequestType<'b>> {
        self.inner.get_field(SubscriptionRequestType::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_subscription_request_type<'b: 'a>(&mut self, subscription_request_type: SubscriptionRequestType<'b>) {
        self.inner.to_mut().set_field(subscription_request_type);
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
        
    pub fn encoded_underlying_issuer_len<'b: 'a>(&'b self) -> Option<EncodedUnderlyingIssuerLen<'b>> {
        self.inner.get_field(EncodedUnderlyingIssuerLen::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_encoded_underlying_issuer_len<'b: 'a>(&mut self, encoded_underlying_issuer_len: EncodedUnderlyingIssuerLen<'b>) {
        self.inner.to_mut().set_field(encoded_underlying_issuer_len);
    }
        
    pub fn encoded_underlying_issuer<'b: 'a>(&'b self) -> Option<EncodedUnderlyingIssuer<'b>> {
        self.inner.get_field(EncodedUnderlyingIssuer::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_encoded_underlying_issuer<'b: 'a>(&mut self, encoded_underlying_issuer: EncodedUnderlyingIssuer<'b>) {
        self.inner.to_mut().set_field(encoded_underlying_issuer);
    }
        
    pub fn encoded_underlying_security_desc_len<'b: 'a>(&'b self) -> Option<EncodedUnderlyingSecurityDescLen<'b>> {
        self.inner.get_field(EncodedUnderlyingSecurityDescLen::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_encoded_underlying_security_desc_len<'b: 'a>(&mut self, encoded_underlying_security_desc_len: EncodedUnderlyingSecurityDescLen<'b>) {
        self.inner.to_mut().set_field(encoded_underlying_security_desc_len);
    }
        
    pub fn encoded_underlying_security_desc<'b: 'a>(&'b self) -> Option<EncodedUnderlyingSecurityDesc<'b>> {
        self.inner.get_field(EncodedUnderlyingSecurityDesc::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_encoded_underlying_security_desc<'b: 'a>(&mut self, encoded_underlying_security_desc: EncodedUnderlyingSecurityDesc<'b>) {
        self.inner.to_mut().set_field(encoded_underlying_security_desc);
    }
        
    pub fn no_underlying_security_alt_id<'b: 'a>(&'b self) -> Option<NoUnderlyingSecurityAltId<'b>> {
        self.inner.get_field(NoUnderlyingSecurityAltId::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_no_underlying_security_alt_id<'b: 'a>(&mut self, no_underlying_security_alt_id: NoUnderlyingSecurityAltId<'b>) {
        self.inner.to_mut().set_field(no_underlying_security_alt_id);
    }
        
    pub fn security_list_request_type<'b: 'a>(&'b self) -> Option<SecurityListRequestType<'b>> {
        self.inner.get_field(SecurityListRequestType::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_security_list_request_type<'b: 'a>(&mut self, security_list_request_type: SecurityListRequestType<'b>) {
        self.inner.to_mut().set_field(security_list_request_type);
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
        
    pub fn no_underlying_security_alt_id_group(&self) -> Option<NoUnderlyingSecurityAltIdGroup> {
        todo!()
    }
    pub fn set_no_underlying_security_alt_id_group(&mut self, _no_underlying_security_alt_id_group: NoUnderlyingSecurityAltIdGroup) {
        todo!()
    }
        
}


pub struct NoUnderlyingSecurityAltIdGroup {

}

