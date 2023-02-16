use std::borrow::Cow;

use dfx_core::message::Message;
use crate::fix44::fields::*;

/// NetworkCounterpartySystemStatusResponse
#[derive(Clone, Debug)]
pub struct NetworkCounterpartySystemStatusResponse<'a> {
    inner: Cow<'a, Message>
}

impl<'a> NetworkCounterpartySystemStatusResponse<'a> {
    //TODO implement
    
    pub fn no_comp_i_ds<'b: 'a>(&'b self) -> Option<NoCompIDs<'b>> {
        self.inner.get_field(NoCompIDs::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_no_comp_i_ds<'b: 'a>(&mut self, no_comp_i_ds: NoCompIDs<'b>) {
        self.inner.to_mut().set_field(no_comp_i_ds);
    }
        
    pub fn no_comp_i_ds_group(&self) -> Option<NoCompIDsGroup> {
        todo!()
    }
    pub fn set_no_comp_i_ds_group(&mut self, _no_comp_i_ds_group: NoCompIDsGroup) {
        todo!()
    }
        
}


pub struct NoCompIDsGroup {

}

