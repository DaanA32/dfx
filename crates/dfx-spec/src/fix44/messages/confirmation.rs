use std::borrow::Cow;

use dfx_core::message::Message;

use super::super::fields::*;

/// Confirmation
#[derive(Clone, Debug)]
pub struct Confirmation<'a> {
    inner: Cow<'a, Message>
}

impl<'a> Confirmation<'a> {
    //TODO implement
    
    pub fn avg_px<'b: 'a>(&'b self) -> Option<AvgPx<'b>> {
        self.inner.get_field(AvgPx::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_avg_px<'b: 'a>(&mut self, avg_px: AvgPx<'b>) {
        self.inner.to_mut().set_field(avg_px);
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
        
    pub fn alloc_id<'b: 'a>(&'b self) -> Option<AllocId<'b>> {
        self.inner.get_field(AllocId::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_alloc_id<'b: 'a>(&mut self, alloc_id: AllocId<'b>) {
        self.inner.to_mut().set_field(alloc_id);
    }
        
    pub fn avg_px_precision<'b: 'a>(&'b self) -> Option<AvgPxPrecision<'b>> {
        self.inner.get_field(AvgPxPrecision::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_avg_px_precision<'b: 'a>(&mut self, avg_px_precision: AvgPxPrecision<'b>) {
        self.inner.to_mut().set_field(avg_px_precision);
    }
        
    pub fn alloc_account<'b: 'a>(&'b self) -> Option<AllocAccount<'b>> {
        self.inner.get_field(AllocAccount::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_alloc_account<'b: 'a>(&mut self, alloc_account: AllocAccount<'b>) {
        self.inner.to_mut().set_field(alloc_account);
    }
        
    pub fn alloc_qty<'b: 'a>(&'b self) -> Option<AllocQty<'b>> {
        self.inner.get_field(AllocQty::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_alloc_qty<'b: 'a>(&mut self, alloc_qty: AllocQty<'b>) {
        self.inner.to_mut().set_field(alloc_qty);
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
        
    pub fn comm_currency<'b: 'a>(&'b self) -> Option<CommCurrency<'b>> {
        self.inner.get_field(CommCurrency::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_comm_currency<'b: 'a>(&mut self, comm_currency: CommCurrency<'b>) {
        self.inner.to_mut().set_field(comm_currency);
    }
        
    pub fn fund_renew_waiv<'b: 'a>(&'b self) -> Option<FundRenewWaiv<'b>> {
        self.inner.get_field(FundRenewWaiv::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_fund_renew_waiv<'b: 'a>(&mut self, fund_renew_waiv: FundRenewWaiv<'b>) {
        self.inner.to_mut().set_field(fund_renew_waiv);
    }
        
    pub fn alloc_acct_id_source<'b: 'a>(&'b self) -> Option<AllocAcctIdSource<'b>> {
        self.inner.get_field(AllocAcctIdSource::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_alloc_acct_id_source<'b: 'a>(&mut self, alloc_acct_id_source: AllocAcctIdSource<'b>) {
        self.inner.to_mut().set_field(alloc_acct_id_source);
    }
        
    pub fn alloc_account_type<'b: 'a>(&'b self) -> Option<AllocAccountType<'b>> {
        self.inner.get_field(AllocAccountType::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_alloc_account_type<'b: 'a>(&mut self, alloc_account_type: AllocAccountType<'b>) {
        self.inner.to_mut().set_field(alloc_account_type);
    }
        
    pub fn avg_par_px<'b: 'a>(&'b self) -> Option<AvgParPx<'b>> {
        self.inner.get_field(AvgParPx::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_avg_par_px<'b: 'a>(&mut self, avg_par_px: AvgParPx<'b>) {
        self.inner.to_mut().set_field(avg_par_px);
    }
        
}


