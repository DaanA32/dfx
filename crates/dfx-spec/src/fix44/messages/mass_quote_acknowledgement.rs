use std::borrow::Cow;

use dfx_core::message::Message;
use crate::fix44::fields::*;

/// MassQuoteAcknowledgement
#[derive(Clone, Debug)]
pub struct MassQuoteAcknowledgement<'a> {
    inner: Cow<'a, Message>
}

impl<'a> MassQuoteAcknowledgement<'a> {
    //TODO implement
    
    pub fn account<'b: 'a>(&'b self) -> Option<Account<'b>> {
        self.inner.get_field(Account::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_account<'b: 'a>(&mut self, account: Account<'b>) {
        self.inner.to_mut().set_field(account);
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

