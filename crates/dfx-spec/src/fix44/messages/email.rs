use std::borrow::Cow;

use dfx_core::message::Message;
use crate::fix44::fields::*;

/// Email
#[derive(Clone, Debug)]
pub struct Email<'a> {
    inner: Cow<'a, Message>
}

impl<'a> Email<'a> {
    //TODO implement
    
    pub fn cl_ord_id<'b: 'a>(&'b self) -> Option<ClOrdId<'b>> {
        self.inner.get_field(ClOrdId::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_cl_ord_id<'b: 'a>(&mut self, cl_ord_id: ClOrdId<'b>) {
        self.inner.to_mut().set_field(cl_ord_id);
    }
        
    pub fn email_type<'b: 'a>(&'b self) -> Option<EmailType<'b>> {
        self.inner.get_field(EmailType::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_email_type<'b: 'a>(&mut self, email_type: EmailType<'b>) {
        self.inner.to_mut().set_field(email_type);
    }
        
    pub fn no_related_sym<'b: 'a>(&'b self) -> Option<NoRelatedSym<'b>> {
        self.inner.get_field(NoRelatedSym::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_no_related_sym<'b: 'a>(&mut self, no_related_sym: NoRelatedSym<'b>) {
        self.inner.to_mut().set_field(no_related_sym);
    }
        
    pub fn email_thread_id<'b: 'a>(&'b self) -> Option<EmailThreadId<'b>> {
        self.inner.get_field(EmailThreadId::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_email_thread_id<'b: 'a>(&mut self, email_thread_id: EmailThreadId<'b>) {
        self.inner.to_mut().set_field(email_thread_id);
    }
        
    pub fn encoded_subject_len<'b: 'a>(&'b self) -> Option<EncodedSubjectLen<'b>> {
        self.inner.get_field(EncodedSubjectLen::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_encoded_subject_len<'b: 'a>(&mut self, encoded_subject_len: EncodedSubjectLen<'b>) {
        self.inner.to_mut().set_field(encoded_subject_len);
    }
        
    pub fn encoded_subject<'b: 'a>(&'b self) -> Option<EncodedSubject<'b>> {
        self.inner.get_field(EncodedSubject::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_encoded_subject<'b: 'a>(&mut self, encoded_subject: EncodedSubject<'b>) {
        self.inner.to_mut().set_field(encoded_subject);
    }
        
    pub fn no_related_sym_group(&self) -> Option<NoRelatedSymGroup> {
        todo!()
    }
    pub fn set_no_related_sym_group(&mut self, _no_related_sym_group: NoRelatedSymGroup) {
        todo!()
    }
        
}


pub struct NoRelatedSymGroup {

}

