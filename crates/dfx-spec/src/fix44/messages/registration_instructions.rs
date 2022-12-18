use std::borrow::Cow;

use dfx_core::message::Message;

use super::super::fields::*;

/// RegistrationInstructions
#[derive(Clone, Debug)]
pub struct RegistrationInstructions<'a> {
    inner: Cow<'a, Message>
}

impl<'a> RegistrationInstructions<'a> {
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
        
    pub fn no_party_i_ds<'b: 'a>(&'b self) -> Option<NoPartyIDs<'b>> {
        self.inner.get_field(NoPartyIDs::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_no_party_i_ds<'b: 'a>(&mut self, no_party_i_ds: NoPartyIDs<'b>) {
        self.inner.to_mut().set_field(no_party_i_ds);
    }
        
    pub fn ownership_type<'b: 'a>(&'b self) -> Option<OwnershipType<'b>> {
        self.inner.get_field(OwnershipType::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_ownership_type<'b: 'a>(&mut self, ownership_type: OwnershipType<'b>) {
        self.inner.to_mut().set_field(ownership_type);
    }
        
    pub fn acct_id_source<'b: 'a>(&'b self) -> Option<AcctIdSource<'b>> {
        self.inner.get_field(AcctIdSource::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_acct_id_source<'b: 'a>(&mut self, acct_id_source: AcctIdSource<'b>) {
        self.inner.to_mut().set_field(acct_id_source);
    }
        
    pub fn no_party_i_ds_group(&self) -> Option<NoPartyIDsGroup> {
        todo!()
    }
    pub fn set_no_party_i_ds_group(&mut self, _no_party_i_ds_group: NoPartyIDsGroup) {
        todo!()
    }
        
}


pub struct NoPartyIDsGroup {

}

