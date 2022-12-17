use std::borrow::Cow;

use dfx_core::message::Message;

use super::super::fields::*;

/// AllocationReport
#[derive(Clone, Debug)]
pub struct AllocationReport<'a> {
    inner: Cow<'a, Message>
}

impl<'a> AllocationReport<'a> {
    //TODO implement
    
    pub fn no_allocs<'b: 'a>(&'b self) -> Option<NoAllocs<'b>> {
        self.inner.get_field(NoAllocs::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_no_allocs<'b: 'a>(&mut self, no_allocs: NoAllocs<'b>) {
        self.inner.to_mut().set_field(no_allocs);
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
        
    pub fn alloc_canc_replace_reason<'b: 'a>(&'b self) -> Option<AllocCancReplaceReason<'b>> {
        self.inner.get_field(AllocCancReplaceReason::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_alloc_canc_replace_reason<'b: 'a>(&mut self, alloc_canc_replace_reason: AllocCancReplaceReason<'b>) {
        self.inner.to_mut().set_field(alloc_canc_replace_reason);
    }
        
    pub fn no_allocs_group(&self) -> Option<NoAllocsGroup> {
        todo!()
    }
    pub fn set_no_allocs_group(&mut self, _no_allocs_group: NoAllocsGroup) {
        todo!()
    }
        
}


pub struct NoAllocsGroup {

}

