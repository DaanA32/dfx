use dfx_core::message_factory::MessageFactory;

#[derive(Debug, Clone, Copy)]
pub struct Fix44MessageFactory;

impl MessageFactory for Fix44MessageFactory {
    fn get_supported_begin_strings(&self) -> Vec<String> {
        vec![String::from("FIX44")]
    }

    fn create(&self, begin_string: &str, msg_type: &str) -> Result<dfx_core::message::Message, dfx_core::message_factory::MessageFactoryError> {
        // check if begin string == FIX44
        todo!("{begin_string} {msg_type}")
    }

    fn create_group(&self, begin_string: &str, msg_type: &str, group_counter_tag: dfx_core::field_map::Tag) -> Option<dfx_core::field_map::Group> {
        // TODO function
        todo!("{begin_string} {msg_type} {group_counter_tag}")
    }
}

