


use crate::data_dictionary::DDMap;
use crate::data_dictionary::ArcGroup;
use crate::data_dictionary::DataDictionary;
use crate::data_dictionary::MessageValidationError;
use crate::data_dictionary::TagException;
use crate::field_map::FieldBase;
use crate::field_map::FieldMap;
use crate::field_map::FieldMapError;
use crate::field_map::Group;
use crate::field_map::Tag;
use crate::fix_values::SessionRejectReason;
use crate::fields::ApplVerID;
use crate::fields::ConversionError;
use crate::fix_values;
pub use crate::message_factory::*;
use crate::parser::read_msg_type;
use crate::session_id::SessionId;
use crate::tags;
use std::fmt::Display;
use std::ops::Deref;
use std::ops::DerefMut;

#[derive(Default, Clone, Debug)]
pub struct Header(FieldMap);

impl Header {
    pub fn calculate_string(&self) -> String {
        self.0.calculate_string(Some(HEADER_FIELD_ORDER.to_vec()))
    }
}

const HEADER_FIELD_ORDER: [Tag; 3] = [tags::BeginString, tags::BodyLength, tags::MsgType];
// const HEADER_FIELD_ORDER: Vec<Tag> = vec![ tags::BeginString, tags::BodyLength, tags::MsgType ];

impl Deref for Header {
    type Target = FieldMap;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Header {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Default, Clone, Debug)]
pub struct Trailer(FieldMap);

impl Trailer {
    pub fn calculate_string(&self) -> String {
        self.0.calculate_string(Some(TRAILER_FIELD_ORDER.to_vec()))
    }
}

const TRAILER_FIELD_ORDER: [Tag; 3] = [tags::SignatureLength, tags::Signature, tags::CheckSum];
// const TRAILER_FIELD_ORDER: Vec<Tag> = vec![ tags::SignatureLength, tags::Signature, tags::CheckSum ];

impl Deref for Trailer {
    type Target = FieldMap;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Trailer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone)]
pub struct Message {
    header: Header,
    body: FieldMap,
    trailer: Trailer,
    // application_data_dictionary: Option<DataDictionary>,
    field_: Tag,
    valid_structure_: bool,
}

impl Default for Message {
    fn default() -> Self {
        Message {
            header: Header::default(),
            body: FieldMap::default(),
            trailer: Trailer::default(),
            // application_data_dictionary: None,
            field_: 0,
            valid_structure_: true,
        }
    }
}

impl std::fmt::Debug for Message {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        fmt.write_str(
            format!(
                "Message (\n\tHeader {:?},\n\tBody: {:?},\n\ttrailer: {:?}\n)",
                self.header, self.body, self.trailer
            )
            .as_str(),
        )
    }
}

impl Message {
    pub const SOH: char = 1 as char;

    pub fn new(msgstr: &[u8]) -> Result<Self, MessageParseError> {
        let mut message = Message::default();
        message.from_string::<DefaultMessageFactory>(msgstr, true, None, None, None, false)?;
        Ok(message)
    }

    pub fn header(&self) -> &Header {
        &self.header
    }
    pub fn header_mut(&mut self) -> &mut Header {
        &mut self.header
    }
    pub fn trailer(&self) -> &Trailer {
        &self.trailer
    }
    pub fn trailer_mut(&mut self) -> &mut Trailer {
        &mut self.trailer
    }

    pub fn has_valid_structure(&self) -> Result<(), MessageValidationError> {
        if self.valid_structure_ {
            Ok(())
        } else {
            Err(MessageValidationError::TagException(TagException::tag_out_of_order(self.field_)))
        }
    }

