pub mod fix44;

#[cfg(test)]
mod tests {

    use dfx_core::{field_map::FieldMap, fields::ConversionError};
    use dfx_core::fields::converters::TryFrom;
    use super::fix44::fields::{Account, Text};


    #[test]
    fn test_account() {
        let account = Account::new("Test");
        let account_clone = account.clone();
        let mut field_map = FieldMap::default();
        field_map.set_field(account);
        let field = field_map.get_field(Account::tag());
        let new_account: Account = TryFrom::try_from(field).unwrap();
        assert_eq!(account_clone, new_account)
    }

    #[test]
    fn test_account_neq_text() {
        let account = Account::new("Test");
        let mut field_map = FieldMap::default();
        field_map.set_field(account);
        let field = field_map.get_field(Account::tag());
        let new_account: Result<Text, ConversionError> = TryFrom::try_from(field);
        assert!(new_account.is_err())
    }
}
