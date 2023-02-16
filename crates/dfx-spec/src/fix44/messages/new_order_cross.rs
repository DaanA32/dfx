use std::borrow::Cow;

use dfx_core::message::Message;
use crate::fix44::fields::*;

/// NewOrderCross
#[derive(Clone, Debug)]
pub struct NewOrderCross<'a> {
    inner: Cow<'a, Message>
}

impl<'a> NewOrderCross<'a> {
    //TODO implement
    
    pub fn currency<'b: 'a>(&'b self) -> Option<Currency<'b>> {
        self.inner.get_field(Currency::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_currency<'b: 'a>(&mut self, currency: Currency<'b>) {
        self.inner.to_mut().set_field(currency);
    }
        
    pub fn compliance_id<'b: 'a>(&'b self) -> Option<ComplianceId<'b>> {
        self.inner.get_field(ComplianceId::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_compliance_id<'b: 'a>(&mut self, compliance_id: ComplianceId<'b>) {
        self.inner.to_mut().set_field(compliance_id);
    }
        
    pub fn discretion_inst<'b: 'a>(&'b self) -> Option<DiscretionInst<'b>> {
        self.inner.get_field(DiscretionInst::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_discretion_inst<'b: 'a>(&mut self, discretion_inst: DiscretionInst<'b>) {
        self.inner.to_mut().set_field(discretion_inst);
    }
        
    pub fn discretion_offset_value<'b: 'a>(&'b self) -> Option<DiscretionOffsetValue<'b>> {
        self.inner.get_field(DiscretionOffsetValue::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_discretion_offset_value<'b: 'a>(&mut self, discretion_offset_value: DiscretionOffsetValue<'b>) {
        self.inner.to_mut().set_field(discretion_offset_value);
    }
        
    pub fn cancellation_rights<'b: 'a>(&'b self) -> Option<CancellationRights<'b>> {
        self.inner.get_field(CancellationRights::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_cancellation_rights<'b: 'a>(&mut self, cancellation_rights: CancellationRights<'b>) {
        self.inner.to_mut().set_field(cancellation_rights);
    }
        
    pub fn designation<'b: 'a>(&'b self) -> Option<Designation<'b>> {
        self.inner.get_field(Designation::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_designation<'b: 'a>(&mut self, designation: Designation<'b>) {
        self.inner.to_mut().set_field(designation);
    }
        
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
        
    pub fn discretion_move_type<'b: 'a>(&'b self) -> Option<DiscretionMoveType<'b>> {
        self.inner.get_field(DiscretionMoveType::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_discretion_move_type<'b: 'a>(&mut self, discretion_move_type: DiscretionMoveType<'b>) {
        self.inner.to_mut().set_field(discretion_move_type);
    }
        
    pub fn discretion_offset_type<'b: 'a>(&'b self) -> Option<DiscretionOffsetType<'b>> {
        self.inner.get_field(DiscretionOffsetType::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_discretion_offset_type<'b: 'a>(&mut self, discretion_offset_type: DiscretionOffsetType<'b>) {
        self.inner.to_mut().set_field(discretion_offset_type);
    }
        
    pub fn discretion_limit_type<'b: 'a>(&'b self) -> Option<DiscretionLimitType<'b>> {
        self.inner.get_field(DiscretionLimitType::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_discretion_limit_type<'b: 'a>(&mut self, discretion_limit_type: DiscretionLimitType<'b>) {
        self.inner.to_mut().set_field(discretion_limit_type);
    }
        
    pub fn discretion_round_direction<'b: 'a>(&'b self) -> Option<DiscretionRoundDirection<'b>> {
        self.inner.get_field(DiscretionRoundDirection::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_discretion_round_direction<'b: 'a>(&mut self, discretion_round_direction: DiscretionRoundDirection<'b>) {
        self.inner.to_mut().set_field(discretion_round_direction);
    }
        
    pub fn discretion_scope<'b: 'a>(&'b self) -> Option<DiscretionScope<'b>> {
        self.inner.get_field(DiscretionScope::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_discretion_scope<'b: 'a>(&mut self, discretion_scope: DiscretionScope<'b>) {
        self.inner.to_mut().set_field(discretion_scope);
    }
        
}


