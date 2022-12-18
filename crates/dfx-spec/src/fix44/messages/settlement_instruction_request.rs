use std::borrow::Cow;

use dfx_core::message::Message;

use super::super::fields::*;

/// SettlementInstructionRequest
#[derive(Clone, Debug)]
pub struct SettlementInstructionRequest<'a> {
    inner: Cow<'a, Message>
}

impl<'a> SettlementInstructionRequest<'a> {
    //TODO implement
    
    pub fn alloc_account<'b: 'a>(&'b self) -> Option<AllocAccount<'b>> {
        self.inner.get_field(AllocAccount::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_alloc_account<'b: 'a>(&mut self, alloc_account: AllocAccount<'b>) {
        self.inner.to_mut().set_field(alloc_account);
    }
        
    pub fn expire_time<'b: 'a>(&'b self) -> Option<ExpireTime<'b>> {
        self.inner.get_field(ExpireTime::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_expire_time<'b: 'a>(&mut self, expire_time: ExpireTime<'b>) {
        self.inner.to_mut().set_field(expire_time);
    }
        
    pub fn effective_time<'b: 'a>(&'b self) -> Option<EffectiveTime<'b>> {
        self.inner.get_field(EffectiveTime::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_effective_time<'b: 'a>(&mut self, effective_time: EffectiveTime<'b>) {
        self.inner.to_mut().set_field(effective_time);
    }
        
    pub fn no_party_i_ds<'b: 'a>(&'b self) -> Option<NoPartyIDs<'b>> {
        self.inner.get_field(NoPartyIDs::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_no_party_i_ds<'b: 'a>(&mut self, no_party_i_ds: NoPartyIDs<'b>) {
        self.inner.to_mut().set_field(no_party_i_ds);
    }
        
    pub fn cfi_code<'b: 'a>(&'b self) -> Option<CfiCode<'b>> {
        self.inner.get_field(CfiCode::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_cfi_code<'b: 'a>(&mut self, cfi_code: CfiCode<'b>) {
        self.inner.to_mut().set_field(cfi_code);
    }
        
    pub fn alloc_acct_id_source<'b: 'a>(&'b self) -> Option<AllocAcctIdSource<'b>> {
        self.inner.get_field(AllocAcctIdSource::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_alloc_acct_id_source<'b: 'a>(&mut self, alloc_acct_id_source: AllocAcctIdSource<'b>) {
        self.inner.to_mut().set_field(alloc_acct_id_source);
    }
        
    pub fn last_update_time<'b: 'a>(&'b self) -> Option<LastUpdateTime<'b>> {
        self.inner.get_field(LastUpdateTime::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_last_update_time<'b: 'a>(&mut self, last_update_time: LastUpdateTime<'b>) {
        self.inner.to_mut().set_field(last_update_time);
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

