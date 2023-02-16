use std::borrow::Cow;

use dfx_core::message::Message;
use crate::fix44::fields::*;

/// BidResponse
#[derive(Clone, Debug)]
pub struct BidResponse<'a> {
    inner: Cow<'a, Message>
}

impl<'a> BidResponse<'a> {
    //TODO implement
    
    pub fn no_bid_components<'b: 'a>(&'b self) -> Option<NoBidComponents<'b>> {
        self.inner.get_field(NoBidComponents::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_no_bid_components<'b: 'a>(&mut self, no_bid_components: NoBidComponents<'b>) {
        self.inner.to_mut().set_field(no_bid_components);
    }
        
    pub fn no_bid_components_group(&self) -> Option<NoBidComponentsGroup> {
        todo!()
    }
    pub fn set_no_bid_components_group(&mut self, _no_bid_components_group: NoBidComponentsGroup) {
        todo!()
    }
        
}


pub struct NoBidComponentsGroup {

}