    pub fn is_header_field(tag: Tag, data_dictionary: Option<&DataDictionary>) -> bool {
        match tag {
            tags::BeginString => true,
            tags::BodyLength => true,
            tags::MsgType => true,
            tags::SenderCompID => true,
            tags::TargetCompID => true,
            tags::OnBehalfOfCompID => true,
            tags::DeliverToCompID => true,
            tags::SecureDataLen => true,
            tags::MsgSeqNum => true,
            tags::SenderSubID => true,
            tags::SenderLocationID => true,
            tags::TargetSubID => true,
            tags::TargetLocationID => true,
            tags::OnBehalfOfSubID => true,
            tags::OnBehalfOfLocationID => true,
            tags::DeliverToSubID => true,
            tags::DeliverToLocationID => true,
            tags::PossDupFlag => true,
            tags::PossResend => true,
            tags::SendingTime => true,
            tags::OrigSendingTime => true,
            tags::XmlDataLen => true,
            tags::XmlData => true,
            tags::MessageEncoding => true,
            tags::LastMsgSeqNumProcessed => true,
            tags::OnBehalfOfSendingTime => true, //TODO
            _ => match data_dictionary {
                Some(dd) => dd.is_header_field(tag),
                None => false,
            },
        }
    }

    pub fn is_trailer_field(tag: Tag, data_dictionary: Option<&DataDictionary>) -> bool {
        match tag {
            tags::SignatureLength => true,
            tags::Signature => true,
            tags::CheckSum => true,
            _ => match data_dictionary {
                Some(dd) => dd.is_trailer_field(tag),
                None => false,
            },
        }
    }

    fn extract_field(
        msgstr: &[u8],
        pos: &mut usize,
        _session_dd: Option<&DataDictionary>,
        _app_dd: Option<&DataDictionary>,
        size_hint: Option<usize>,
    ) -> Result<FieldBase, MessageParseError> {
        let tagend = msgstr[*pos..].iter().position(|c| *c == b'=')
            .ok_or(MessageParseError::FailedToFindEqualsAt(*pos))?;

        let tagend = *pos + tagend;
        if *pos >= tagend {
            return Err(MessageParseError::PosGreaterThanLen(*pos, tagend));
        }

        let mut tag = 0;
        let mut neg = false;
        let mut start = true;
        for byte in &msgstr[*pos..tagend] {
            let byte = *byte;
            if byte == b'-' && start {
                neg = true;
                start = false;
            } else if byte.is_ascii_digit() {
                tag *= 10;
                tag += byte as Tag - b'0' as Tag;
                start = false;
            } else {
                return Err(MessageParseError::InvalidTagNumber(String::from_utf8_lossy(&msgstr[*pos..tagend]).to_string()));
            }
        }
        let tag = if neg { -tag } else { tag };

        *pos = tagend + 1;

        let fieldend = if let Some(value) = size_hint {
            Some(value)
        } else {
            msgstr[*pos..].iter().position(|c| *c == Message::SOH as u8)
        };
        let fieldend = fieldend.ok_or(MessageParseError::FailedToFindSohAt(*pos))?;
        let fieldend = *pos + fieldend;
        let value = &msgstr[*pos..fieldend];
        let field = FieldBase::from_bytes(tag, value.into());

        /*
         TODO data dict stuff
        if (((null != sessionDD) && sessionDD.IsDataField(field.Tag)) || ((null != appDD) && appDD.IsDataField(field.Tag)))
        {
            string fieldLength = "";
            // Assume length field is 1 less
            int lenField = field.Tag - 1;
            // Special case for Signature which violates above assumption
            if (Tags.Signature.Equals(field.Tag))
                lenField = Tags.SignatureLength;
            if ((null != group) && group.isSetField(lenField))
            {
                fieldLength = group.GetField(lenField);
                soh = equalSign + 1 + atol(fieldLength.c_str());
            }
            else if (isSetField(lenField))
            {
                fieldLength = getField(lenField);
                soh = equalSign + 1 + atol(fieldLength.c_str());
            }
        }
        */

        *pos = fieldend + 1;
        Ok(field)
    }

