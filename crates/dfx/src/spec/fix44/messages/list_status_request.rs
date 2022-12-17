use std::borrow::Cow;

use dfx_core::message::Message;

use super::super::fields::*;

/// ListStatusRequest
#[derive(Clone, Debug)]
pub struct ListStatusRequest<'a> {
    inner: Cow<'a, Message>
}

impl<'a> ListStatusRequest<'a> {
    //TODO implement
    
    pub fn text<'b: 'a>(&'b self) -> Option<Text<'b>> {
        self.inner.get_field(Text::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_text<'b: 'a>(&mut self, text: Text<'b>) {
        self.inner.to_mut().set_field(text);
    }
        
    pub fn list_id<'b: 'a>(&'b self) -> Option<ListId<'b>> {
        self.inner.get_field(ListId::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_list_id<'b: 'a>(&mut self, list_id: ListId<'b>) {
        self.inner.to_mut().set_field(list_id);
    }
        
    pub fn encoded_text_len<'b: 'a>(&'b self) -> Option<EncodedTextLen<'b>> {
        self.inner.get_field(EncodedTextLen::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_encoded_text_len<'b: 'a>(&mut self, encoded_text_len: EncodedTextLen<'b>) {
        self.inner.to_mut().set_field(encoded_text_len);
    }
        
    pub fn encoded_text<'b: 'a>(&'b self) -> Option<EncodedText<'b>> {
        self.inner.get_field(EncodedText::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_encoded_text<'b: 'a>(&mut self, encoded_text: EncodedText<'b>) {
        self.inner.to_mut().set_field(encoded_text);
    }
        
}


