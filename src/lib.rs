pub mod fields;
pub mod tags;
pub mod data_dictionary;
pub mod field_map;
pub mod message;
pub mod message_factory;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
