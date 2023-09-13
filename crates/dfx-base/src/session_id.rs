use std::fmt::Display;

//TODO partial match on sender + target comp?
#[derive(Debug, Clone, Default, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct SessionId {
    pub(crate) id: String,
    pub(crate) begin_string: String,
    pub(crate) sender_comp_id: String,
    pub(crate) sender_sub_id: String,
    pub(crate) sender_location_id: String,
    pub(crate) target_comp_id: String,
    pub(crate) target_sub_id: String,
    pub(crate) target_location_id: String,
    // pub(crate) session_qualifier: String,
    pub(crate) is_fixt: bool,
}

impl SessionId {
    pub fn new<A, B, C, D, E, F, G>(
        begin_string: A,
        sender_comp_id: B,
        sender_sub_id: C,
        sender_location_id: D,
        target_comp_id: E,
        target_sub_id: F,
        target_location_id: G,
        // session_qualifier: String,
    ) -> Self
    where
        A: Into<String>,
        B: Into<String>,
        C: Into<String>,
        D: Into<String>,
        E: Into<String>,
        F: Into<String>,
        G: Into<String>,
    {
        let begin_string = begin_string.into();
        let sender_comp_id = sender_comp_id.into();
        let sender_sub_id = sender_sub_id.into();
        let sender_location_id = sender_location_id.into();
        let target_comp_id = target_comp_id.into();
        let target_sub_id = target_sub_id.into();
        let target_location_id = target_location_id.into();
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

    pub fn is_empty(&self) -> bool {
        self.sender_comp_id.len() + self.target_comp_id.len() == 0
    }

    pub fn id(&self) -> &str {
        self.id.as_ref()
    }

    pub fn begin_string(&self) -> &str {
        self.begin_string.as_ref()
    }

    pub fn sender_comp_id(&self) -> &str {
        self.sender_comp_id.as_ref()
    }

    pub fn sender_sub_id(&self) -> &str {
        self.sender_sub_id.as_ref()
    }

    pub fn sender_location_id(&self) -> &str {
        self.sender_location_id.as_ref()
    }

    pub fn target_comp_id(&self) -> &str {
        self.target_comp_id.as_ref()
    }

    pub fn target_sub_id(&self) -> &str {
        self.target_sub_id.as_ref()
    }

    pub fn target_location_id(&self) -> &str {
        self.target_location_id.as_ref()
    }

    pub fn is_fixt(&self) -> bool {
        self.is_fixt
    }


    pub fn prefix(&self) -> String {
        format!("{}-{}{}{}{}{}-{}{}{}{}{}",
            self.begin_string(),
            self.sender_comp_id(),
            if self.sender_sub_id().is_empty() { "" } else { "-" },
            self.sender_sub_id(),
            if self.sender_location_id().is_empty() { "" } else { "-" },
            self.sender_location_id(),
            self.target_comp_id(),
            if self.target_sub_id().is_empty() { "" } else { "-" },
            self.target_sub_id(),
            if self.target_location_id().is_empty() { "" } else { "-" },
            self.target_location_id(),
        )
    }
}

impl Display for SessionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.id))
    }
}
