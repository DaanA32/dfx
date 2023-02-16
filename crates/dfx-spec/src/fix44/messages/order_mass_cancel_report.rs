use std::borrow::Cow;

use dfx_core::message::Message;
use crate::fix44::fields::*;

/// OrderMassCancelReport
#[derive(Clone, Debug)]
pub struct OrderMassCancelReport<'a> {
    inner: Cow<'a, Message>
}

impl<'a> OrderMassCancelReport<'a> {
    //TODO implement
    
    pub fn no_affected_orders<'b: 'a>(&'b self) -> Option<NoAffectedOrders<'b>> {
        self.inner.get_field(NoAffectedOrders::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_no_affected_orders<'b: 'a>(&mut self, no_affected_orders: NoAffectedOrders<'b>) {
        self.inner.to_mut().set_field(no_affected_orders);
    }
        
    pub fn no_affected_orders_group(&self) -> Option<NoAffectedOrdersGroup> {
        todo!()
    }
    pub fn set_no_affected_orders_group(&mut self, _no_affected_orders_group: NoAffectedOrdersGroup) {
        todo!()
    }
        
}


pub struct NoAffectedOrdersGroup {

}

