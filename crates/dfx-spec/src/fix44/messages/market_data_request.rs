use std::borrow::Cow;

use dfx_core::message::Message;
use crate::fix44::fields::*;

/// MarketDataRequest
#[derive(Clone, Debug)]
pub struct MarketDataRequest<'a> {
    inner: Cow<'a, Message>
}

impl<'a> MarketDataRequest<'a> {
    //TODO implement
    
    pub fn no_related_sym<'b: 'a>(&'b self) -> Option<NoRelatedSym<'b>> {
        self.inner.get_field(NoRelatedSym::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_no_related_sym<'b: 'a>(&mut self, no_related_sym: NoRelatedSym<'b>) {
        self.inner.to_mut().set_field(no_related_sym);
    }
        
    pub fn aggregated_book<'b: 'a>(&'b self) -> Option<AggregatedBook<'b>> {
        self.inner.get_field(AggregatedBook::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_aggregated_book<'b: 'a>(&mut self, aggregated_book: AggregatedBook<'b>) {
        self.inner.to_mut().set_field(aggregated_book);
    }
        
    pub fn appl_queue_max<'b: 'a>(&'b self) -> Option<ApplQueueMax<'b>> {
        self.inner.get_field(ApplQueueMax::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_appl_queue_max<'b: 'a>(&mut self, appl_queue_max: ApplQueueMax<'b>) {
        self.inner.to_mut().set_field(appl_queue_max);
    }
        
    pub fn appl_queue_action<'b: 'a>(&'b self) -> Option<ApplQueueAction<'b>> {
        self.inner.get_field(ApplQueueAction::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_appl_queue_action<'b: 'a>(&mut self, appl_queue_action: ApplQueueAction<'b>) {
        self.inner.to_mut().set_field(appl_queue_action);
    }
        
    pub fn no_related_sym_group(&self) -> Option<NoRelatedSymGroup> {
        todo!()
    }
    pub fn set_no_related_sym_group(&mut self, _no_related_sym_group: NoRelatedSymGroup) {
        todo!()
    }
        
}


pub struct NoRelatedSymGroup {

}

