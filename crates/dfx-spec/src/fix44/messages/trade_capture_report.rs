use std::borrow::Cow;

use dfx_core::message::Message;

use super::super::fields::*;

/// TradeCaptureReport
#[derive(Clone, Debug)]
pub struct TradeCaptureReport<'a> {
    inner: Cow<'a, Message>
}

impl<'a> TradeCaptureReport<'a> {
    //TODO implement
    
    pub fn avg_px<'b: 'a>(&'b self) -> Option<AvgPx<'b>> {
        self.inner.get_field(AvgPx::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_avg_px<'b: 'a>(&mut self, avg_px: AvgPx<'b>) {
        self.inner.to_mut().set_field(avg_px);
    }
        
    pub fn exec_id<'b: 'a>(&'b self) -> Option<ExecId<'b>> {
        self.inner.get_field(ExecId::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_exec_id<'b: 'a>(&mut self, exec_id: ExecId<'b>) {
        self.inner.to_mut().set_field(exec_id);
    }
        
    pub fn exec_type<'b: 'a>(&'b self) -> Option<ExecType<'b>> {
        self.inner.get_field(ExecType::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_exec_type<'b: 'a>(&mut self, exec_type: ExecType<'b>) {
        self.inner.to_mut().set_field(exec_type);
    }
        
    pub fn exec_restatement_reason<'b: 'a>(&'b self) -> Option<ExecRestatementReason<'b>> {
        self.inner.get_field(ExecRestatementReason::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_exec_restatement_reason<'b: 'a>(&mut self, exec_restatement_reason: ExecRestatementReason<'b>) {
        self.inner.to_mut().set_field(exec_restatement_reason);
    }
        
    pub fn clearing_business_date<'b: 'a>(&'b self) -> Option<ClearingBusinessDate<'b>> {
        self.inner.get_field(ClearingBusinessDate::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_clearing_business_date<'b: 'a>(&mut self, clearing_business_date: ClearingBusinessDate<'b>) {
        self.inner.to_mut().set_field(clearing_business_date);
    }
        
    pub fn termination_type<'b: 'a>(&'b self) -> Option<TerminationType<'b>> {
        self.inner.get_field(TerminationType::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_termination_type<'b: 'a>(&mut self, termination_type: TerminationType<'b>) {
        self.inner.to_mut().set_field(termination_type);
    }
        
    pub fn copy_msg_indicator<'b: 'a>(&'b self) -> Option<CopyMsgIndicator<'b>> {
        self.inner.get_field(CopyMsgIndicator::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_copy_msg_indicator<'b: 'a>(&mut self, copy_msg_indicator: CopyMsgIndicator<'b>) {
        self.inner.to_mut().set_field(copy_msg_indicator);
    }
        
    pub fn avg_px_indicator<'b: 'a>(&'b self) -> Option<AvgPxIndicator<'b>> {
        self.inner.get_field(AvgPxIndicator::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_avg_px_indicator<'b: 'a>(&mut self, avg_px_indicator: AvgPxIndicator<'b>) {
        self.inner.to_mut().set_field(avg_px_indicator);
    }
        
    pub fn margin_ratio<'b: 'a>(&'b self) -> Option<MarginRatio<'b>> {
        self.inner.get_field(MarginRatio::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_margin_ratio<'b: 'a>(&mut self, margin_ratio: MarginRatio<'b>) {
        self.inner.to_mut().set_field(margin_ratio);
    }
        
    pub fn agreement_desc<'b: 'a>(&'b self) -> Option<AgreementDesc<'b>> {
        self.inner.get_field(AgreementDesc::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_agreement_desc<'b: 'a>(&mut self, agreement_desc: AgreementDesc<'b>) {
        self.inner.to_mut().set_field(agreement_desc);
    }
        
    pub fn agreement_id<'b: 'a>(&'b self) -> Option<AgreementId<'b>> {
        self.inner.get_field(AgreementId::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_agreement_id<'b: 'a>(&mut self, agreement_id: AgreementId<'b>) {
        self.inner.to_mut().set_field(agreement_id);
    }
        
    pub fn agreement_date<'b: 'a>(&'b self) -> Option<AgreementDate<'b>> {
        self.inner.get_field(AgreementDate::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_agreement_date<'b: 'a>(&mut self, agreement_date: AgreementDate<'b>) {
        self.inner.to_mut().set_field(agreement_date);
    }
        
    pub fn start_date<'b: 'a>(&'b self) -> Option<StartDate<'b>> {
        self.inner.get_field(StartDate::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_start_date<'b: 'a>(&mut self, start_date: StartDate<'b>) {
        self.inner.to_mut().set_field(start_date);
    }
        
    pub fn end_date<'b: 'a>(&'b self) -> Option<EndDate<'b>> {
        self.inner.get_field(EndDate::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_end_date<'b: 'a>(&mut self, end_date: EndDate<'b>) {
        self.inner.to_mut().set_field(end_date);
    }
        
    pub fn agreement_currency<'b: 'a>(&'b self) -> Option<AgreementCurrency<'b>> {
        self.inner.get_field(AgreementCurrency::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_agreement_currency<'b: 'a>(&mut self, agreement_currency: AgreementCurrency<'b>) {
        self.inner.to_mut().set_field(agreement_currency);
    }
        
    pub fn delivery_type<'b: 'a>(&'b self) -> Option<DeliveryType<'b>> {
        self.inner.get_field(DeliveryType::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_delivery_type<'b: 'a>(&mut self, delivery_type: DeliveryType<'b>) {
        self.inner.to_mut().set_field(delivery_type);
    }
        
}


