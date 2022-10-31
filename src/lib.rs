pub mod fields;
pub mod tags;
pub mod data_dictionary;
pub mod data_dictionary_provider;
pub mod field_map;
pub mod message;
pub mod message_factory;
pub mod message_store;
pub mod message_store_factory;
pub mod log_factory;
pub mod session;
pub mod log;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
