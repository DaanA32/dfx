use std::borrow::Cow;

use dfx_core::message::Message;
use crate::fix44::fields::*;

/// QuoteResponse
#[derive(Clone, Debug)]
pub struct QuoteResponse<'a> {
    inner: Cow<'a, Message>
}

impl<'a> QuoteResponse<'a> {
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
        
    pub fn commission<'b: 'a>(&'b self) -> Option<Commission<'b>> {
        self.inner.get_field(Commission::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_commission<'b: 'a>(&mut self, commission: Commission<'b>) {
        self.inner.to_mut().set_field(commission);
    }
        
    pub fn comm_type<'b: 'a>(&'b self) -> Option<CommType<'b>> {
        self.inner.get_field(CommType::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_comm_type<'b: 'a>(&mut self, comm_type: CommType<'b>) {
        self.inner.to_mut().set_field(comm_type);
    }
        
    pub fn currency<'b: 'a>(&'b self) -> Option<Currency<'b>> {
        self.inner.get_field(Currency::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_currency<'b: 'a>(&mut self, currency: Currency<'b>) {
        self.inner.to_mut().set_field(currency);
    }
        
    pub fn ex_destination<'b: 'a>(&'b self) -> Option<ExDestination<'b>> {
        self.inner.get_field(ExDestination::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_ex_destination<'b: 'a>(&mut self, ex_destination: ExDestination<'b>) {
        self.inner.to_mut().set_field(ex_destination);
    }
        
    pub fn bid_px<'b: 'a>(&'b self) -> Option<BidPx<'b>> {
        self.inner.get_field(BidPx::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_bid_px<'b: 'a>(&mut self, bid_px: BidPx<'b>) {
        self.inner.to_mut().set_field(bid_px);
    }
        
    pub fn bid_size<'b: 'a>(&'b self) -> Option<BidSize<'b>> {
        self.inner.get_field(BidSize::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_bid_size<'b: 'a>(&mut self, bid_size: BidSize<'b>) {
        self.inner.to_mut().set_field(bid_size);
    }
        
    pub fn bid_spot_rate<'b: 'a>(&'b self) -> Option<BidSpotRate<'b>> {
        self.inner.get_field(BidSpotRate::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_bid_spot_rate<'b: 'a>(&mut self, bid_spot_rate: BidSpotRate<'b>) {
        self.inner.to_mut().set_field(bid_spot_rate);
    }
        
    pub fn bid_forward_points<'b: 'a>(&'b self) -> Option<BidForwardPoints<'b>> {
        self.inner.get_field(BidForwardPoints::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_bid_forward_points<'b: 'a>(&mut self, bid_forward_points: BidForwardPoints<'b>) {
        self.inner.to_mut().set_field(bid_forward_points);
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
        
    pub fn account_type<'b: 'a>(&'b self) -> Option<AccountType<'b>> {
        self.inner.get_field(AccountType::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_account_type<'b: 'a>(&mut self, account_type: AccountType<'b>) {
        self.inner.to_mut().set_field(account_type);
    }
        
    pub fn cust_order_capacity<'b: 'a>(&'b self) -> Option<CustOrderCapacity<'b>> {
        self.inner.get_field(CustOrderCapacity::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_cust_order_capacity<'b: 'a>(&mut self, cust_order_capacity: CustOrderCapacity<'b>) {
        self.inner.to_mut().set_field(cust_order_capacity);
    }
        
    pub fn bid_yield<'b: 'a>(&'b self) -> Option<BidYield<'b>> {
        self.inner.get_field(BidYield::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_bid_yield<'b: 'a>(&mut self, bid_yield: BidYield<'b>) {
        self.inner.to_mut().set_field(bid_yield);
    }
        
    pub fn bid_forward_points2<'b: 'a>(&'b self) -> Option<BidForwardPoints2<'b>> {
        self.inner.get_field(BidForwardPoints2::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_bid_forward_points2<'b: 'a>(&mut self, bid_forward_points2: BidForwardPoints2<'b>) {
        self.inner.to_mut().set_field(bid_forward_points2);
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


