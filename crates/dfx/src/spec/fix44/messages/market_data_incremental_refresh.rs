use std::borrow::Cow;

use dfx_core::message::Message;

use super::super::fields::*;

/// MarketDataIncrementalRefresh
#[derive(Clone, Debug)]
pub struct MarketDataIncrementalRefresh<'a> {
    inner: Cow<'a, Message>
}

impl<'a> MarketDataIncrementalRefresh<'a> {
    //TODO implement
    
    pub fn no_md_entries<'b: 'a>(&'b self) -> Option<NoMdEntries<'b>> {
        self.inner.get_field(NoMdEntries::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_no_md_entries<'b: 'a>(&mut self, no_md_entries: NoMdEntries<'b>) {
        self.inner.to_mut().set_field(no_md_entries);
    }
        
    pub fn appl_queue_depth<'b: 'a>(&'b self) -> Option<ApplQueueDepth<'b>> {
        self.inner.get_field(ApplQueueDepth::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_appl_queue_depth<'b: 'a>(&mut self, appl_queue_depth: ApplQueueDepth<'b>) {
        self.inner.to_mut().set_field(appl_queue_depth);
    }
        
    pub fn appl_queue_resolution<'b: 'a>(&'b self) -> Option<ApplQueueResolution<'b>> {
        self.inner.get_field(ApplQueueResolution::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_appl_queue_resolution<'b: 'a>(&mut self, appl_queue_resolution: ApplQueueResolution<'b>) {
        self.inner.to_mut().set_field(appl_queue_resolution);
    }
        
    pub fn no_md_entries_group(&self) -> Option<NoMdEntriesGroup> {
        todo!()
    }
    pub fn set_no_md_entries_group(&mut self, _no_md_entries_group: NoMdEntriesGroup) {
        todo!()
    }
        
}


pub struct NoMdEntriesGroup {

}

