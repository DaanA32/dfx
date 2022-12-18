use std::borrow::Cow;

use dfx_core::message::Message;

use super::super::fields::*;

/// AllocationInstructionAck
#[derive(Clone, Debug)]
pub struct AllocationInstructionAck<'a> {
    inner: Cow<'a, Message>
}

impl<'a> AllocationInstructionAck<'a> {
    //TODO implement
    
    pub fn no_allocs<'b: 'a>(&'b self) -> Option<NoAllocs<'b>> {
        self.inner.get_field(NoAllocs::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_no_allocs<'b: 'a>(&mut self, no_allocs: NoAllocs<'b>) {
        self.inner.to_mut().set_field(no_allocs);
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
