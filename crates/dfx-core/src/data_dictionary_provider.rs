use std::collections::BTreeMap;

use crate::data_dictionary::DataDictionary;

pub trait DataDictionaryProvider: Send {
    fn get_session_data_dictionary(&self, begin_string: &str) -> &DataDictionary;
    fn get_application_data_dictionary(&self, appl_ver_id: &str) -> &DataDictionary;
    fn add_session_data_dictionary(&mut self, begin_string: &str, dictionary: DataDictionary);
    fn add_application_data_dictionary(&mut self, appl_ver_id: &str, dictionary: DataDictionary);
}

#[derive(Clone, Debug, Default)]
pub struct DefaultDataDictionaryProvider {
    default: DataDictionary,
    transport: BTreeMap<String, DataDictionary>,
    app: BTreeMap<String, DataDictionary>,
}
impl DefaultDataDictionaryProvider {
    pub fn new() -> Self {
        DefaultDataDictionaryProvider::default()
    }
    pub fn boxed() -> Box<dyn DataDictionaryProvider> {
        Box::new(DefaultDataDictionaryProvider::default())
    }
}

impl DataDictionaryProvider for DefaultDataDictionaryProvider {
    //TODO should this not be session id instead of begin string?
    fn get_session_data_dictionary(&self, begin_string: &str) -> &DataDictionary {
        //TODO
        // DataDictionary.DataDictionary dd;
        // if (!transportDataDictionaries_.TryGetValue(beginString, out dd))
        //     return emptyDataDictionary_;
        // return dd;
        self.transport.get(begin_string).unwrap_or_else(|| &self.default)
    }
    fn get_application_data_dictionary(&self, appl_ver_id: &str) -> &DataDictionary {
        //TODO
        // DataDictionary.DataDictionary dd;
        // if (!applicationDataDictionaries_.TryGetValue(applVerID, out dd))
        //     return emptyDataDictionary_;
        // return dd;
        self.app.get(appl_ver_id).unwrap_or_else(|| &self.default)
    }

    fn add_session_data_dictionary(&mut self, begin_string: &str, dictionary: DataDictionary) {
        self.transport.insert(begin_string.into(), dictionary);
    }

    fn add_application_data_dictionary(&mut self, appl_ver_id: &str, dictionary: DataDictionary) {
        self.app.insert(appl_ver_id.into(), dictionary);
    }
}