    /// Creates a Message from a FIX string.
    ///
    /// msg_factory
    /// > If [None], any groups will be constructed as generic Group objects
    ///
    /// ignoreBody
    /// > (default false) if true, ignores all non-header and non-trailer fields.
    /// >
    /// > Intended for callers that only need rejection-related information from the header.
    pub fn from_string<MsgFactory: MessageFactory>(
        &mut self,
        msgstr: &[u8],
        validate: bool,
        session_dd: Option<&DataDictionary>,
        app_dd: Option<&DataDictionary>,
        msg_factory: Option<&MsgFactory>,
        ignore_body: bool,
    ) -> Result<(), MessageParseError> {
        // self.application_data_dictionary = app_dd.cloned();
        self.clear();

        let mut msg_type;
        let mut expecting_header = true;
        let mut expecting_body = true;
        let mut count = 0;
        let mut pos = 0;
        let mut msg_map: Option<&DDMap> = None;
        let mut size_hint = None;

        while pos < msgstr.len() {
            let f = Message::extract_field(msgstr, &mut pos, session_dd, app_dd, size_hint)?;
            match (session_dd, app_dd) {
                (Some(session_dd), _) if session_dd.is_length_field(f.tag()) => {
                    size_hint = f.to_usize()
                }
                (_, Some(app_dd)) if app_dd.is_length_field(f.tag()) => size_hint = f.to_usize(),
                _ => size_hint = None,
            };

            if validate && count < 3 && HEADER_FIELD_ORDER[count] != f.tag() {
                return Err(MessageParseError::InvalidMessage(
                    "Header fields out of order".into(),
                ));
            }
            count += 1;

            if Message::is_header_field(f.tag(), session_dd) {
                if !expecting_header {
                    if 0 == self.field_ {
                        self.field_ = f.tag();
                    }
                    self.valid_structure_ = false;
                }

                if tags::MsgType == f.tag() {
                    msg_type = f.string_value();
                    if let Some(app_dd) = app_dd {
                        msg_map = app_dd.get_map_for_message(msg_type?.as_str());
                    }
                }

                if !self.header.set_field_base(f.clone(), Some(false)) {
                    self.header.repeated_tags_mut().push(f.clone());
                }

                match session_dd {
                    Some(dd) if dd.header().is_group(f.tag()) => {
                        pos = Message::set_group(
                            f.clone(),
                            msgstr,
                            pos,
                            &mut self.header,
                            dd.header().get_group(f.tag()),
                            session_dd,
                            app_dd,
                            msg_factory,
                        )?;
                    },
                    _ => {}
                }
            } else if Message::is_trailer_field(f.tag(), session_dd) {
                expecting_header = false;
                expecting_body = false;
                if !self.trailer.set_field_base(f.clone(), Some(false)) {
                    self.trailer.repeated_tags_mut().push(f.clone());
                }

                match session_dd {
                    Some(dd) if dd.header().is_group(f.tag()) => {
                        pos = Message::set_group(
                            f.clone(),
                            msgstr,
                            pos,
                            &mut self.trailer,
                            dd.trailer().get_group(f.tag()),
                            session_dd,
                            app_dd,
                            msg_factory,
                        )?;
                    },
                    _ => {}
                }
            } else if !ignore_body {
                if !expecting_body {
                    if self.field_ == 0 {
                        self.field_ = f.tag();
                    }
                    self.valid_structure_ = false;
                }

                expecting_header = false;
                if !self.set_field_base(f.clone(), Some(false)) {
                    self.repeated_tags_mut().push(f.clone());
                }

                match msg_map {
                    Some(map) if map.is_group(f.tag()) => {
                        pos = Message::set_group(
                            f.clone(),
                            msgstr,
                            pos,
                            self,
                            map.get_group(f.tag()),
                            session_dd,
                            app_dd,
                            msg_factory,
                        )?;
                    },
                    _ => {},
                }
            }
        }

        if validate {
            self.validate()?;
        }
        Ok(())
    }

