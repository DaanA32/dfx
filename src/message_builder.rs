use crate::message_factory::MessageFactory;

pub(crate) struct MessageBuilder {
    pub(crate) msg: String,
    pub(crate) sender_default_appl_ver_id: String,
    pub(crate) validate_length_and_checksum: bool,
    pub(crate) session_data_dictionary: crate::data_dictionary::DataDictionary,
    pub(crate) application_data_dictionary: crate::data_dictionary::DataDictionary,
    pub(crate) msg_factory: Box<dyn MessageFactory>,
}
impl MessageBuilder {
    pub(crate) fn new(
        msg: String,
        sender_default_appl_ver_id: String,
        validate_length_and_checksum: bool,
        session_data_dictionary: crate::data_dictionary::DataDictionary,
        application_data_dictionary: crate::data_dictionary::DataDictionary,
        msg_factory: Box<dyn MessageFactory>,
    ) -> MessageBuilder {
        MessageBuilder {
            msg,
            sender_default_appl_ver_id,
            validate_length_and_checksum,
            session_data_dictionary,
            application_data_dictionary,
            msg_factory,
        }
    }
}
