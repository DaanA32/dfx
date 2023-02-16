use std::borrow::Cow;

use dfx_core::message::Message;
use crate::fix44::fields::*;

/// TradingSessionStatus
#[derive(Clone, Debug)]
pub struct TradingSessionStatus<'a> {
    inner: Cow<'a, Message>
}

impl<'a> TradingSessionStatus<'a> {
    //TODO implement
    
    pub fn text<'b: 'a>(&'b self) -> Option<Text<'b>> {
        self.inner.get_field(Text::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_text<'b: 'a>(&mut self, text: Text<'b>) {
        self.inner.to_mut().set_field(text);
    }
        
    pub fn unsolicited_indicator<'b: 'a>(&'b self) -> Option<UnsolicitedIndicator<'b>> {
        self.inner.get_field(UnsolicitedIndicator::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_unsolicited_indicator<'b: 'a>(&mut self, unsolicited_indicator: UnsolicitedIndicator<'b>) {
        self.inner.to_mut().set_field(unsolicited_indicator);
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
        
    pub fn trad_ses_status<'b: 'a>(&'b self) -> Option<TradSesStatus<'b>> {
        self.inner.get_field(TradSesStatus::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_trad_ses_status<'b: 'a>(&mut self, trad_ses_status: TradSesStatus<'b>) {
        self.inner.to_mut().set_field(trad_ses_status);
    }
        
    pub fn trad_ses_start_time<'b: 'a>(&'b self) -> Option<TradSesStartTime<'b>> {
        self.inner.get_field(TradSesStartTime::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_trad_ses_start_time<'b: 'a>(&mut self, trad_ses_start_time: TradSesStartTime<'b>) {
        self.inner.to_mut().set_field(trad_ses_start_time);
    }
        
    pub fn trad_ses_open_time<'b: 'a>(&'b self) -> Option<TradSesOpenTime<'b>> {
        self.inner.get_field(TradSesOpenTime::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_trad_ses_open_time<'b: 'a>(&mut self, trad_ses_open_time: TradSesOpenTime<'b>) {
        self.inner.to_mut().set_field(trad_ses_open_time);
    }
        
    pub fn trad_ses_pre_close_time<'b: 'a>(&'b self) -> Option<TradSesPreCloseTime<'b>> {
        self.inner.get_field(TradSesPreCloseTime::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_trad_ses_pre_close_time<'b: 'a>(&mut self, trad_ses_pre_close_time: TradSesPreCloseTime<'b>) {
        self.inner.to_mut().set_field(trad_ses_pre_close_time);
    }
        
    pub fn trad_ses_close_time<'b: 'a>(&'b self) -> Option<TradSesCloseTime<'b>> {
        self.inner.get_field(TradSesCloseTime::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_trad_ses_close_time<'b: 'a>(&mut self, trad_ses_close_time: TradSesCloseTime<'b>) {
        self.inner.to_mut().set_field(trad_ses_close_time);
    }
        
    pub fn trad_ses_end_time<'b: 'a>(&'b self) -> Option<TradSesEndTime<'b>> {
        self.inner.get_field(TradSesEndTime::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_trad_ses_end_time<'b: 'a>(&mut self, trad_ses_end_time: TradSesEndTime<'b>) {
        self.inner.to_mut().set_field(trad_ses_end_time);
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
        
    pub fn total_volume_traded<'b: 'a>(&'b self) -> Option<TotalVolumeTraded<'b>> {
        self.inner.get_field(TotalVolumeTraded::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_total_volume_traded<'b: 'a>(&mut self, total_volume_traded: TotalVolumeTraded<'b>) {
        self.inner.to_mut().set_field(total_volume_traded);
    }
        
    pub fn trad_ses_status_rej_reason<'b: 'a>(&'b self) -> Option<TradSesStatusRejReason<'b>> {
        self.inner.get_field(TradSesStatusRejReason::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_trad_ses_status_rej_reason<'b: 'a>(&mut self, trad_ses_status_rej_reason: TradSesStatusRejReason<'b>) {
        self.inner.to_mut().set_field(trad_ses_status_rej_reason);
    }
        
    pub fn trading_session_sub_id<'b: 'a>(&'b self) -> Option<TradingSessionSubId<'b>> {
        self.inner.get_field(TradingSessionSubId::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_trading_session_sub_id<'b: 'a>(&mut self, trading_session_sub_id: TradingSessionSubId<'b>) {
        self.inner.to_mut().set_field(trading_session_sub_id);
    }
        
}


