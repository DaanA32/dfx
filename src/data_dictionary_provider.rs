use crate::data_dictionary::DataDictionary;

pub trait DataDictionaryProvider {
    fn get_session_data_dictionary(&self, begin_string: &str) -> DataDictionary;
    fn get_application_data_dictionary(&self, begin_string: &str) -> DataDictionary;
}

pub struct DefaultDataDictionaryProvider;

impl DataDictionaryProvider for DefaultDataDictionaryProvider {
    fn get_session_data_dictionary(&self, begin_string: &str) -> DataDictionary {
        todo!()
    }
    fn get_application_data_dictionary(&self, begin_string: &str) -> DataDictionary {
        todo!()
    }
}
