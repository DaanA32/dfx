use std::borrow::Cow;

use dfx_core::message::Message;
use crate::fix44::fields::*;

/// MassQuote
#[derive(Clone, Debug)]
pub struct MassQuote<'a> {
    inner: Cow<'a, Message>
}

impl<'a> MassQuote<'a> {
    //TODO implement
    
    pub fn account<'b: 'a>(&'b self) -> Option<Account<'b>> {
        self.inner.get_field(Account::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_account<'b: 'a>(&mut self, account: Account<'b>) {
        self.inner.to_mut().set_field(account);
    }
        
    pub fn def_bid_size<'b: 'a>(&'b self) -> Option<DefBidSize<'b>> {
        self.inner.get_field(DefBidSize::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_def_bid_size<'b: 'a>(&mut self, def_bid_size: DefBidSize<'b>) {
        self.inner.to_mut().set_field(def_bid_size);
    }
        
    pub fn def_offer_size<'b: 'a>(&'b self) -> Option<DefOfferSize<'b>> {
        self.inner.get_field(DefOfferSize::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_def_offer_size<'b: 'a>(&mut self, def_offer_size: DefOfferSize<'b>) {
        self.inner.to_mut().set_field(def_offer_size);
    }
        
    pub fn no_party_i_ds<'b: 'a>(&'b self) -> Option<NoPartyIDs<'b>> {
        self.inner.get_field(NoPartyIDs::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_no_party_i_ds<'b: 'a>(&mut self, no_party_i_ds: NoPartyIDs<'b>) {
        self.inner.to_mut().set_field(no_party_i_ds);
    }
        
    pub fn account_type<'b: 'a>(&'b self) -> Option<AccountType<'b>> {
        self.inner.get_field(AccountType::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_account_type<'b: 'a>(&mut self, account_type: AccountType<'b>) {
        self.inner.to_mut().set_field(account_type);
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

