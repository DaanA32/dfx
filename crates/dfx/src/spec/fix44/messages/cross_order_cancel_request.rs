use std::borrow::Cow;

use dfx_core::message::Message;

use super::super::fields::*;

/// CrossOrderCancelRequest
#[derive(Clone, Debug)]
pub struct CrossOrderCancelRequest<'a> {
    inner: Cow<'a, Message>
}

impl<'a> CrossOrderCancelRequest<'a> {
    //TODO implement
    
    pub fn cross_id<'b: 'a>(&'b self) -> Option<CrossId<'b>> {
        self.inner.get_field(CrossId::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_cross_id<'b: 'a>(&mut self, cross_id: CrossId<'b>) {
        self.inner.to_mut().set_field(cross_id);
    }
        
    pub fn cross_type<'b: 'a>(&'b self) -> Option<CrossType<'b>> {
        self.inner.get_field(CrossType::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_cross_type<'b: 'a>(&mut self, cross_type: CrossType<'b>) {
        self.inner.to_mut().set_field(cross_type);
    }
        
    pub fn cross_prioritization<'b: 'a>(&'b self) -> Option<CrossPrioritization<'b>> {
        self.inner.get_field(CrossPrioritization::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_cross_prioritization<'b: 'a>(&mut self, cross_prioritization: CrossPrioritization<'b>) {
        self.inner.to_mut().set_field(cross_prioritization);
    }
        
    pub fn no_legs<'b: 'a>(&'b self) -> Option<NoLegs<'b>> {
        self.inner.get_field(NoLegs::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_no_legs<'b: 'a>(&mut self, no_legs: NoLegs<'b>) {
        self.inner.to_mut().set_field(no_legs);
    }
        
    pub fn no_legs_group(&self) -> Option<NoLegsGroup> {
        todo!()
    }
    pub fn set_no_legs_group(&mut self, _no_legs_group: NoLegsGroup) {
        todo!()
    }
        
}


pub struct NoLegsGroup {

}