    fn set_group<MsgFactory: MessageFactory>(
        grp_no_fld: FieldBase,
        msgstr: &[u8],
        pos: usize,
        map: &mut FieldMap,
        group_dd: Option<&ArcGroup>,
        session_dd: Option<&DataDictionary>,
        app_dd: Option<&DataDictionary>,
        msg_factory: Option<&MsgFactory>,
    ) -> Result<usize, MessageParseError> {
        match group_dd {
            Some(group_dd) => {
                let mut pos = pos;
                let grp_entry_delimiter_tag = group_dd.delim();
                let grp_pos = pos;
                let mut group: Option<Group> = None;
                let mut size_hint = None;

                while pos < msgstr.len() {
                    let grp_pos = pos;
                    let f = Message::extract_field(msgstr, &mut pos, session_dd, app_dd, size_hint)?;
                    match (session_dd, app_dd) {
                        (Some(session_dd), _) if session_dd.is_length_field(f.tag()) => {
                            size_hint = f.to_usize()
                        }
                        (_, Some(app_dd)) if app_dd.is_length_field(f.tag()) => size_hint = f.to_usize(),
                        _ => size_hint = None,
                    };
                    if f.tag() == grp_entry_delimiter_tag {

                        if let Some(ingroup) = group {
                            // We were already building an entry, so the delimiter means it's done.
                            map.add_group(f.tag(), &ingroup, Some(false));
                            group = None;
                        }

                        // Create a new group!
                        if let Some(factory) = msg_factory.as_ref() {
                            let begin_string = Message::extract_begin_string(msgstr)?;
                            let msg_type = Message::get_msg_type(msgstr)?;
                            group = factory.create_group(
                                begin_string.as_str(),
                                msg_type,
                                grp_no_fld.tag(),
                            );
                        }

                        //If above failed (shouldn't ever happen), just use a generic Group.
                        if group.is_none() {
                            group = Some(Group::new(grp_no_fld.tag(), grp_entry_delimiter_tag));
                        }

                    } else if !group_dd.is_field(f.tag()) {
                        // This field is not in the group, thus the repeating group is done.

                        if let Some(group) = group {
                            map.add_group(f.tag(), &group, Some(false));
                        }
                        return Ok(grp_pos);
                    } else if group_dd.is_field(f.tag())
                        && matches!(&group, Some(group) if group.is_field_set(f.tag()))
                    {
                        // Tag is appearing for the second time within a group element.
                        // Presumably the sender didn't set the delimiter (or their DD has a different delimiter).

                        return Err(
                            MessageParseError::RepeatedTagWithoutGroupDelimiterTagException(
                                grp_no_fld.tag(),
                                f.tag(),
                            ),
                        );
                    }

                    match group.as_mut() {
                        Some(group) => {
                            // f is just a field in our group entry.  Add it and iterate again.
                            group.set_field_base(f.clone(), None);

                            if group_dd.is_group(f.tag()) {
                                // f is a counter for a nested group.  Recurse!

                                pos = Message::set_group(
                                    f.clone(),
                                    msgstr,
                                    pos,
                                    group,
                                    group_dd.get_group(f.tag()),
                                    session_dd,
                                    app_dd,
                                    msg_factory,
                                )?;
                            }
                        },
                        None => {
                            // This means we got into the group's fields without finding a delimiter tag.
                            let _b = &msgstr[pos..];
                            return Err(MessageParseError::GroupDelimiterTagException(
                                grp_no_fld.tag(),
                                grp_entry_delimiter_tag,
                            ));
                        }
                    }
                }

                Ok(grp_pos)
            },
            None => Ok(pos),
        }
    }

    fn validate(&self) -> Result<(), MessageParseError> {
        let received_body_length = self.header.get_int(tags::BodyLength)?;
        if self.body_length() != received_body_length {
            return Err(MessageParseError::InvalidMessage(format!(
                "Expected BodyLength={}, Received BodyLength={}, Message.SeqNum={}",
                self.body_length(),
                received_body_length,
                self.header.get_int(tags::MsgSeqNum)?
            )));
        }
        let received_checksum = self.trailer.get_int(tags::CheckSum)?;
        if self.checksum() != received_checksum {
            return Err(MessageParseError::InvalidMessage(format!(
                "Expected CheckSum={}, Received CheckSum={}, Message.SeqNum={}",
                self.checksum(),
                received_checksum,
                self.header.get_int(tags::MsgSeqNum)?
            )));
        }
        Ok(())
    }

    fn body_length(&self) -> u32 {
        self.header.len() + self.len() + self.trailer.len()
    }

