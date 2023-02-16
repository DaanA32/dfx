use std::borrow::Cow;

use dfx_core::message::Message;
use crate::fix44::fields::*;

/// CollateralReport
#[derive(Clone, Debug)]
pub struct CollateralReport<'a> {
    inner: Cow<'a, Message>
}

impl<'a> CollateralReport<'a> {
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
        
    pub fn currency<'b: 'a>(&'b self) -> Option<Currency<'b>> {
        self.inner.get_field(Currency::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_currency<'b: 'a>(&mut self, currency: Currency<'b>) {
        self.inner.to_mut().set_field(currency);
    }
        
    pub fn no_execs<'b: 'a>(&'b self) -> Option<NoExecs<'b>> {
        self.inner.get_field(NoExecs::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_no_execs<'b: 'a>(&mut self, no_execs: NoExecs<'b>) {
        self.inner.to_mut().set_field(no_execs);
    }
        
    pub fn accrued_interest_amt<'b: 'a>(&'b self) -> Option<AccruedInterestAmt<'b>> {
        self.inner.get_field(AccruedInterestAmt::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_accrued_interest_amt<'b: 'a>(&mut self, accrued_interest_amt: AccruedInterestAmt<'b>) {
        self.inner.to_mut().set_field(accrued_interest_amt);
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
        
    pub fn coll_rpt_id<'b: 'a>(&'b self) -> Option<CollRptId<'b>> {
        self.inner.get_field(CollRptId::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_coll_rpt_id<'b: 'a>(&mut self, coll_rpt_id: CollRptId<'b>) {
        self.inner.to_mut().set_field(coll_rpt_id);
    }
        
    pub fn coll_inquiry_id<'b: 'a>(&'b self) -> Option<CollInquiryId<'b>> {
        self.inner.get_field(CollInquiryId::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_coll_inquiry_id<'b: 'a>(&mut self, coll_inquiry_id: CollInquiryId<'b>) {
        self.inner.to_mut().set_field(coll_inquiry_id);
    }
        
    pub fn coll_status<'b: 'a>(&'b self) -> Option<CollStatus<'b>> {
        self.inner.get_field(CollStatus::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_coll_status<'b: 'a>(&mut self, coll_status: CollStatus<'b>) {
        self.inner.to_mut().set_field(coll_status);
    }
        
    pub fn end_accrued_interest_amt<'b: 'a>(&'b self) -> Option<EndAccruedInterestAmt<'b>> {
        self.inner.get_field(EndAccruedInterestAmt::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_end_accrued_interest_amt<'b: 'a>(&mut self, end_accrued_interest_amt: EndAccruedInterestAmt<'b>) {
        self.inner.to_mut().set_field(end_accrued_interest_amt);
    }
        
    pub fn end_cash<'b: 'a>(&'b self) -> Option<EndCash<'b>> {
        self.inner.get_field(EndCash::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_end_cash<'b: 'a>(&mut self, end_cash: EndCash<'b>) {
        self.inner.to_mut().set_field(end_cash);
    }
        
    pub fn no_execs_group(&self) -> Option<NoExecsGroup> {
        todo!()
    }
    pub fn set_no_execs_group(&mut self, _no_execs_group: NoExecsGroup) {
        todo!()
    }
        
}


pub struct NoExecsGroup {

}

