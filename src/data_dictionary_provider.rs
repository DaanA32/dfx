use crate::data_dictionary::DataDictionary;

pub(crate) trait DataDictionaryProvider: Send {
    fn get_session_data_dictionary(&self, begin_string: &str) -> DataDictionary;
    fn get_application_data_dictionary(&self, begin_string: &str) -> DataDictionary;
}

pub(crate) struct DefaultDataDictionaryProvider;
impl DefaultDataDictionaryProvider {
    pub fn new() -> Box<dyn DataDictionaryProvider> {
        Box::new(DefaultDataDictionaryProvider)
    }
}

impl DataDictionaryProvider for DefaultDataDictionaryProvider {
    fn get_session_data_dictionary(&self, begin_string: &str) -> DataDictionary {
        //TODO
        // DataDictionary.DataDictionary dd;
        // if (!transportDataDictionaries_.TryGetValue(beginString, out dd))
        //     return emptyDataDictionary_;
        // return dd;
        DataDictionary::default()
    }
    fn get_application_data_dictionary(&self, appl_ver_id: &str) -> DataDictionary {
        //TODO
        // DataDictionary.DataDictionary dd;
        // if (!applicationDataDictionaries_.TryGetValue(applVerID, out dd))
        //     return emptyDataDictionary_;
        // return dd;
        DataDictionary::default()
    }
}