    fn checksum(&self) -> u32 {
        (self.header.calculate_total() + self.calculate_total() + self.trailer.calculate_total())
            % 256
    }

    fn clear(&mut self) {
        self.field_ = 0;
        self.header.clear();
        self.body.clear();
        self.trailer.clear();
    }

    pub fn to_string_mut(&mut self) -> String {
        let len = self.body_length().to_string();
        self.header
            .set_field_base(FieldBase::new(tags::BodyLength, len), Some(true));
        let checksum = format!("{:03}", self.checksum());
        self.trailer
            .set_field_base(FieldBase::new(tags::CheckSum, checksum), Some(true));
        format!(
            "{}{}{}",
            self.header.calculate_string(),
            self.calculate_string(None),
            self.trailer.calculate_string()
        )
    }

    pub fn is_admin(&self) -> bool {
        matches!(self.header.get_field(tags::MsgType), Some(field) if Message::is_admin_msg_type(field.value()))
    }

    pub fn is_admin_msg_type(msg_type: &[u8]) -> bool {
        msg_type.len() == 1 && "0A12345n".contains(msg_type[0] as char)
    }

    pub fn extract_begin_string(msgstr: &[u8]) -> Result<String, MessageParseError> {
        let mut pos = 0;
        let f = Message::extract_field(msgstr, &mut pos, None, None, None)?;
        Ok(f.string_value().clone()?)
    }

    pub(crate) fn get_msg_type(bytes: &[u8]) -> Result<&str, MessageParseError> {
        match read_msg_type(bytes) {
            Some(s) => Ok(s),
            None => Err(MessageParseError::Malformed { tag: 35, message: format!(
                    "missing or malformed tag 35 in msg: {:?}",
                    String::from_utf8_lossy(bytes)
            )})
        }
    }

    pub fn identify_type(msg_str: &[u8]) -> Result<&str, MessageParseError> {
        //TODO wrap in MsgType field?
        Message::get_msg_type(msg_str)
    }

    pub fn get_appl_ver_id(begin_string: &str) -> Result<u32, String> {
        match begin_string {
            fix_values::BeginString::FIX40 => Ok(ApplVerID::FIX40),
            fix_values::BeginString::FIX41 => Ok(ApplVerID::FIX41),
            fix_values::BeginString::FIX42 => Ok(ApplVerID::FIX42),
            fix_values::BeginString::FIX43 => Ok(ApplVerID::FIX43),
            fix_values::BeginString::FIX44 => Ok(ApplVerID::FIX44),
            fix_values::BeginString::FIX50 => Ok(ApplVerID::FIX50),
            fix_values::BeginString::FIX50SP1 => Ok(ApplVerID::FIX50SP1),
            fix_values::BeginString::FIX50SP2 => Ok(ApplVerID::FIX50SP2),
            _ => Err(format!("ApplVerID for {} not supported", begin_string)),
        }
    }

