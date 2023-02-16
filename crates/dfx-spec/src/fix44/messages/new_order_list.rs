use std::borrow::Cow;

use dfx_core::message::Message;
use crate::fix44::fields::*;

/// NewOrderList
#[derive(Clone, Debug)]
pub struct NewOrderList<'a> {
    inner: Cow<'a, Message>
}

impl<'a> NewOrderList<'a> {
    //TODO implement
    
    pub fn list_id<'b: 'a>(&'b self) -> Option<ListId<'b>> {
        self.inner.get_field(ListId::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_list_id<'b: 'a>(&mut self, list_id: ListId<'b>) {
        self.inner.to_mut().set_field(list_id);
    }
        
    pub fn list_exec_inst<'b: 'a>(&'b self) -> Option<ListExecInst<'b>> {
        self.inner.get_field(ListExecInst::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_list_exec_inst<'b: 'a>(&mut self, list_exec_inst: ListExecInst<'b>) {
        self.inner.to_mut().set_field(list_exec_inst);
    }
        
    pub fn no_orders<'b: 'a>(&'b self) -> Option<NoOrders<'b>> {
        self.inner.get_field(NoOrders::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_no_orders<'b: 'a>(&mut self, no_orders: NoOrders<'b>) {
        self.inner.to_mut().set_field(no_orders);
    }
        
    pub fn encoded_list_exec_inst_len<'b: 'a>(&'b self) -> Option<EncodedListExecInstLen<'b>> {
        self.inner.get_field(EncodedListExecInstLen::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_encoded_list_exec_inst_len<'b: 'a>(&mut self, encoded_list_exec_inst_len: EncodedListExecInstLen<'b>) {
        self.inner.to_mut().set_field(encoded_list_exec_inst_len);
    }
        
    pub fn encoded_list_exec_inst<'b: 'a>(&'b self) -> Option<EncodedListExecInst<'b>> {
        self.inner.get_field(EncodedListExecInst::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_encoded_list_exec_inst<'b: 'a>(&mut self, encoded_list_exec_inst: EncodedListExecInst<'b>) {
        self.inner.to_mut().set_field(encoded_list_exec_inst);
    }
        
    pub fn bid_id<'b: 'a>(&'b self) -> Option<BidId<'b>> {
        self.inner.get_field(BidId::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_bid_id<'b: 'a>(&mut self, bid_id: BidId<'b>) {
        self.inner.to_mut().set_field(bid_id);
    }
        
    pub fn client_bid_id<'b: 'a>(&'b self) -> Option<ClientBidId<'b>> {
        self.inner.get_field(ClientBidId::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_client_bid_id<'b: 'a>(&mut self, client_bid_id: ClientBidId<'b>) {
        self.inner.to_mut().set_field(client_bid_id);
    }
        
    pub fn bid_type<'b: 'a>(&'b self) -> Option<BidType<'b>> {
        self.inner.get_field(BidType::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_bid_type<'b: 'a>(&mut self, bid_type: BidType<'b>) {
        self.inner.to_mut().set_field(bid_type);
    }
        
    pub fn list_exec_inst_type<'b: 'a>(&'b self) -> Option<ListExecInstType<'b>> {
        self.inner.get_field(ListExecInstType::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_list_exec_inst_type<'b: 'a>(&mut self, list_exec_inst_type: ListExecInstType<'b>) {
        self.inner.to_mut().set_field(list_exec_inst_type);
    }
        
    pub fn cancellation_rights<'b: 'a>(&'b self) -> Option<CancellationRights<'b>> {
        self.inner.get_field(CancellationRights::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_cancellation_rights<'b: 'a>(&mut self, cancellation_rights: CancellationRights<'b>) {
        self.inner.to_mut().set_field(cancellation_rights);
    }
        
    pub fn allowable_one_sidedness_pct<'b: 'a>(&'b self) -> Option<AllowableOneSidednessPct<'b>> {
        self.inner.get_field(AllowableOneSidednessPct::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_allowable_one_sidedness_pct<'b: 'a>(&mut self, allowable_one_sidedness_pct: AllowableOneSidednessPct<'b>) {
        self.inner.to_mut().set_field(allowable_one_sidedness_pct);
    }
        
    pub fn allowable_one_sidedness_value<'b: 'a>(&'b self) -> Option<AllowableOneSidednessValue<'b>> {
        self.inner.get_field(AllowableOneSidednessValue::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_allowable_one_sidedness_value<'b: 'a>(&mut self, allowable_one_sidedness_value: AllowableOneSidednessValue<'b>) {
        self.inner.to_mut().set_field(allowable_one_sidedness_value);
    }
        
    pub fn allowable_one_sidedness_curr<'b: 'a>(&'b self) -> Option<AllowableOneSidednessCurr<'b>> {
        self.inner.get_field(AllowableOneSidednessCurr::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_allowable_one_sidedness_curr<'b: 'a>(&mut self, allowable_one_sidedness_curr: AllowableOneSidednessCurr<'b>) {
        self.inner.to_mut().set_field(allowable_one_sidedness_curr);
    }
        
    pub fn last_fragment<'b: 'a>(&'b self) -> Option<LastFragment<'b>> {
        self.inner.get_field(LastFragment::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_last_fragment<'b: 'a>(&mut self, last_fragment: LastFragment<'b>) {
        self.inner.to_mut().set_field(last_fragment);
    }
        
    pub fn no_orders_group(&self) -> Option<NoOrdersGroup> {
        todo!()
    }
    pub fn set_no_orders_group(&mut self, _no_orders_group: NoOrdersGroup) {
        todo!()
    }
        
}


pub struct NoOrdersGroup {

}

