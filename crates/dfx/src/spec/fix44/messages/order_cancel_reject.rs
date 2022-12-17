use std::borrow::Cow;

use dfx_core::message::Message;

use super::super::fields::*;

/// OrderCancelReject
#[derive(Clone, Debug)]
pub struct OrderCancelReject<'a> {
    inner: Cow<'a, Message>
}

impl<'a> OrderCancelReject<'a> {
    //TODO implement
    
    pub fn account<'b: 'a>(&'b self) -> Option<Account<'b>> {
        self.inner.get_field(Account::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_account<'b: 'a>(&mut self, account: Account<'b>) {
        self.inner.to_mut().set_field(account);
    }
        
    pub fn cl_ord_id<'b: 'a>(&'b self) -> Option<ClOrdId<'b>> {
        self.inner.get_field(ClOrdId::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_cl_ord_id<'b: 'a>(&mut self, cl_ord_id: ClOrdId<'b>) {
        self.inner.to_mut().set_field(cl_ord_id);
    }
        
    pub fn order_id<'b: 'a>(&'b self) -> Option<OrderId<'b>> {
        self.inner.get_field(OrderId::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_order_id<'b: 'a>(&mut self, order_id: OrderId<'b>) {
        self.inner.to_mut().set_field(order_id);
    }
        
    pub fn ord_status<'b: 'a>(&'b self) -> Option<OrdStatus<'b>> {
        self.inner.get_field(OrdStatus::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_ord_status<'b: 'a>(&mut self, ord_status: OrdStatus<'b>) {
        self.inner.to_mut().set_field(ord_status);
    }
        
    pub fn orig_cl_ord_id<'b: 'a>(&'b self) -> Option<OrigClOrdId<'b>> {
        self.inner.get_field(OrigClOrdId::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_orig_cl_ord_id<'b: 'a>(&mut self, orig_cl_ord_id: OrigClOrdId<'b>) {
        self.inner.to_mut().set_field(orig_cl_ord_id);
    }
        
    pub fn text<'b: 'a>(&'b self) -> Option<Text<'b>> {
        self.inner.get_field(Text::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_text<'b: 'a>(&mut self, text: Text<'b>) {
        self.inner.to_mut().set_field(text);
    }
        
    pub fn transact_time<'b: 'a>(&'b self) -> Option<TransactTime<'b>> {
        self.inner.get_field(TransactTime::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_transact_time<'b: 'a>(&mut self, transact_time: TransactTime<'b>) {
        self.inner.to_mut().set_field(transact_time);
    }
        
    pub fn list_id<'b: 'a>(&'b self) -> Option<ListId<'b>> {
        self.inner.get_field(ListId::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_list_id<'b: 'a>(&mut self, list_id: ListId<'b>) {
        self.inner.to_mut().set_field(list_id);
    }
        
    pub fn trade_date<'b: 'a>(&'b self) -> Option<TradeDate<'b>> {
        self.inner.get_field(TradeDate::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_trade_date<'b: 'a>(&mut self, trade_date: TradeDate<'b>) {
        self.inner.to_mut().set_field(trade_date);
    }
        
    pub fn cxl_rej_reason<'b: 'a>(&'b self) -> Option<CxlRejReason<'b>> {
        self.inner.get_field(CxlRejReason::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_cxl_rej_reason<'b: 'a>(&mut self, cxl_rej_reason: CxlRejReason<'b>) {
        self.inner.to_mut().set_field(cxl_rej_reason);
    }
        
    pub fn secondary_order_id<'b: 'a>(&'b self) -> Option<SecondaryOrderId<'b>> {
        self.inner.get_field(SecondaryOrderId::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_secondary_order_id<'b: 'a>(&mut self, secondary_order_id: SecondaryOrderId<'b>) {
        self.inner.to_mut().set_field(secondary_order_id);
    }
        
    pub fn trade_origination_date<'b: 'a>(&'b self) -> Option<TradeOriginationDate<'b>> {
        self.inner.get_field(TradeOriginationDate::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_trade_origination_date<'b: 'a>(&mut self, trade_origination_date: TradeOriginationDate<'b>) {
        self.inner.to_mut().set_field(trade_origination_date);
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
        
    pub fn cxl_rej_response_to<'b: 'a>(&'b self) -> Option<CxlRejResponseTo<'b>> {
        self.inner.get_field(CxlRejResponseTo::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_cxl_rej_response_to<'b: 'a>(&mut self, cxl_rej_response_to: CxlRejResponseTo<'b>) {
        self.inner.to_mut().set_field(cxl_rej_response_to);
    }
        
    pub fn secondary_cl_ord_id<'b: 'a>(&'b self) -> Option<SecondaryClOrdId<'b>> {
        self.inner.get_field(SecondaryClOrdId::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_secondary_cl_ord_id<'b: 'a>(&mut self, secondary_cl_ord_id: SecondaryClOrdId<'b>) {
        self.inner.to_mut().set_field(secondary_cl_ord_id);
    }
        
    pub fn account_type<'b: 'a>(&'b self) -> Option<AccountType<'b>> {
        self.inner.get_field(AccountType::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_account_type<'b: 'a>(&mut self, account_type: AccountType<'b>) {
        self.inner.to_mut().set_field(account_type);
    }
        
    pub fn cl_ord_link_id<'b: 'a>(&'b self) -> Option<ClOrdLinkId<'b>> {
        self.inner.get_field(ClOrdLinkId::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_cl_ord_link_id<'b: 'a>(&mut self, cl_ord_link_id: ClOrdLinkId<'b>) {
        self.inner.to_mut().set_field(cl_ord_link_id);
    }
        
    pub fn orig_ord_mod_time<'b: 'a>(&'b self) -> Option<OrigOrdModTime<'b>> {
        self.inner.get_field(OrigOrdModTime::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_orig_ord_mod_time<'b: 'a>(&mut self, orig_ord_mod_time: OrigOrdModTime<'b>) {
        self.inner.to_mut().set_field(orig_ord_mod_time);
    }
        
    pub fn working_indicator<'b: 'a>(&'b self) -> Option<WorkingIndicator<'b>> {
        self.inner.get_field(WorkingIndicator::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_working_indicator<'b: 'a>(&mut self, working_indicator: WorkingIndicator<'b>) {
        self.inner.to_mut().set_field(working_indicator);
    }
        
    pub fn acct_id_source<'b: 'a>(&'b self) -> Option<AcctIdSource<'b>> {
        self.inner.get_field(AcctIdSource::tag()).map(|v| v.try_into().ok()).flatten()
    }
    pub fn set_acct_id_source<'b: 'a>(&mut self, acct_id_source: AcctIdSource<'b>) {
        self.inner.to_mut().set_field(acct_id_source);
    }
        
}