    pub fn reverse_route(&mut self, header: &Header) {
        // required routing tags
        self.header.remove_field(tags::BeginString);
        self.header.remove_field(tags::SenderCompID);
        self.header.remove_field(tags::SenderSubID);
        self.header.remove_field(tags::SenderLocationID);
        self.header.remove_field(tags::TargetCompID);
        self.header.remove_field(tags::TargetSubID);
        self.header.remove_field(tags::TargetLocationID);

        if let Some(begin_string) = header.get_field(tags::BeginString) {
            if begin_string.value().len() > 0 {
                self.header.set_tag_value(tags::BeginString, begin_string.value());
            }

            self.header.remove_field(tags::OnBehalfOfLocationID);
            self.header.remove_field(tags::DeliverToLocationID);

            let value: &[u8] = begin_string.value();
            if value >= b"FIX.4.1" {
                if let Some(field) = header.get_field(tags::OnBehalfOfLocationID) {
                    let on_behalf_of_location_id = field.value();
                    if on_behalf_of_location_id.len() > 0 {
                        self.header
                            .set_tag_value(tags::DeliverToLocationID, on_behalf_of_location_id);
                    }
                }

                if let Some(field) = header.get_field(tags::DeliverToLocationID) {
                    let deliver_to_location_id = field.value();
                    if deliver_to_location_id.len() > 0 {
                        self.header
                            .set_tag_value(tags::OnBehalfOfLocationID, deliver_to_location_id);
                    }
                }
            }
        }

        if let Some(field) = header.get_field(tags::SenderCompID) {
            let sender_comp_id = field.value();
            if sender_comp_id.len() > 0 {
                self.header.set_tag_value(tags::TargetCompID, sender_comp_id);
            }
        }

        if let Some(field) = header.get_field(tags::SenderSubID) {
            let sender_sub_id = field.value();
            if sender_sub_id.len() > 0 {
                self.header.set_tag_value(tags::TargetSubID, sender_sub_id);
            }
        }

        if let Some(field) = header.get_field(tags::SenderLocationID) {
            let sender_location_id = field.value();
            if sender_location_id.len() > 0 {
                self.header
                    .set_tag_value(tags::TargetLocationID, sender_location_id);
            }
        }

        if let Some(field) = header.get_field(tags::TargetCompID) {
            let target_comp_id = field.value();
            if target_comp_id.len() > 0 {
                self.header.set_tag_value(tags::SenderCompID, target_comp_id);
            }
        }

        if let Some(field) = header.get_field(tags::TargetSubID) {
            let target_sub_id = field.value();
            if target_sub_id.len() > 0 {
                self.header.set_tag_value(tags::SenderSubID, target_sub_id);
            }
        }

        if let Some(field) = header.get_field(tags::TargetLocationID) {
            let target_location_id = field.value();
            if target_location_id.len() > 0 {
                self.header
                    .set_tag_value(tags::SenderLocationID, target_location_id);
            }
        }

        // optional routing tags
        self.header.remove_field(tags::OnBehalfOfCompID);
        self.header.remove_field(tags::OnBehalfOfSubID);
        self.header.remove_field(tags::DeliverToCompID);
        self.header.remove_field(tags::DeliverToSubID);

        if let Some(field) = header.get_field(tags::OnBehalfOfCompID) {
            let on_behalf_of_comp_id = field.value();
            if on_behalf_of_comp_id.len() > 0 {
                self.header
                    .set_tag_value(tags::DeliverToCompID, on_behalf_of_comp_id);
            }
        }

        if let Some(field) = header.get_field(tags::OnBehalfOfSubID) {
            let on_behalf_of_sub_id = field.value();
            if on_behalf_of_sub_id.len() > 0 {
                self.header
                    .set_tag_value(tags::DeliverToSubID, on_behalf_of_sub_id);
            }
        }

        if let Some(field) = header.get_field(tags::DeliverToCompID) {
            let deliver_to_comp_id = field.value();
            if deliver_to_comp_id.len() > 0 {
                self.header
                    .set_tag_value(tags::OnBehalfOfCompID, deliver_to_comp_id);
            }
        }

        if let Some(field) = header.get_field(tags::DeliverToSubID) {
            let deliver_to_sub_id = field.value();
            if deliver_to_sub_id.len() > 0 {
                self.header
                    .set_tag_value(tags::OnBehalfOfSubID, deliver_to_sub_id);
            }
        }
    }

    pub fn extract_contra_session_id(&self) -> SessionId {
        SessionId::new(
            self.header
                .get_string(tags::BeginString)
                .unwrap_or_default(),
            self.header
                .get_string(tags::TargetCompID)
                .unwrap_or_default(),
            self.header
                .get_string(tags::TargetSubID)
                .unwrap_or_default(),
            self.header
                .get_string(tags::TargetLocationID)
                .unwrap_or_default(),
            self.header
                .get_string(tags::SenderCompID)
                .unwrap_or_default(),
            self.header
                .get_string(tags::SenderSubID)
                .unwrap_or_default(),
            self.header
                .get_string(tags::SenderLocationID)
                .unwrap_or_default(),
        )
    }
}

impl Deref for Message {
    type Target = FieldMap;
    fn deref(&self) -> &Self::Target {
        &self.body
    }
}

impl DerefMut for Message {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.body
    }
}

