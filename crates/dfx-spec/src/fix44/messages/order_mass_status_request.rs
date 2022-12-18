use std::borrow::Cow;

use dfx_core::message::Message;

use super::super::fields::*;

/// OrderMassStatusRequest
#[derive(Clone, Debug)]
pub struct OrderMassStatusRequest<'a> {
    inner: Cow<'a, Message>
}

impl<'a> OrderMassStatusRequest<'a> {
    //TODO implement
    
    pub fn account<'b: 'a>(&'b self) -> Option<Account<'b>> {
        self.inner.get_field(Account::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_account<'b: 'a>(&mut self, account: Account<'b>) {
        self.inner.to_mut().set_field(account);
    }
        
    pub fn coupon_rate<'b: 'a>(&'b self) -> Option<CouponRate<'b>> {
        self.inner.get_field(CouponRate::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_coupon_rate<'b: 'a>(&mut self, coupon_rate: CouponRate<'b>) {
        self.inner.to_mut().set_field(coupon_rate);
    }
        
    pub fn coupon_payment_date<'b: 'a>(&'b self) -> Option<CouponPaymentDate<'b>> {
        self.inner.get_field(CouponPaymentDate::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_coupon_payment_date<'b: 'a>(&mut self, coupon_payment_date: CouponPaymentDate<'b>) {
        self.inner.to_mut().set_field(coupon_payment_date);
    }
        
    pub fn contract_multiplier<'b: 'a>(&'b self) -> Option<ContractMultiplier<'b>> {
        self.inner.get_field(ContractMultiplier::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_contract_multiplier<'b: 'a>(&mut self, contract_multiplier: ContractMultiplier<'b>) {
        self.inner.to_mut().set_field(contract_multiplier);
    }
        
    pub fn credit_rating<'b: 'a>(&'b self) -> Option<CreditRating<'b>> {
        self.inner.get_field(CreditRating::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_credit_rating<'b: 'a>(&mut self, credit_rating: CreditRating<'b>) {
        self.inner.to_mut().set_field(credit_rating);
    }
        
    pub fn encoded_issuer_len<'b: 'a>(&'b self) -> Option<EncodedIssuerLen<'b>> {
        self.inner.get_field(EncodedIssuerLen::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_encoded_issuer_len<'b: 'a>(&mut self, encoded_issuer_len: EncodedIssuerLen<'b>) {
        self.inner.to_mut().set_field(encoded_issuer_len);
    }
        
    pub fn encoded_issuer<'b: 'a>(&'b self) -> Option<EncodedIssuer<'b>> {
        self.inner.get_field(EncodedIssuer::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_encoded_issuer<'b: 'a>(&mut self, encoded_issuer: EncodedIssuer<'b>) {
        self.inner.to_mut().set_field(encoded_issuer);
    }
        
    pub fn encoded_security_desc_len<'b: 'a>(&'b self) -> Option<EncodedSecurityDescLen<'b>> {
        self.inner.get_field(EncodedSecurityDescLen::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_encoded_security_desc_len<'b: 'a>(&mut self, encoded_security_desc_len: EncodedSecurityDescLen<'b>) {
        self.inner.to_mut().set_field(encoded_security_desc_len);
    }
        
    pub fn encoded_security_desc<'b: 'a>(&'b self) -> Option<EncodedSecurityDesc<'b>> {
        self.inner.get_field(EncodedSecurityDesc::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_encoded_security_desc<'b: 'a>(&mut self, encoded_security_desc: EncodedSecurityDesc<'b>) {
        self.inner.to_mut().set_field(encoded_security_desc);
    }
        
    pub fn cfi_code<'b: 'a>(&'b self) -> Option<CfiCode<'b>> {
        self.inner.get_field(CfiCode::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_cfi_code<'b: 'a>(&mut self, cfi_code: CfiCode<'b>) {
        self.inner.to_mut().set_field(cfi_code);
    }
        
    pub fn country_of_issue<'b: 'a>(&'b self) -> Option<CountryOfIssue<'b>> {
        self.inner.get_field(CountryOfIssue::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_country_of_issue<'b: 'a>(&mut self, country_of_issue: CountryOfIssue<'b>) {
        self.inner.to_mut().set_field(country_of_issue);
    }
        
    pub fn acct_id_source<'b: 'a>(&'b self) -> Option<AcctIdSource<'b>> {
        self.inner.get_field(AcctIdSource::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_acct_id_source<'b: 'a>(&mut self, acct_id_source: AcctIdSource<'b>) {
        self.inner.to_mut().set_field(acct_id_source);
    }
        
    pub fn contract_settl_month<'b: 'a>(&'b self) -> Option<ContractSettlMonth<'b>> {
        self.inner.get_field(ContractSettlMonth::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_contract_settl_month<'b: 'a>(&mut self, contract_settl_month: ContractSettlMonth<'b>) {
        self.inner.to_mut().set_field(contract_settl_month);
    }
        
    pub fn no_events<'b: 'a>(&'b self) -> Option<NoEvents<'b>> {
        self.inner.get_field(NoEvents::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_no_events<'b: 'a>(&mut self, no_events: NoEvents<'b>) {
        self.inner.to_mut().set_field(no_events);
    }
        
    pub fn dated_date<'b: 'a>(&'b self) -> Option<DatedDate<'b>> {
        self.inner.get_field(DatedDate::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_dated_date<'b: 'a>(&mut self, dated_date: DatedDate<'b>) {
        self.inner.to_mut().set_field(dated_date);
    }
        
    pub fn cp_program<'b: 'a>(&'b self) -> Option<CpProgram<'b>> {
        self.inner.get_field(CpProgram::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_cp_program<'b: 'a>(&mut self, cp_program: CpProgram<'b>) {
        self.inner.to_mut().set_field(cp_program);
    }
        
    pub fn cp_reg_type<'b: 'a>(&'b self) -> Option<CpRegType<'b>> {
        self.inner.get_field(CpRegType::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_cp_reg_type<'b: 'a>(&mut self, cp_reg_type: CpRegType<'b>) {
        self.inner.to_mut().set_field(cp_reg_type);
    }
        
    pub fn no_events_group(&self) -> Option<NoEventsGroup> {
        todo!()
    }
    pub fn set_no_events_group(&mut self, _no_events_group: NoEventsGroup) {
        todo!()
    }
        
}


pub struct NoEventsGroup {

}

