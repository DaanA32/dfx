use std::borrow::Cow;

use dfx_core::message::Message;

use super::super::fields::*;

/// CollateralInquiry
#[derive(Clone, Debug)]
pub struct CollateralInquiry<'a> {
    inner: Cow<'a, Message>
}

impl<'a> CollateralInquiry<'a> {
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
        
    pub fn accrued_interest_amt<'b: 'a>(&'b self) -> Option<AccruedInterestAmt<'b>> {
        self.inner.get_field(AccruedInterestAmt::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_accrued_interest_amt<'b: 'a>(&mut self, accrued_interest_amt: AccruedInterestAmt<'b>) {
        self.inner.to_mut().set_field(accrued_interest_amt);
    }
        
    pub fn account_type<'b: 'a>(&'b self) -> Option<AccountType<'b>> {
        self.inner.get_field(AccountType::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_account_type<'b: 'a>(&mut self, account_type: AccountType<'b>) {
        self.inner.to_mut().set_field(account_type);
    }
        
    pub fn clearing_business_date<'b: 'a>(&'b self) -> Option<ClearingBusinessDate<'b>> {
        self.inner.get_field(ClearingBusinessDate::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_clearing_business_date<'b: 'a>(&mut self, clearing_business_date: ClearingBusinessDate<'b>) {
        self.inner.to_mut().set_field(clearing_business_date);
    }
        
    pub fn cash_outstanding<'b: 'a>(&'b self) -> Option<CashOutstanding<'b>> {
        self.inner.get_field(CashOutstanding::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_cash_outstanding<'b: 'a>(&mut self, cash_outstanding: CashOutstanding<'b>) {
        self.inner.to_mut().set_field(cash_outstanding);
    }
        
    pub fn no_coll_inquiry_qualifier<'b: 'a>(&'b self) -> Option<NoCollInquiryQualifier<'b>> {
        self.inner.get_field(NoCollInquiryQualifier::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_no_coll_inquiry_qualifier<'b: 'a>(&mut self, no_coll_inquiry_qualifier: NoCollInquiryQualifier<'b>) {
        self.inner.to_mut().set_field(no_coll_inquiry_qualifier);
    }
        
    pub fn no_coll_inquiry_qualifier_group(&self) -> Option<NoCollInquiryQualifierGroup> {
        todo!()
    }
    pub fn set_no_coll_inquiry_qualifier_group(&mut self, _no_coll_inquiry_qualifier_group: NoCollInquiryQualifierGroup) {
        todo!()
    }
        
}


pub struct NoCollInquiryQualifierGroup {

}
