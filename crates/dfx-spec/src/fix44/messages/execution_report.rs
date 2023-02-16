use std::borrow::Cow;

use dfx_core::message::Message;
use crate::fix44::fields::*;

/// ExecutionReport
#[derive(Clone, Debug)]
pub struct ExecutionReport<'a> {
    inner: Cow<'a, Message>
}

impl<'a> ExecutionReport<'a> {
    //TODO implement
    
    pub fn account<'b: 'a>(&'b self) -> Option<Account<'b>> {
        self.inner.get_field(Account::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_account<'b: 'a>(&mut self, account: Account<'b>) {
        self.inner.to_mut().set_field(account);
    }
        
    pub fn avg_px<'b: 'a>(&'b self) -> Option<AvgPx<'b>> {
        self.inner.get_field(AvgPx::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_avg_px<'b: 'a>(&mut self, avg_px: AvgPx<'b>) {
        self.inner.to_mut().set_field(avg_px);
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
        
    pub fn accrued_interest_rate<'b: 'a>(&'b self) -> Option<AccruedInterestRate<'b>> {
        self.inner.get_field(AccruedInterestRate::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_accrued_interest_rate<'b: 'a>(&mut self, accrued_interest_rate: AccruedInterestRate<'b>) {
        self.inner.to_mut().set_field(accrued_interest_rate);
    }
        
    pub fn accrued_interest_amt<'b: 'a>(&'b self) -> Option<AccruedInterestAmt<'b>> {
        self.inner.get_field(AccruedInterestAmt::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_accrued_interest_amt<'b: 'a>(&mut self, accrued_interest_amt: AccruedInterestAmt<'b>) {
        self.inner.to_mut().set_field(accrued_interest_amt);
    }
        
    pub fn basis_feature_date<'b: 'a>(&'b self) -> Option<BasisFeatureDate<'b>> {
        self.inner.get_field(BasisFeatureDate::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_basis_feature_date<'b: 'a>(&mut self, basis_feature_date: BasisFeatureDate<'b>) {
        self.inner.to_mut().set_field(basis_feature_date);
    }
        
    pub fn basis_feature_price<'b: 'a>(&'b self) -> Option<BasisFeaturePrice<'b>> {
        self.inner.get_field(BasisFeaturePrice::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_basis_feature_price<'b: 'a>(&mut self, basis_feature_price: BasisFeaturePrice<'b>) {
        self.inner.to_mut().set_field(basis_feature_price);
    }
        
    pub fn comm_currency<'b: 'a>(&'b self) -> Option<CommCurrency<'b>> {
        self.inner.get_field(CommCurrency::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_comm_currency<'b: 'a>(&mut self, comm_currency: CommCurrency<'b>) {
        self.inner.to_mut().set_field(comm_currency);
    }
        
    pub fn cancellation_rights<'b: 'a>(&'b self) -> Option<CancellationRights<'b>> {
        self.inner.get_field(CancellationRights::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_cancellation_rights<'b: 'a>(&mut self, cancellation_rights: CancellationRights<'b>) {
        self.inner.to_mut().set_field(cancellation_rights);
    }
        
    pub fn fund_renew_waiv<'b: 'a>(&'b self) -> Option<FundRenewWaiv<'b>> {
        self.inner.get_field(FundRenewWaiv::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_fund_renew_waiv<'b: 'a>(&mut self, fund_renew_waiv: FundRenewWaiv<'b>) {
        self.inner.to_mut().set_field(fund_renew_waiv);
    }
        
    pub fn cash_margin<'b: 'a>(&'b self) -> Option<CashMargin<'b>> {
        self.inner.get_field(CashMargin::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_cash_margin<'b: 'a>(&mut self, cash_margin: CashMargin<'b>) {
        self.inner.to_mut().set_field(cash_margin);
    }
        
    pub fn account_type<'b: 'a>(&'b self) -> Option<AccountType<'b>> {
        self.inner.get_field(AccountType::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_account_type<'b: 'a>(&mut self, account_type: AccountType<'b>) {
        self.inner.to_mut().set_field(account_type);
    }
        
    pub fn cl_ord_link_id<'b: 'a>(&'b self) -> Option<ClOrdLinkId<'b>> {
        self.inner.get_field(ClOrdLinkId::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_cl_ord_link_id<'b: 'a>(&mut self, cl_ord_link_id: ClOrdLinkId<'b>) {
        self.inner.to_mut().set_field(cl_ord_link_id);
    }
        
    pub fn booking_unit<'b: 'a>(&'b self) -> Option<BookingUnit<'b>> {
        self.inner.get_field(BookingUnit::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_booking_unit<'b: 'a>(&mut self, booking_unit: BookingUnit<'b>) {
        self.inner.to_mut().set_field(booking_unit);
    }
        
    pub fn clearing_fee_indicator<'b: 'a>(&'b self) -> Option<ClearingFeeIndicator<'b>> {
        self.inner.get_field(ClearingFeeIndicator::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_clearing_fee_indicator<'b: 'a>(&mut self, clearing_fee_indicator: ClearingFeeIndicator<'b>) {
        self.inner.to_mut().set_field(clearing_fee_indicator);
    }
        
    pub fn acct_id_source<'b: 'a>(&'b self) -> Option<AcctIdSource<'b>> {
        self.inner.get_field(AcctIdSource::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_acct_id_source<'b: 'a>(&mut self, acct_id_source: AcctIdSource<'b>) {
        self.inner.to_mut().set_field(acct_id_source);
    }
        
    pub fn booking_type<'b: 'a>(&'b self) -> Option<BookingType<'b>> {
        self.inner.get_field(BookingType::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_booking_type<'b: 'a>(&mut self, booking_type: BookingType<'b>) {
        self.inner.to_mut().set_field(booking_type);
    }
        
}


