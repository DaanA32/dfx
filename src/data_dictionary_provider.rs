use crate::data_dictionary::DataDictionary;

pub trait DataDictionaryProvider {
    fn create(&self) -> DataDictionary;
}
