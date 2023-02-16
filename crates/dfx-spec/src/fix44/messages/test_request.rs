use std::borrow::Cow;

use dfx_core::message::Message;
use crate::fix44::fields::*;

/// TestRequest
#[derive(Clone, Debug)]
pub struct TestRequest<'a> {
    inner: Cow<'a, Message>
}

impl<'a> TestRequest<'a> {
    //TODO implement
    
    pub fn test_req_id<'b: 'a>(&'b self) -> Option<TestReqId<'b>> {
        self.inner.get_field(TestReqId::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_test_req_id<'b: 'a>(&mut self, test_req_id: TestReqId<'b>) {
        self.inner.to_mut().set_field(test_req_id);
    }
        
}


