use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct SessionId {
    pub id: String,
    pub begin_string: String,
    pub sender_comp_id: String,
    pub sender_sub_id: String,
    pub sender_location_id: String,
    pub target_comp_id: String,
    pub target_sub_id: String,
    pub target_location_id: String,
    // pub session_qualifier: String,
    pub is_fixt: bool,
}
impl SessionId {
    pub fn new(
        begin_string: String,
        sender_comp_id: String,
        sender_sub_id: String,
        sender_location_id: String,
        target_comp_id: String,
        target_sub_id: String,
        target_location_id: String,
        // session_qualifier: String,
    ) -> Self {
        let id = format!(
            "{}:{}{}{}->{}{}{}",
            begin_string,
            sender_comp_id,
            sender_sub_id,
            sender_location_id,
            target_comp_id,
            target_sub_id,
            target_location_id
        );
        let is_fixt = begin_string.starts_with("FIXT");
        SessionId {
            id,
            begin_string,
            sender_comp_id,
            sender_sub_id,
            sender_location_id,
            target_comp_id,
            target_sub_id,
            target_location_id,
            // session_qualifier,
            is_fixt,
        }
    }
}

impl Display for SessionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.id))
    }
}
