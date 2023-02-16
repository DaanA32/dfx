use std::borrow::Cow;

use dfx_core::message::Message;
use crate::fix44::fields::*;

/// ListStrikePrice
#[derive(Clone, Debug)]
pub struct ListStrikePrice<'a> {
    inner: Cow<'a, Message>
}

impl<'a> ListStrikePrice<'a> {
    //TODO implement
    
    pub fn no_strikes<'b: 'a>(&'b self) -> Option<NoStrikes<'b>> {
        self.inner.get_field(NoStrikes::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_no_strikes<'b: 'a>(&mut self, no_strikes: NoStrikes<'b>) {
        self.inner.to_mut().set_field(no_strikes);
    }
        
    pub fn no_strikes_group(&self) -> Option<NoStrikesGroup> {
        todo!()
    }
    pub fn set_no_strikes_group(&mut self, _no_strikes_group: NoStrikesGroup) {
        todo!()
    }
        
}


pub struct NoStrikesGroup {

}

