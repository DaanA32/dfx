use std::borrow::Cow;

use dfx_core::message::Message;

use super::super::fields::*;

/// MarketDataSnapshotFullRefresh
#[derive(Clone, Debug)]
pub struct MarketDataSnapshotFullRefresh<'a> {
    inner: Cow<'a, Message>
}

impl<'a> MarketDataSnapshotFullRefresh<'a> {
    //TODO implement
    
    pub fn financial_status<'b: 'a>(&'b self) -> Option<FinancialStatus<'b>> {
        self.inner.get_field(FinancialStatus::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_financial_status<'b: 'a>(&mut self, financial_status: FinancialStatus<'b>) {
        self.inner.to_mut().set_field(financial_status);
    }
        
    pub fn corporate_action<'b: 'a>(&'b self) -> Option<CorporateAction<'b>> {
        self.inner.get_field(CorporateAction::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_corporate_action<'b: 'a>(&mut self, corporate_action: CorporateAction<'b>) {
        self.inner.to_mut().set_field(corporate_action);
    }
        
    pub fn no_legs<'b: 'a>(&'b self) -> Option<NoLegs<'b>> {
        self.inner.get_field(NoLegs::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_no_legs<'b: 'a>(&mut self, no_legs: NoLegs<'b>) {
        self.inner.to_mut().set_field(no_legs);
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
        
    pub fn no_legs_group(&self) -> Option<NoLegsGroup> {
        todo!()
    }
    pub fn set_no_legs_group(&mut self, _no_legs_group: NoLegsGroup) {
        todo!()
    }
        
}


pub struct NoLegsGroup {

}

