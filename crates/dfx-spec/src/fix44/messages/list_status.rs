use std::borrow::Cow;

use dfx_core::message::Message;
use crate::fix44::fields::*;

/// ListStatus
#[derive(Clone, Debug)]
pub struct ListStatus<'a> {
    inner: Cow<'a, Message>
}

impl<'a> ListStatus<'a> {
    //TODO implement
    
    pub fn list_id<'b: 'a>(&'b self) -> Option<ListId<'b>> {
        self.inner.get_field(ListId::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_list_id<'b: 'a>(&mut self, list_id: ListId<'b>) {
        self.inner.to_mut().set_field(list_id);
    }
        
    pub fn no_orders<'b: 'a>(&'b self) -> Option<NoOrders<'b>> {
        self.inner.get_field(NoOrders::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_no_orders<'b: 'a>(&mut self, no_orders: NoOrders<'b>) {
        self.inner.to_mut().set_field(no_orders);
    }
        
    pub fn no_rpts<'b: 'a>(&'b self) -> Option<NoRpts<'b>> {
        self.inner.get_field(NoRpts::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_no_rpts<'b: 'a>(&mut self, no_rpts: NoRpts<'b>) {
        self.inner.to_mut().set_field(no_rpts);
    }
        
    pub fn list_status_type<'b: 'a>(&'b self) -> Option<ListStatusType<'b>> {
        self.inner.get_field(ListStatusType::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_list_status_type<'b: 'a>(&mut self, list_status_type: ListStatusType<'b>) {
        self.inner.to_mut().set_field(list_status_type);
    }
        
    pub fn list_order_status<'b: 'a>(&'b self) -> Option<ListOrderStatus<'b>> {
        self.inner.get_field(ListOrderStatus::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_list_order_status<'b: 'a>(&mut self, list_order_status: ListOrderStatus<'b>) {
        self.inner.to_mut().set_field(list_order_status);
    }
        
    pub fn list_status_text<'b: 'a>(&'b self) -> Option<ListStatusText<'b>> {
        self.inner.get_field(ListStatusText::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_list_status_text<'b: 'a>(&mut self, list_status_text: ListStatusText<'b>) {
        self.inner.to_mut().set_field(list_status_text);
    }
        
    pub fn encoded_list_status_text_len<'b: 'a>(&'b self) -> Option<EncodedListStatusTextLen<'b>> {
        self.inner.get_field(EncodedListStatusTextLen::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_encoded_list_status_text_len<'b: 'a>(&mut self, encoded_list_status_text_len: EncodedListStatusTextLen<'b>) {
        self.inner.to_mut().set_field(encoded_list_status_text_len);
    }
        
    pub fn encoded_list_status_text<'b: 'a>(&'b self) -> Option<EncodedListStatusText<'b>> {
        self.inner.get_field(EncodedListStatusText::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_encoded_list_status_text<'b: 'a>(&mut self, encoded_list_status_text: EncodedListStatusText<'b>) {
        self.inner.to_mut().set_field(encoded_list_status_text);
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