impl Display for Message {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.write_fmt(format_args!(
            "{}{}{}",
            self.header.calculate_string(),
            self.calculate_string(None),
            self.trailer.calculate_string()
        ))
    }
}

#[derive(Debug, Clone)]
pub enum MessageParseError {
    InvalidMessage(String),
    InvalidTagNumber(String),
    FailedToFindEqualsAt(usize),
    FailedToFindSohAt(usize),
    PosGreaterThanLen(usize, usize),
    RepeatedTagWithoutGroupDelimiterTagException(Tag, Tag),
    GroupDelimiterTagException(Tag, Tag),
    FieldMapError(FieldMapError),
    Malformed { tag: Tag, message: String },
    ConversionError(ConversionError),
}

impl From<FieldMapError> for MessageParseError {
    fn from(e: FieldMapError) -> MessageParseError {
        MessageParseError::FieldMapError(e)
    }
}

impl From<ConversionError> for MessageParseError {
    fn from(e: ConversionError) -> MessageParseError {
        MessageParseError::ConversionError(e)
    }
}

impl MessageParseError {
    pub fn as_tag(&self) -> Option<Tag> {
        match self {
            Self::InvalidMessage(_) => None,
            Self::InvalidTagNumber(_) => None,
            Self::FailedToFindEqualsAt(_) => todo!(),
            Self::FailedToFindSohAt(_) => todo!(),
            Self::PosGreaterThanLen(_, _) => todo!(),
            Self::RepeatedTagWithoutGroupDelimiterTagException(_num, _delim) => todo!(),
            Self::GroupDelimiterTagException(num, _delim) => Some(*num),
            Self::FieldMapError(_) => todo!(),
            Self::Malformed { tag, .. } => Some(*tag),
            Self::ConversionError(_) => todo!(),
        }
    }
    pub fn as_session_reject(self) -> Option<SessionRejectReason> {
        match self {
            Self::InvalidMessage(reason) => Some(SessionRejectReason::OTHER(reason)),
            Self::InvalidTagNumber(_) => Some(SessionRejectReason::INVALID_TAG_NUMBER()),
            Self::FailedToFindEqualsAt(_) => None,
            Self::FailedToFindSohAt(_) => None,
            Self::PosGreaterThanLen(_, _) => None,
            Self::RepeatedTagWithoutGroupDelimiterTagException(_num, _delim) => None,
            Self::GroupDelimiterTagException(num, delim) => Some(TagException::group_delimiter_tag_exception(num, delim).session_reject_reason().clone()),
            Self::FieldMapError(_) => None,
            Self::Malformed { tag: _, .. } => Some(SessionRejectReason::INVALID_MSGTYPE()),
            Self::ConversionError(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Message;
    use crate::data_dictionary::DataDictionary;
    use crate::message::MessageParseError;
    use crate::message_factory::DefaultMessageFactory;
    
    #[test]
    fn test_parse() {
        let dd = DataDictionary::from_file("../../spec/FIX44.xml").expect("Able to read FIX44.xml file.");
        println!("{:#?}", dd);

        let mut message = Message::default();

        // let msgstr = "8=FIXT.1.1\x019=73\x0135=W\x0134=3\x0149=sender\x0152=20110909-09:09:09.999\x0156=target\x0155=sym\x01268=1\x01269=0\x01272=20111012\x01273=22:15:30.444\x0110=249\x01";
        let expected = "8=FIX.4.4|9=115|35=A|34=1|49=sender-comp-id|52=20221025-10:49:30.969|56=target-comp-id|98=0|108=30|141=Y|553=username|554=password|10=159|";

        let msgstr = expected.replace('|', "\x01");
        let result =
            message.from_string::<DefaultMessageFactory>(msgstr.as_bytes(), true, Some(&dd), Some(&dd), None, false);
        println!("{:?}", result);
        assert!(result.is_ok());

        let actual = message.to_string_mut().replace(Message::SOH, "|");

        println!("{:?}", expected);
        println!("{:?}", actual);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_validate() {
        let dd = DataDictionary::from_file("../../spec/FIX44.xml").expect("Able to read FIX44.xml file.");

        let mut message = Message::default();

        // let msgstr = "8=FIXT.1.1\x019=73\x0135=W\x0134=3\x0149=sender\x0152=20110909-09:09:09.999\x0156=target\x0155=sym\x01268=1\x01269=0\x01272=20111012\x01273=22:15:30.444\x0110=249\x01";
        let expected = "8=FIX.4.4|9=115|35=A|34=1|49=sender-comp-id|52=20221025-10:49:30.969|56=target-comp-id|98=0|108=30|141=Y|553=username|554=password|10=159|";

        let msgstr = expected.replace('|', "\x01");
        let result =
            message.from_string::<DefaultMessageFactory>(msgstr.as_bytes(), true, Some(&dd), Some(&dd), None, false);
        println!("{:?}", result);
        assert!(result.is_ok());
        assert!(message.is_admin());

        let actual = message.to_string_mut().replace(Message::SOH, "|");

        println!("{:?}", expected);
        println!("{:?}", actual);
        assert_eq!(expected, actual);

        let result = DataDictionary::validate(&message, Some(&dd), &dd, "FIX.4.4", "A");
        println!("{:?}", message);
        println!("{:?}", result);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_msg_type_success() {
        let msgstr = "8=FIX.4.4|9=115|35=A|34=1|49=sender-comp-id|52=20221025-10:49:30.969|56=target-comp-id|98=0|108=30|141=Y|553=username|554=password|10=159|";
        let msgstr = msgstr.replace('|', "\x01");
        let msg_type = Message::get_msg_type(msgstr.as_bytes());
        assert!(msg_type.is_ok());
        assert!(matches!(msg_type, Ok(msg_type) if msg_type == "A"));
    }

    #[test]
    fn test_get_msg_type_failure() {
        let msgstr = "8=FIX.4.4|9=115|35=|34=1|49=sender-comp-id|52=20221025-10:49:30.969|56=target-comp-id|98=0|108=30|141=Y|553=username|554=password|10=159|";
        let msgstr = msgstr.replace('|', "\x01");
        let msg_type = Message::get_msg_type(msgstr.as_bytes());
        assert!(msg_type.is_err());
        assert!(matches!(
            msg_type,
            Err(MessageParseError::Malformed { .. })
        ));
    }

    #[test]
    fn test_get_msg_type_raw_data() {
        let dd = DataDictionary::from_file("../../spec/FIX44.xml").expect("Able to read FIX44.xml file.");

        let mut message = Message::default();

        // let msgstr = "8=FIXT.1.1\x019=73\x0135=W\x0134=3\x0149=sender\x0152=20110909-09:09:09.999\x0156=target\x0155=sym\x01268=1\x01269=0\x01272=20111012\x01273=22:15:30.444\x0110=249\x01";
        let expected = b"8=FIX.4.4|9=127|35=0|34=1|49=sender-comp-id|52=20221025-10:49:30.969|56=target-comp-id|90=3|91=\xC1\x01\xC0|98=0|108=30|141=Y|553=username|554=password|10=149|";
        let msgstr: Vec<u8> = expected
            .iter()
            .map(|b| if *b == b'|' { 1_u8 } else { *b })
            .collect();

        let result = message.from_string::<DefaultMessageFactory>(&msgstr, false, Some(&dd), Some(&dd), None, false);
        println!("{:?}", result);
        let actual = message.to_string().replace(Message::SOH, "|");
        println!("{}", actual);
        assert!(result.is_ok());
        assert!(message.is_admin());

        let actual = message.to_string_mut().replace(Message::SOH, "|");

        let msgstr: String = expected
            .iter()
            .map(|b| *b as char)
            .map(|c| if c == Message::SOH { '|' } else { c })
            .collect();

        println!("{:?}", expected);
        println!("{:?}", actual);

        assert_eq!(msgstr, actual);

        let result = DataDictionary::validate(&message, Some(&dd), &dd, "FIX.4.4", "A");
        println!("{:?}", message);
        println!("{:?}", result);
        assert!(result.is_ok());
    }
}
