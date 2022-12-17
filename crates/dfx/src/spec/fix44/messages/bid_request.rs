use std::borrow::Cow;

use dfx_core::message::Message;

use super::super::fields::*;

/// BidRequest
#[derive(Clone, Debug)]
pub struct BidRequest<'a> {
    inner: Cow<'a, Message>
}

impl<'a> BidRequest<'a> {
    //TODO implement
    
    pub fn basis_px_type<'b: 'a>(&'b self) -> Option<BasisPxType<'b>> {
        self.inner.get_field(BasisPxType::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_basis_px_type<'b: 'a>(&mut self, basis_px_type: BasisPxType<'b>) {
        self.inner.to_mut().set_field(basis_px_type);
    }
        
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

