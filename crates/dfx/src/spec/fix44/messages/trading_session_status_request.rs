use std::borrow::Cow;

use dfx_core::message::Message;

use super::super::fields::*;

/// TradingSessionStatusRequest
#[derive(Clone, Debug)]
pub struct TradingSessionStatusRequest<'a> {
    inner: Cow<'a, Message>
}

impl<'a> TradingSessionStatusRequest<'a> {
    //TODO implement
    
    pub fn subscription_request_type<'b: 'a>(&'b self) -> Option<SubscriptionRequestType<'b>> {
        self.inner.get_field(SubscriptionRequestType::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_subscription_request_type<'b: 'a>(&mut self, subscription_request_type: SubscriptionRequestType<'b>) {
        self.inner.to_mut().set_field(subscription_request_type);
    }
        
    pub fn trad_ses_req_id<'b: 'a>(&'b self) -> Option<TradSesReqId<'b>> {
        self.inner.get_field(TradSesReqId::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_trad_ses_req_id<'b: 'a>(&mut self, trad_ses_req_id: TradSesReqId<'b>) {
        self.inner.to_mut().set_field(trad_ses_req_id);
    }
        
    pub fn trading_session_id<'b: 'a>(&'b self) -> Option<TradingSessionId<'b>> {
        self.inner.get_field(TradingSessionId::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_trading_session_id<'b: 'a>(&mut self, trading_session_id: TradingSessionId<'b>) {
        self.inner.to_mut().set_field(trading_session_id);
    }
        
    pub fn trad_ses_method<'b: 'a>(&'b self) -> Option<TradSesMethod<'b>> {
        self.inner.get_field(TradSesMethod::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_trad_ses_method<'b: 'a>(&mut self, trad_ses_method: TradSesMethod<'b>) {
        self.inner.to_mut().set_field(trad_ses_method);
    }
        
    pub fn trad_ses_mode<'b: 'a>(&'b self) -> Option<TradSesMode<'b>> {
        self.inner.get_field(TradSesMode::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_trad_ses_mode<'b: 'a>(&mut self, trad_ses_mode: TradSesMode<'b>) {
        self.inner.to_mut().set_field(trad_ses_mode);
    }
        
    pub fn trading_session_sub_id<'b: 'a>(&'b self) -> Option<TradingSessionSubId<'b>> {
        self.inner.get_field(TradingSessionSubId::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_trading_session_sub_id<'b: 'a>(&mut self, trading_session_sub_id: TradingSessionSubId<'b>) {
        self.inner.to_mut().set_field(trading_session_sub_id);
    }
        
}


