use std::borrow::Cow;

use dfx_core::message::Message;

use super::super::fields::*;

/// CollateralInquiryAck
#[derive(Clone, Debug)]
pub struct CollateralInquiryAck<'a> {
    inner: Cow<'a, Message>
}

impl<'a> CollateralInquiryAck<'a> {
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

