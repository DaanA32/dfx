use std::borrow::Cow;

use dfx_core::message::Message;

use super::super::fields::*;

/// ConfirmationRequest
#[derive(Clone, Debug)]
pub struct ConfirmationRequest<'a> {
    inner: Cow<'a, Message>
}

impl<'a> ConfirmationRequest<'a> {
    //TODO implement
    
    pub fn alloc_id<'b: 'a>(&'b self) -> Option<AllocId<'b>> {
        self.inner.get_field(AllocId::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_alloc_id<'b: 'a>(&mut self, alloc_id: AllocId<'b>) {
        self.inner.to_mut().set_field(alloc_id);
    }
        
    pub fn no_orders<'b: 'a>(&'b self) -> Option<NoOrders<'b>> {
        self.inner.get_field(NoOrders::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_no_orders<'b: 'a>(&mut self, no_orders: NoOrders<'b>) {
        self.inner.to_mut().set_field(no_orders);
    }
        
    pub fn alloc_account<'b: 'a>(&'b self) -> Option<AllocAccount<'b>> {
        self.inner.get_field(AllocAccount::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_alloc_account<'b: 'a>(&mut self, alloc_account: AllocAccount<'b>) {
        self.inner.to_mut().set_field(alloc_account);
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
        
    pub fn individual_alloc_id<'b: 'a>(&'b self) -> Option<IndividualAllocId<'b>> {
        self.inner.get_field(IndividualAllocId::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_individual_alloc_id<'b: 'a>(&mut self, individual_alloc_id: IndividualAllocId<'b>) {
        self.inner.to_mut().set_field(individual_alloc_id);
    }
        
    pub fn alloc_acct_id_source<'b: 'a>(&'b self) -> Option<AllocAcctIdSource<'b>> {
        self.inner.get_field(AllocAcctIdSource::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_alloc_acct_id_source<'b: 'a>(&mut self, alloc_acct_id_source: AllocAcctIdSource<'b>) {
        self.inner.to_mut().set_field(alloc_acct_id_source);
    }
        
    pub fn confirm_type<'b: 'a>(&'b self) -> Option<ConfirmType<'b>> {
        self.inner.get_field(ConfirmType::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_confirm_type<'b: 'a>(&mut self, confirm_type: ConfirmType<'b>) {
        self.inner.to_mut().set_field(confirm_type);
    }
        
    pub fn alloc_account_type<'b: 'a>(&'b self) -> Option<AllocAccountType<'b>> {
        self.inner.get_field(AllocAccountType::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_alloc_account_type<'b: 'a>(&mut self, alloc_account_type: AllocAccountType<'b>) {
        self.inner.to_mut().set_field(alloc_account_type);
    }
        
    pub fn confirm_req_id<'b: 'a>(&'b self) -> Option<ConfirmReqId<'b>> {
        self.inner.get_field(ConfirmReqId::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_confirm_req_id<'b: 'a>(&mut self, confirm_req_id: ConfirmReqId<'b>) {
        self.inner.to_mut().set_field(confirm_req_id);
    }
        
    pub fn no_orders_group(&self) -> Option<NoOrdersGroup> {
        todo!()
    }
    pub fn set_no_orders_group(&mut self, _no_orders_group: NoOrdersGroup) {
        todo!()
    }
        
}


pub struct NoOrdersGroup {

}

