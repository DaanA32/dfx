use std::borrow::Cow;

use dfx_core::message::Message;

use super::super::fields::*;

/// Heartbeat
#[derive(Clone, Debug)]
pub struct Heartbeat<'a> {
    inner: Cow<'a, Message>
}

impl<'a> Heartbeat<'a> {
    //TODO implement
    
    pub fn test_req_id<'b: 'a>(&'b self) -> Option<TestReqId<'b>> {
        self.inner.get_field(TestReqId::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_test_req_id<'b: 'a>(&mut self, test_req_id: TestReqId<'b>) {
        self.inner.to_mut().set_field(test_req_id);
    }
        
}


