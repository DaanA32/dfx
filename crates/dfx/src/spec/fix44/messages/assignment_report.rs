use std::borrow::Cow;

use dfx_core::message::Message;

use super::super::fields::*;

/// AssignmentReport
#[derive(Clone, Debug)]
pub struct AssignmentReport<'a> {
    inner: Cow<'a, Message>
}

impl<'a> AssignmentReport<'a> {
    //TODO implement
    
    pub fn account<'b: 'a>(&'b self) -> Option<Account<'b>> {
        self.inner.get_field(Account::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_account<'b: 'a>(&mut self, account: Account<'b>) {
        self.inner.to_mut().set_field(account);
    }
        
    pub fn currency<'b: 'a>(&'b self) -> Option<Currency<'b>> {
        self.inner.get_field(Currency::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_currency<'b: 'a>(&mut self, currency: Currency<'b>) {
        self.inner.to_mut().set_field(currency);
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
        
    pub fn expire_date<'b: 'a>(&'b self) -> Option<ExpireDate<'b>> {
        self.inner.get_field(ExpireDate::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_expire_date<'b: 'a>(&mut self, expire_date: ExpireDate<'b>) {
        self.inner.to_mut().set_field(expire_date);
    }
        
    pub fn no_legs<'b: 'a>(&'b self) -> Option<NoLegs<'b>> {
        self.inner.get_field(NoLegs::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_no_legs<'b: 'a>(&mut self, no_legs: NoLegs<'b>) {
        self.inner.to_mut().set_field(no_legs);
    }
        
    pub fn account_type<'b: 'a>(&'b self) -> Option<AccountType<'b>> {
        self.inner.get_field(AccountType::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_account_type<'b: 'a>(&mut self, account_type: AccountType<'b>) {
        self.inner.to_mut().set_field(account_type);
    }
        
    pub fn clearing_business_date<'b: 'a>(&'b self) -> Option<ClearingBusinessDate<'b>> {
        self.inner.get_field(ClearingBusinessDate::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_clearing_business_date<'b: 'a>(&mut self, clearing_business_date: ClearingBusinessDate<'b>) {
        self.inner.to_mut().set_field(clearing_business_date);
    }
        
    pub fn assignment_method<'b: 'a>(&'b self) -> Option<AssignmentMethod<'b>> {
        self.inner.get_field(AssignmentMethod::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_assignment_method<'b: 'a>(&mut self, assignment_method: AssignmentMethod<'b>) {
        self.inner.to_mut().set_field(assignment_method);
    }
        
    pub fn assignment_unit<'b: 'a>(&'b self) -> Option<AssignmentUnit<'b>> {
        self.inner.get_field(AssignmentUnit::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_assignment_unit<'b: 'a>(&mut self, assignment_unit: AssignmentUnit<'b>) {
        self.inner.to_mut().set_field(assignment_unit);
    }
        
    pub fn exercise_method<'b: 'a>(&'b self) -> Option<ExerciseMethod<'b>> {
        self.inner.get_field(ExerciseMethod::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_exercise_method<'b: 'a>(&mut self, exercise_method: ExerciseMethod<'b>) {
        self.inner.to_mut().set_field(exercise_method);
    }
        
    pub fn asgn_rpt_id<'b: 'a>(&'b self) -> Option<AsgnRptId<'b>> {
        self.inner.get_field(AsgnRptId::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_asgn_rpt_id<'b: 'a>(&mut self, asgn_rpt_id: AsgnRptId<'b>) {
        self.inner.to_mut().set_field(asgn_rpt_id);
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

