use std::borrow::Cow;

use dfx_core::message::Message;


/// XmLnonFix
#[derive(Clone, Debug)]
pub struct XmLnonFix<'a> {
    inner: Cow<'a, Message>
}

impl<'a> XmLnonFix<'a> {
    //TODO implement
    
pub fn value(&self) -> &dfx_core::field_map::FieldMap {
    &self.inner
}

}


