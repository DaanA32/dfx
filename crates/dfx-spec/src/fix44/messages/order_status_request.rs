use std::borrow::Cow;

use dfx_core::message::Message;
use crate::fix44::fields::*;

/// OrderStatusRequest
#[derive(Clone, Debug)]
pub struct OrderStatusRequest<'a> {
    inner: Cow<'a, Message>
}

impl<'a> OrderStatusRequest<'a> {
    //TODO implement
    
    pub fn account<'b: 'a>(&'b self) -> Option<Account<'b>> {
        self.inner.get_field(Account::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_account<'b: 'a>(&mut self, account: Account<'b>) {
        self.inner.to_mut().set_field(account);
    }
        
    pub fn cl_ord_id<'b: 'a>(&'b self) -> Option<ClOrdId<'b>> {
        self.inner.get_field(ClOrdId::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_cl_ord_id<'b: 'a>(&mut self, cl_ord_id: ClOrdId<'b>) {
        self.inner.to_mut().set_field(cl_ord_id);
    }
        
    pub fn cl_ord_link_id<'b: 'a>(&'b self) -> Option<ClOrdLinkId<'b>> {
        self.inner.get_field(ClOrdLinkId::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_cl_ord_link_id<'b: 'a>(&mut self, cl_ord_link_id: ClOrdLinkId<'b>) {
        self.inner.to_mut().set_field(cl_ord_link_id);
    }
        
    pub fn acct_id_source<'b: 'a>(&'b self) -> Option<AcctIdSource<'b>> {
        self.inner.get_field(AcctIdSource::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_acct_id_source<'b: 'a>(&mut self, acct_id_source: AcctIdSource<'b>) {
        self.inner.to_mut().set_field(acct_id_source);
    }
        
    pub fn termination_type<'b: 'a>(&'b self) -> Option<TerminationType<'b>> {
        self.inner.get_field(TerminationType::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_termination_type<'b: 'a>(&mut self, termination_type: TerminationType<'b>) {
        self.inner.to_mut().set_field(termination_type);
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


