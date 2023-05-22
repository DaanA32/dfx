use crate::field_map::FieldBase;
use crate::field_map::FieldMap;
use crate::field_map::FieldMapError;
use crate::field_map::Group;
use crate::field_map::Tag;
use crate::fields;
use crate::fields::ConversionError;
use crate::fields::types::FieldType;
use crate::fix_values::SessionRejectReason;
use crate::message::Message;
use crate::tags;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use chrono::NaiveTime;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashSet;
use std::fs::File;
use std::ops::Deref;
use std::ops::DerefMut;
use std::path::Path;

#[derive(Clone, Debug)]
pub enum MessageValidationError {
    UnsupportedVersion { expected: String, actual: String },
    TagException(TagException),
    FieldMapError(FieldMapError),
    // MissingGroupDefinition(),
    //DictionaryParseException(String),
    ConversionError(ConversionError),
    //InvalidStructure(u32),
}

#[derive(Clone, Debug)]
pub struct TagException {
    field: Tag,
    session_reject_reason: SessionRejectReason,
    inner: Option<String>, //todo
    msg_type: Option<String>, //todo
}

impl TagException {
    pub fn other(msg: String, tag: Tag) -> TagException {
        Self { field: tag, session_reject_reason: SessionRejectReason::OTHER(msg), inner: None, msg_type: None }
    }
    pub fn tag_out_of_order(tag: Tag) -> TagException {
        Self { field: tag, session_reject_reason: SessionRejectReason::TAG_SPECIFIED_OUT_OF_REQUIRED_ORDER(), inner: None, msg_type: None }
    }
    pub fn invalid_tag_number(tag: Tag) -> TagException {
        Self { field: tag, session_reject_reason: SessionRejectReason::INVALID_TAG_NUMBER(), inner: None, msg_type: None }
    }
    pub fn required_tag_missing(tag: Tag) -> TagException {
        Self { field: tag, session_reject_reason: SessionRejectReason::REQUIRED_TAG_MISSING(), inner: None, msg_type: None }
    }
    pub fn tag_not_defined_for_message(tag: Tag, msg_type: String) -> TagException {
        Self { field: tag, session_reject_reason: SessionRejectReason::TAG_NOT_DEFINED_FOR_THIS_MESSAGE_TYPE(), inner: None, msg_type: Some(msg_type) }
    }
    pub fn no_tag_value(tag: Tag) -> TagException {
        Self { field: tag, session_reject_reason: SessionRejectReason::TAG_SPECIFIED_WITHOUT_A_VALUE(), inner: None, msg_type: None }
    }
    pub fn incorrect_tag_value(tag: Tag) -> TagException {
        Self { field: tag, session_reject_reason: SessionRejectReason::VALUE_IS_INCORRECT(), inner: None, msg_type: None }
    }
    pub fn repeated_tag(tag: Tag) -> TagException {
        Self { field: tag, session_reject_reason: SessionRejectReason::TAG_APPEARS_MORE_THAN_ONCE(), inner: None, msg_type: None }
    }
    pub fn incorrect_data_format(tag: Tag, inner: String) -> TagException {
        Self { field: tag, session_reject_reason: SessionRejectReason::INCORRECT_DATA_FORMAT_FOR_VALUE(), inner: Some(inner), msg_type: None }
    } //TODO inner
    pub fn invalid_message_type() -> TagException {
        Self { field: tags::MsgType, session_reject_reason: SessionRejectReason::INVALID_MSGTYPE(), inner: None, msg_type: None }
    }
    pub fn repeating_group_count_mismatch(tag: Tag) -> TagException {
        Self { field: tag, session_reject_reason: SessionRejectReason::INCORRECT_NUM_IN_GROUP_COUNT_FOR_REPEATING_GROUP(), inner: None, msg_type: None }
    }
    pub fn group_delimiter_tag_exception(counter_tag: Tag, delimiter_tag: Tag) -> TagException {
        Self { field: counter_tag, session_reject_reason: SessionRejectReason::OTHER(format!("Group {counter_tag}'s first entry does not start with delimiter {delimiter_tag}")), inner: None, msg_type: None }
    }
    pub fn repeated_tag_without_group_delimiter_tag_exception(counter_tag: Tag, trouble_tag: Tag) -> TagException {
        Self { field: counter_tag, session_reject_reason: SessionRejectReason::OTHER(format!("Group {counter_tag} contains a repeat occurrence of tag {trouble_tag} in a single group, which is illegal.")), inner: None, msg_type: None }
    }

    pub fn msg_type(&self) -> Option<&String> {
        self.msg_type.as_ref()
    }

    pub fn inner(&self) -> Option<&String> {
        self.inner.as_ref()
    }

    pub fn session_reject_reason(&self) -> &SessionRejectReason {
        &self.session_reject_reason
    }

    pub fn field(&self) -> Tag {
        self.field
    }
}

impl From<FieldMapError> for MessageValidationError {
    fn from(e: FieldMapError) -> Self {
        MessageValidationError::FieldMapError(e)
    }
}

impl From<ConversionError> for MessageValidationError {
    fn from(e: ConversionError) -> Self {
        MessageValidationError::ConversionError(e)
    }
}

#[derive(Debug)]
/// TODO
pub enum DataDictionaryError {
    DeserializeError(serde_xml_rs::Error),
}

impl From<serde_xml_rs::Error> for DataDictionaryError {
    fn from(error: serde_xml_rs::Error) -> Self {
        Self::DeserializeError(error)
    }
}

#[derive(Clone, Debug, Default)]
pub struct DataDictionary {
    check_fields_have_values: bool,
    check_fields_out_of_order: bool,
    check_user_defined_fields: bool,
    allow_unknown_message_fields: bool,
    version: Option<String>,
    fields_by_tag: BTreeMap<Tag, DDField>,
    fields_by_name: BTreeMap<String, DDField>,
    messages: BTreeMap<String, DDMap>,
    // spec: FixSpec,
    header: DDMap,
    trailer: DDMap,
}


impl DataDictionary {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<DataDictionary, DataDictionaryError> {
        //let reader = File::open(path).unwrap();
        let path: &Path = path.as_ref();
        let mut reader = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", path.display(), why),
            Ok(file) => file,
        };

        let mut contents = String::new();
        reader.read_to_string(&mut contents).unwrap();
        let dd = DataDictionary::load_from_string(&contents).unwrap();
        Ok(dd)
    }

    pub fn version(&self) -> Option<&String> {
        self.version.as_ref()
    }

    pub fn header(&self) -> &DDMap {
        &self.header
    }

    pub fn trailer(&self) -> &DDMap {
        &self.trailer
    }

    pub fn is_header_field(&self, tag: Tag) -> bool {
        self.header.is_field(tag)
    }
    pub fn is_trailer_field(&self, tag: Tag) -> bool {
        self.trailer.is_field(tag)
    }

    pub fn validate(
        message: &Message,
        session_data_dictionary: Option<&DataDictionary>,
        app_data_dictionary: &DataDictionary,
        begin_string: &str,
        msg_type: &str,
    ) -> Result<(), MessageValidationError> {
        // bool bodyOnly = (null == sessionDataDict);

        // if ((null != sessionDataDict) && (null != sessionDataDict.Version))
        //     if (!sessionDataDict.Version.Equals(beginString))
        //         throw new UnsupportedVersion(beginString);
        if let Some(dictionary) = session_data_dictionary {
            if matches!(dictionary.version(), Some(version) if version != begin_string) {
                return Err(MessageValidationError::UnsupportedVersion {
                    expected: dictionary.version().unwrap().into(),
                    actual: begin_string.into(),
                });
            }
        }
        // println!("Checked version.");

        // if (((null != sessionDataDict) && sessionDataDict.CheckFieldsOutOfOrder) || ((null != appDataDict) && appDataDict.CheckFieldsOutOfOrder))
        // {
        //     int field;
        //     if (!message.HasValidStructure(out field))
        //         throw new TagOutOfOrder(field);
        // }
        let check_order_session = session_data_dictionary
            .map(|d| d.check_fields_out_of_order())
            .unwrap_or(false);
        let check_order_app = app_data_dictionary.check_fields_out_of_order();
        if check_order_session || check_order_app {
            // println!("valid_structure");
            message.has_valid_structure()?;
        }
        // println!("Checked structure.");

        // if ((null != appDataDict) && (null != appDataDict.Version))
        // {
        //     appDataDict.CheckMsgType(msgType);
        //     appDataDict.CheckHasRequired(message, msgType);
        // }
        if app_data_dictionary.version().is_some() {
            app_data_dictionary.check_msg_type(msg_type)?;
            app_data_dictionary.check_has_required(message, msg_type)?;
        }
        // println!("Checked msg_type.");

        // if (!bodyOnly)
        // {
        //     sessionDataDict.Iterate(message.Header, msgType);
        //     sessionDataDict.Iterate(message.Trailer, msgType);
        // }
        if let Some(dictionary) = session_data_dictionary {
            dictionary.iterate(message.header(), msg_type)?;
            dictionary.iterate(message.trailer(), msg_type)?;
        }
        // println!("Checked header & trailer.");

        // appDataDict.Iterate(message, msgType);
        app_data_dictionary.iterate(message, msg_type)?;
        // println!("Checked body.");
        Ok(())
    }

    fn check_msg_type(&self, msg_type: &str) -> Result<(), MessageValidationError> {
        if self.messages.contains_key(msg_type) {
            Ok(())
        } else {
            //TODO should this accept msg_type?
            Err(MessageValidationError::TagException(TagException::invalid_message_type()))
        }
    }
    fn check_has_required(
        &self,
        message: &Message,
        msg_type: &str,
    ) -> Result<(), MessageValidationError> {
        // foreach (int field in Header.ReqFields)
        // {
        //     if (!message.Header.IsSetField(field))
        //         throw new RequiredTagMissing(field);
        // }
        for field in self.header.required_fields() {
            if !message.header().is_field_set(*field) {
                // println!("missing header_field");
                return Err(MessageValidationError::TagException(TagException::required_tag_missing(*field)));
            }
        }

        // foreach (int field in Trailer.ReqFields)
        // {
        //     if (!message.Trailer.IsSetField(field))
        //         throw new RequiredTagMissing(field);
        // }
        for field in self.trailer.required_fields() {
            if !message.trailer().is_field_set(*field) {
                // println!("missing trailer_field");
                return Err(MessageValidationError::TagException(TagException::required_tag_missing(*field)));
            }
        }

        // foreach (int field in Messages[msgType].ReqFields)
        // {
        //     if (!message.IsSetField(field))
        //         throw new RequiredTagMissing(field);
        // }
        for field in self.messages[msg_type].required_fields() {
            if !message.is_field_set(*field) {
                // println!("missing body_field {msg_type} {:?}", self.messages[msg_type].required_fields());
                return Err(MessageValidationError::TagException(TagException::required_tag_missing(*field)));
            }
        }
        Ok(())
    }
    fn check_has_no_repeated_tags(map: &FieldMap) -> Result<(), MessageValidationError> {
        if !map.repeated_tags().is_empty() {
            Err(MessageValidationError::TagException(
                TagException::repeated_tag(map.repeated_tags().get(0).unwrap().tag()),
            ))
        } else {
            Ok(())
        }
    }
    fn check_fields_out_of_order(&self) -> bool {
        self.check_fields_out_of_order
    }
    fn check_has_value(&self, field: &FieldBase) -> Result<(), MessageValidationError> {
        if self.check_fields_have_values && field.value().is_empty() {
            Err(MessageValidationError::TagException(TagException::no_tag_value(field.tag())))
        } else {
            Ok(())
        }
    }
    fn check_valid_format(&self, field: &FieldBase) -> Result<(), MessageValidationError> {
        // TODO check format based on type received.
        // println!("check_valid_format");
        if let Some(field_definition) = self.fields_by_tag.get(&field.tag()) {
            let field_type = FieldType::get(field_definition.field_type().as_str());
            // println!("{field_type:?}");
            // println!("{field:?}");
            if matches!(field_type, Ok(ftype) if ftype == fields::types::FieldType::String) {
                return Ok(());
            }

            if !self.check_fields_have_values && field.value().len() < 1 {
                return Ok(());
            }

            let err = match field_type {
                Ok(ftype) => match ftype {
                    FieldType::Boolean => field.as_value::<bool>().err(),
                    FieldType::Char => { field.as_value::<char>().err() },
                    FieldType::DateOnly => { field.as_value::<NaiveDate>().err() },
                    FieldType::DateTime => { field.as_value::<NaiveDateTime>().err() },
                    FieldType::Decimal => { field.as_value::<f32>().err() },
                    FieldType::Int => { field.as_value::<i32>().err() },
                    FieldType::String => unreachable!(),
                    FieldType::TimeOnly => { field.as_value::<NaiveTime>().err() },
                },
                Err(msg) => todo!("{msg}"),
            };
            if let Some(e) = err {
                Err(MessageValidationError::TagException(TagException::incorrect_data_format(field.tag(), format!("{e:?}"))))
            } else {
                Ok(())
            }

        } else {
            Ok(())
        }
        // try
        //     {
        //         Type type;
        //         if (!TryGetFieldType(field.Tag, out type))
        //             return;
        //         if (type == typeof(StringField))
        //             return;

        //         if (false == CheckFieldsHaveValues && field.ToString().Length < 1)
        //         {
        //             // If ValidateFieldsHaveValues=N, don't check empty non-string fields
        //             // because engine should not decide how to convert empty to e.g. float or datetime.
        //             // (User code may see IncorrectDataFormat exceptions
        //             //  when attempting to extract fields in not-string formats.)
        //             return;
        //         }

        //         if (type == typeof(CharField))
        //             Fields.Converters.CharConverter.Convert(field.ToString());
        //         else if (type == typeof(IntField))
        //             Fields.Converters.IntConverter.Convert(field.ToString());
        //         else if (type == typeof(DecimalField))
        //             Fields.Converters.DecimalConverter.Convert(field.ToString());
        //         else if (type == typeof(BooleanField))
        //             Fields.Converters.BoolConverter.Convert(field.ToString());

        //         else if (type == typeof(DateTimeField))
        //             Fields.Converters.DateTimeConverter.ConvertToDateTime(field.ToString());
        //         else if (type == typeof(DateOnlyField))
        //             Fields.Converters.DateTimeConverter.ConvertToDateOnly(field.ToString());
        //         else if (type == typeof(TimeOnlyField))
        //             Fields.Converters.DateTimeConverter.ConvertToTimeOnly(field.ToString());
        //         return;

        //     }
        //     catch (FieldConvertError e)
        //     {
        //         throw new IncorrectDataFormat(field.Tag, e);
        //     }
    }
    fn check_valid_tag_number(&self, tag: Tag) -> Result<(), MessageValidationError> {
        // if (AllowUnknownMessageFields)
        //     return;
        // if (!FieldsByTag.ContainsKey(tag))
        // {
        //     throw new InvalidTagNumber(tag);
        // }
        if !self.allow_unknown_message_fields && !self.fields_by_tag.contains_key(&tag) {
            return Err(MessageValidationError::TagException(TagException::invalid_tag_number(tag)));
        }
        Ok(())
    }
    fn check_value(&self, field: &FieldBase) -> Result<(), MessageValidationError> {
        match self.fields_by_tag.get(&field.tag()) {
            Some(fld) => {
                // println!("{:?}", fld);
                if fld.has_enums() {
                    if fld.is_multiple_value_field_with_enums() {
                        let string_value = field.string_value()?;
                        let splitted = string_value.split(' ');
                        for value in splitted {
                            if !fld.enums().contains_key(value) {
                                return Err(MessageValidationError::TagException(
                                    TagException::incorrect_tag_value(field.tag())
                                    // field.tag(),
                                    // value.to_string(),
                                ));
                            }
                        }
                        Ok(())
                    } else if !fld.enums().contains_key(&field.string_value()?) {
                        // println!("{:?}", field);
                        // println!("{:?}", fld.enums());
                        Err(MessageValidationError::TagException(
                            TagException::incorrect_tag_value(field.tag())
                            // field.tag(),
                            // value.to_string(),
                        ))
                        // Err(MessageValidationError::IncorrectEnumValue(
                        //     field.tag(),
                        //     field.string_value()?,
                        // ))
                    } else {
                        Ok(())
                    }
                } else {
                    Ok(())
                }
            }
            None => Ok(()),
        }
    }
    fn check_is_in_message(
        &self,
        field: &FieldBase,
        msg_type: &str,
    ) -> Result<(), MessageValidationError> {
        // println!("allow_unknown_message_fields: {}", self.allow_unknown_message_fields);
        if self.allow_unknown_message_fields {
            return Ok(());
        }

        let fields: Vec<&String> = self.messages.get(msg_type).iter()
            .map(|f| f.name())
            .collect();
        // println!("{} in message: {:?}", msg_type, fields);
        // println!("{} in message: {:?}", msg_type, self.messages.get(msg_type).map(|f| f.fields.len()));
        // if let Some(field) = self.messages.get(msg_type) {
        //     println!("{} in message: {:?}", msg_type, field.fields.iter()
        //     .map(|f| f.1.name())
        //     .collect::<Vec<&String>>());
        // }
        if matches!(self.messages.get(msg_type), Some(dd) if dd.fields.contains_key(&field.tag())) {
            return Ok(());
        }
        // Err(MessageValidationError::TagNotDefinedForMessage(
        //     field.tag(),
        //     msg_type.into(),
        // ))
        Err(MessageValidationError::TagException(
            TagException::tag_not_defined_for_message(field.tag(), msg_type.into())
        ))
    }
    fn check_is_in_group(
        &self,
        field: &FieldBase,
        dd_group: &DDGroup,
        msg_type: &str,
    ) -> Result<(), MessageValidationError> {
        // if (ddgrp.IsField(field.Tag))
        //     return;
        // throw new TagNotDefinedForMessage(field.Tag, msgType);
        if dd_group.is_field(field.tag()) {
            Ok(())
        } else {
            // Err(MessageValidationError::TagNotDefinedForMessage(
            //     field.tag(),
            //     msg_type.into(),
            // ))
            Err(MessageValidationError::TagException(
                TagException::tag_not_defined_for_message(field.tag(), msg_type.into())
            ))
        }
    }
    fn check_group_count(
        &self,
        field: &FieldBase,
        map: &FieldMap,
        msg_type: &str,
    ) -> Result<(), MessageValidationError> {
        // if (IsGroup(msgType, field.Tag))
        // {
        //     if (map.GetInt(field.Tag) != map.GroupCount(field.Tag))
        //     {
        //         throw new RepeatingGroupCountMismatch(field.Tag);
        //     }
        // }
        if self.is_group(msg_type, field.tag())
            && map.get_int(field.tag())? as usize != map.group_count(field.tag()).unwrap_or(0)
        {
            // return Err(MessageValidationError::RepeatingGroupCountMismatch(
            //     field.tag(),
            // ));
            return Err(MessageValidationError::TagException(
                TagException::repeating_group_count_mismatch(field.tag())
            ));
        }
        Ok(())
    }
    fn is_group(&self, msg_type: &str, tag: Tag) -> bool {
        // if (Messages.ContainsKey(msgType))
        // {
        //     return Messages[msgType].IsGroup(tag);
        // }
        // return false;
        if self.messages.contains_key(msg_type) {
            return self.messages[msg_type].is_group(tag);
        }
        false
    }
    fn should_check_tag(&self, field: &FieldBase) -> bool {
        // println!("check: {} tag: {}", self.check_user_defined_fields, field.tag());
        if !self.check_user_defined_fields && (field.tag() >= fields::limits::USER_MIN) {
            return false;
        }
        true
    }

    fn iterate(&self, message: &FieldMap, msg_type: &str) -> Result<(), MessageValidationError> {
        // DataDictionary.CheckHasNoRepeatedTags(map);
        DataDictionary::check_has_no_repeated_tags(message)?;
        // println!("Checked no repeated tags");

        // check non-group fields
        // int lastField = 0;
        // foreach (KeyValuePair<int, Fields.IField> kvp in map)
        // {
        let mut last_field = 0;
        for (_k, v) in message.entries() {
            // Fields.IField field = kvp.Value;
            let field = v;
            // if (lastField != 0 && field.Tag == lastField)
            //     throw new RepeatedTag(lastField);
            if last_field != 0 && field.tag() == last_field {
                // return Err(MessageValidationError::RepeatedTag(field.tag()));
                return Err(MessageValidationError::TagException(TagException::repeated_tag(field.tag())));
            }
            // CheckHasValue(field);
            self.check_has_value(field)?;

            // if (!string.IsNullOrEmpty(this.Version))
            if !self.version.is_none() && !matches!(&self.version, Some(version) if version.is_empty()) {
                // CheckValidFormat(field);
                self.check_valid_format(field)?;

                // if (ShouldCheckTag(field))
                if self.should_check_tag(field) {
                    // CheckValidTagNumber(field.Tag);
                    // println!("check_valid_tag_number");
                    self.check_valid_tag_number(field.tag())?;

                    // CheckValue(field);
                    // println!("check_value");
                    self.check_value(field)?;
                    // if (!Message.IsHeaderField(field.Tag, this) && !Message.IsTrailerField(field.Tag, this))
                    if !Message::is_header_field(field.tag(), Some(self))
                        && !Message::is_trailer_field(field.tag(), Some(self))
                    {
                        // println!("body field");
                        // CheckIsInMessage(field, msgType);
                        self.check_is_in_message(field, msg_type)?;
                        // CheckGroupDefinitionCount(field, map, msgType);
                        // println!("check_group_count");
                        self.check_group_count(field, message, msg_type)?;
                        // println!("check_group_count_end");
                    } else {
                        // println!("header or trailer field");
                    }
                }
            }

            // lastField = field.Tag;
            last_field = field.tag();
        }
        // println!("Checked fields.");

        // check contents of each group
        // foreach (int groupTag in map.GetGroupDefinitionTags())
        for tag in message.group_tags() {
            // for (int i = 1; i <= map.GroupDefinitionCount(groupTag); i++)
            // println!("Checking group: {tag}");
            for i in 1..=message.group_count(*tag)? {
                // GroupDefinition g = map.GetGroupDefinition(i, groupTag);
                // DDGrp ddg = this.Messages[msgType].GetGroupDefinition(groupTag);
                // IterateGroupDefinition(g, ddg, msgType);
                // println!("start group {tag}:{i}");
                let g = message.get_group(i as u32, *tag)?;
                // println!("get group {tag}:{i}");
                let ddg = self.messages[msg_type].get_group(*tag);
                // println!("end group {tag}:{i}");
                self.iterate_group(g, ddg, msg_type)?;
                // println!("end group {tag}:{i}");
            }
            // println!("Checked group: {tag}");
        }
        // println!("Checked groups.");

        Ok(())
    }

    fn iterate_group(
        &self,
        group: &Group,
        group_definition: Option<&DDGroup>,
        msg_type: &str,
    ) -> Result<(), MessageValidationError> {
        // println!("-----------------------");
        // println!("{group:?}");
        if group_definition.is_none() {
            //return Err(MessageValidationError::MissingGroupDefinition());
            return Ok(());
        }
        let group_definition = group_definition.unwrap();
        // DataDictionary.CheckHasNoRepeatedTags(group);
        DataDictionary::check_has_no_repeated_tags(group)?;
        // println!("Checked has no repeated tags group");

        // int lastField = 0;
        let mut last_field = 0;
        // foreach (KeyValuePair<int, Fields.IField> kvp in group)
        for (_, v) in group.entries() {
            // println!("{v:?}");
            let field = v;

            // if (lastField != 0 && field.Tag == lastField)
            //     throw new RepeatedTag(lastField);
            if last_field != 0 && field.tag() == last_field {
                return Err(MessageValidationError::TagException(TagException::repeated_tag(last_field)));
            }
            // println!("repeated tag");
            // CheckHasValue(field);
            self.check_has_value(field)?;
            // println!("check_has_value");

            // if (!string.IsNullOrEmpty(this.Version))
            if !self.version.is_none() && !matches!(&self.version, Some(version) if version.is_empty()) {
                // println!("version match");
                // CheckValidFormat(field);
                self.check_valid_format(field)?;
                // println!("check_valid_format");

                // if (ShouldCheckTag(field))
                if self.should_check_tag(field) {
                    // CheckValidTagNumber(field.Tag);
                    self.check_valid_tag_number(field.tag())?;

                    // CheckValue(field);
                    self.check_value(field)?;
                    // CheckIsInGroup(field, ddgroup, msgType);
                    self.check_is_in_group(field, group_definition, msg_type)?;
                    // CheckGroupCount(field, map, msgType);
                    self.check_group_count(field, group, msg_type)?;
                }
            }
            last_field = field.tag();
        }
        // println!("Checked fields group");

        // check contents of each nested group
        // foreach (int groupTag in map.GetGroupTags())
        for tag in group.group_tags() {
            // println!("Checking group nested: {tag}");
            // for (int i = 1; i <= map.GroupCount(groupTag); i++)
            for i in 1..=group.group_count(*tag)? {
                // Group g = group.GetGroup(i, groupTag);
                // DDGrp ddg = ddgroup.GetGroup(groupTag);
                // IterateGroup(g, ddg, msgType);

                // println!("start group {tag}:{i}");
                let g = group.get_group(i as u32, *tag)?;
                // println!("get group {tag}:{i}");
                let ddg = group_definition.get_group(*tag);
                // println!("end group {tag}:{i}");
                self.iterate_group(g, ddg, msg_type)?;
                // println!("end group {tag}:{i}");
            }
            // println!("Checked group nested: {tag}");
        }
        // println!("Checked groups in group");

        Ok(())
    }

    pub fn get_map_for_message(&self, msg_type: &str) -> Option<&DDMap> {
        self.messages.get(msg_type)
    }

    pub fn get_field_by_name(&self, field_name: &str) -> Option<&DDField> {
        self.fields_by_name.get(field_name)
    }

    pub(crate) fn is_length_field(&self, tag: Tag) -> bool {
        match self.fields_by_tag.get(&tag) {
            Some(field) => field.field_type() == "LENGTH" && field.name() != "BodyLength",
            None => false,
        }
    }

    pub fn fields_by_name(&self) -> &BTreeMap<String, DDField> {
        &self.fields_by_name
    }

    pub fn messages(&self) -> &BTreeMap<String, DDMap> {
        &self.messages
    }


    pub fn check_fields_have_values(&self) -> bool {
        self.check_fields_have_values
    }

    pub fn check_fields_have_values_mut(&mut self) -> &mut bool {
        &mut self.check_fields_have_values
    }

    pub fn set_check_fields_have_values(&mut self, check_fields_have_values: bool) {
        self.check_fields_have_values = check_fields_have_values;
    }

    pub fn check_fields_out_of_order_mut(&mut self) -> &mut bool {
        &mut self.check_fields_out_of_order
    }

    pub fn set_check_fields_out_of_order(&mut self, check_fields_out_of_order: bool) {
        self.check_fields_out_of_order = check_fields_out_of_order;
    }

    pub fn check_user_defined_fields(&self) -> bool {
        self.check_user_defined_fields
    }

    pub fn check_user_defined_fields_mut(&mut self) -> &mut bool {
        &mut self.check_user_defined_fields
    }

    pub fn set_check_user_defined_fields(&mut self, check_user_defined_fields: bool) {
        self.check_user_defined_fields = check_user_defined_fields;
    }

    pub fn allow_unknown_message_fields(&self) -> bool {
        self.allow_unknown_message_fields
    }

    pub fn allow_unknown_message_fields_mut(&mut self) -> &mut bool {
        &mut self.allow_unknown_message_fields
    }

    pub fn set_allow_unknown_message_fields(&mut self, allow_unknown_message_fields: bool) {
        self.allow_unknown_message_fields = allow_unknown_message_fields;
    }
}

#[derive(Default, Debug, Clone)]
pub struct DDMap {
    fields: BTreeMap<Tag, DDField>,
    groups: BTreeMap<Tag, DDGroup>,
    required_fields: BTreeSet<Tag>,
    name: String,
    msg_type: String,
    admin: bool,
}
impl DDMap {
    pub fn new(name: String) -> Self {
        DDMap {
            fields: BTreeMap::default(),
            groups: BTreeMap::default(),
            required_fields: BTreeSet::default(),
            name,
            msg_type: "".into(),
            admin: false,
        }
    }
    pub fn new_with_values(name: String, msg_type: String, admin: bool) -> Self {
        DDMap {
            fields: BTreeMap::default(),
            groups: BTreeMap::default(),
            required_fields: BTreeSet::default(),
            name,
            msg_type,
            admin,
        }
    }
    pub fn add_field(&mut self, field: DDField) {
        self.fields.insert(field.tag(), field);
    }
    pub fn is_field(&self, tag: Tag) -> bool {
        self.fields.contains_key(&tag)
    }
    pub fn get_field(&self, tag: Tag) -> Option<&DDField> {
        self.fields.get(&tag)
    }
    pub fn add_group(&mut self, group: DDGroup) {
        self.groups.insert(group.delim(), group);
    }
    pub fn is_group(&self, tag: Tag) -> bool {
        self.groups.contains_key(&tag)
    }
    pub fn get_group(&self, tag: Tag) -> Option<&DDGroup> {
        self.groups.get(&tag)
    }
    pub fn required_fields(&self) -> &BTreeSet<Tag> {
        &self.required_fields
    }
    pub fn required_fields_mut(&mut self) -> &mut BTreeSet<Tag> {
        &mut self.required_fields
    }
    pub fn add_required_field(&mut self, tag: Tag) {
        self.required_fields.insert(tag);
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn fields(&self) -> &BTreeMap<Tag, DDField> {
        &self.fields
    }
    pub fn groups(&self) -> &BTreeMap<Tag, DDGroup> {
        &self.groups
    }

    pub fn admin(&self) -> bool {
        self.admin
    }

    pub fn msg_type(&self) -> &str {
        self.msg_type.as_ref()
    }
}
trait AsDDMap {
    fn as_map(&self) -> &DDMap;
    fn as_map_mut(&mut self) -> &mut DDMap;
}
impl AsDDMap for DDMap {
    fn as_map(&self) -> &DDMap {
        self
    }
    fn as_map_mut(&mut self) -> &mut DDMap {
        self
    }
}
impl<D: DerefMut<Target = DDMap>> AsDDMap for D {
    fn as_map(&self) -> &DDMap {
        self.deref()
    }
    fn as_map_mut(&mut self) -> &mut DDMap {
        self.deref_mut()
    }
}

#[derive(Clone, Debug)]
pub enum DictionaryError {
    ParseError(String),
}

#[derive(Debug, Clone)]
pub struct DDField {
    // public int Tag;
    tag: Tag,
    // public String Name;
    name: String,
    // public Dictionary<String, String> EnumDict;
    enum_dictionary: BTreeMap<String, String>,
    // public String FixFldType;
    field_type: String,
    // TODO type?
    // public Type FieldType;
    is_multiple_value_field_with_enums: bool,
}
impl DDField {

    pub fn from_xml_str(xml_str: &str) -> Self {
        todo!("DataDictionary::from_xml_str({xml_str})")
    }

    pub fn new(
        // public int Tag;
        tag: Tag,
        // public String Name;
        name: String,
        // public Dictionary<String, String> EnumDict;
        enum_dictionary: BTreeMap<String, String>,
        // public String FixFldType;
        field_type: String,
        // TODO type?
        // public Type FieldType;
        // is_multiple_value_field_with_enums: bool
    ) -> Self {
        // case "MULTIPLEVALUESTRING": multipleValueFieldWithEnums = true; return typeof( Fields.StringField );
        // case "MULTIPLESTRINGVALUE": multipleValueFieldWithEnums = true; return typeof( Fields.StringField );
        // case "MULTIPLECHARVALUE": multipleValueFieldWithEnums = true; return typeof( Fields.StringField );
        let is_multiple_value_field_with_enums = matches!(
            field_type.as_str(),
            "MULTIPLEVALUESTRING" | "MULTIPLESTRINGVALUE" | "MULTIPLECHARVALUE"
        );
        DDField {
            tag,
            name,
            enum_dictionary,
            field_type,
            is_multiple_value_field_with_enums,
        }
    }
    pub fn tag(&self) -> Tag {
        self.tag
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn has_enums(&self) -> bool {
        !self.enum_dictionary.is_empty()
    }
    pub fn enums(&self) -> &BTreeMap<String, String> {
        &self.enum_dictionary
    }
    pub fn field_type(&self) -> &String {
        &self.field_type
    }
    pub fn is_multiple_value_field_with_enums(&self) -> bool {
        self.is_multiple_value_field_with_enums
    }
}

#[derive(Default, Debug, Clone)]
pub struct DDGroup {
    num_fld: Tag,
    delim: Tag,
    required: bool,
    name: String,
    map: DDMap,
}
impl DDGroup {
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn num_fld(&self) -> Tag {
        self.num_fld
    }
    pub fn delim(&self) -> Tag {
        self.delim
    }
    pub fn required(&self) -> bool {
        self.required
    }
}
impl Deref for DDGroup {
    type Target = DDMap;
    fn deref(&self) -> &Self::Target {
        &self.map
    }
}
impl DerefMut for DDGroup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

#[derive(Debug)]
enum GoM<'a> {
    Map(&'a mut DDMap),
    Group(&'a mut DDGroup)
}
impl<'a> Deref for GoM<'a> {
    type Target = DDMap;
    fn deref(&self) -> &Self::Target {
        match self {
            GoM::Map(g) => g,
            GoM::Group(g) => g,
        }
    }
}
impl<'a> DerefMut for GoM<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            GoM::Map(g) => g,
            GoM::Group(g) => g,
        }
    }
}

use std::io::Read;
use std::println;
use xmltree::Element;

impl DataDictionary {
    pub fn new() -> DataDictionary {
        DataDictionary {
            version: None,
            fields_by_tag: BTreeMap::new(),
            fields_by_name: BTreeMap::new(),
            messages: BTreeMap::new(),
            check_fields_out_of_order: true,
            check_fields_have_values: true,
            check_user_defined_fields: true,
            allow_unknown_message_fields: false,
            header: DDMap::default(),
            trailer: DDMap::default(),
        }
    }

    pub fn load(&mut self, path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        Self::load_from_string(&contents)
    }

    pub fn load_from_string(contents: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let root_doc = Element::parse(contents.as_bytes())?;

        let (major_version, minor_version, version) = get_version_info(&root_doc);
        let (fields_by_tag, fields_by_name) = parse_fields(&root_doc);
        let components_by_name = cache_components(&root_doc);
        let messages = parse_messages(&root_doc, &fields_by_name, &components_by_name);
        let header = parse_header(&root_doc, &fields_by_name, &components_by_name);
        let trailer = parse_trailer(&root_doc, &fields_by_name, &components_by_name);

        Ok(DataDictionary {
            version: Some(version),
            fields_by_tag,
            fields_by_name,
            messages,
            check_fields_out_of_order: true,
            check_fields_have_values: true,
            check_user_defined_fields: true,
            allow_unknown_message_fields: false,
            header,
            trailer,
        })
    }

}

fn get_version_info(doc: &Element) -> (String, String, String) {
    let major_version = doc.attributes.get("major").unwrap().to_string();
    let minor_version = doc.attributes.get("minor").unwrap().to_string();
    let version = "FIX".to_string();
    let version_type = doc.attributes.get("type").unwrap_or(&version);
    if version_type != "FIX" && version_type != "FIXT" {
        panic!("Type must be FIX or FIXT in config");
    }
    let version = format!("{}.{}.{}", version_type, major_version, minor_version);
    return (major_version, minor_version, version);
}

fn parse_fields(doc: &Element) -> (BTreeMap<i32, DDField>, BTreeMap<String, DDField>) {
    let mut fields_by_tag: BTreeMap<i32, DDField> = BTreeMap::new();
    let mut fields_by_name: BTreeMap<String, DDField> = BTreeMap::new();
    let field_nodes = doc
        .children.iter()
        .filter_map(|c| c.as_element())
        .filter(|c| c.name == "fields")
        .flat_map(|node| node.children.iter())
        .filter_map(|c| c.as_element())
        .filter(|node| node.name == "field");

    for field_node in field_nodes {
        let tag_str = field_node.attributes.get("number").unwrap();
        let name = field_node.attributes.get("name").unwrap();
        let field_type = field_node.attributes.get("type").unwrap();

        let tag = tag_str.parse::<i32>().unwrap();
        let mut enums = BTreeMap::new();
        // if let Some(enum_nodes) = field_node.get_child("value") {
        //     for enum_node in enum_nodes.children.iter() {
        //         println!("{:?}", enum_node);
        //         let enum_value = enum_node.as_element().unwrap().attributes.get("enum").unwrap().clone();
        //         let description = enum_node.as_element().unwrap().attributes.get("description").map(|s| s.clone()).unwrap_or_default();
        //         enums.insert(enum_value, description);
        //     }
        // }
        for enum_node in field_node.children.iter()
            .filter_map(|c| c.as_element())
            .filter(|c| c.name == "value")
        {
            let enum_value = enum_node.attributes.get("enum").unwrap().clone();
            let description = enum_node.attributes.get("description").map(|s| s.clone()).unwrap_or_default();
            enums.insert(enum_value, description);
        }

        let is_multiple_value_field_with_enums = matches!(
            field_type.as_str(),
            "MULTIPLEVALUESTRING" | "MULTIPLESTRINGVALUE" | "MULTIPLECHARVALUE"
        );

        let dd_field = DDField {
            tag,
            name: name.clone(),
            enum_dictionary: enums,
            field_type: field_type.clone(),
            is_multiple_value_field_with_enums
        };

        fields_by_tag.insert(tag, dd_field.clone());
        fields_by_name.insert(name.clone(), dd_field);
    }
    return (fields_by_tag, fields_by_name);
}

fn cache_components(doc: &Element) -> BTreeMap<String, Element> {
    let mut components_by_name: BTreeMap<String, Element> = BTreeMap::new();
    let component_nodes = doc
        .children.iter()
        .filter_map(|c| c.as_element())
        .filter(|c| c.name == "components")
        .flat_map(|node| node.children.iter())
        .filter_map(|c| c.as_element())
        .filter(|node| node.name == "component");

    for component_node in component_nodes {
        let name = component_node.attributes.get("name").unwrap().clone();
        components_by_name.insert(name, component_node.clone());
    }
    components_by_name
}

fn parse_messages(doc: &Element, fields_by_name: &BTreeMap<String, DDField>, components_by_name: &BTreeMap<String, Element>) -> BTreeMap<String, DDMap> {
    let mut messages: BTreeMap<String, DDMap> = BTreeMap::new();
    let message_nodes = doc
        .children.iter()
        .filter_map(|c| c.as_element())
        .filter(|c| c.name == "messages")
        .flat_map(|node| node.children.iter())
        .filter_map(|c| c.as_element())
        .filter(|node| node.name == "message");

    for message_node in message_nodes {
        let mut dd_map = DDMap::default();
        parse_msg_element(&message_node, &mut dd_map, fields_by_name, components_by_name);
        let msg_type = message_node.attributes.get("msgtype").unwrap().clone();
        messages.insert(msg_type, dd_map);
    }
    messages
}

fn parse_header(doc: &Element, fields_by_name: &BTreeMap<String, DDField>, components_by_name: &BTreeMap<String, Element>) -> DDMap {
    let mut dd_map = DDMap::default();
    if let Some(header_node) = doc.get_child("header") {
        parse_msg_element(&header_node, &mut dd_map, fields_by_name, components_by_name);
    }
    dd_map
}

fn parse_trailer(doc: &Element, fields_by_name: &BTreeMap<String, DDField>, components_by_name: &BTreeMap<String, Element>) -> DDMap {
    let mut dd_map = DDMap::default();
    if let Some(trailer_node) = doc.get_child("trailer") {
        parse_msg_element(&trailer_node, &mut dd_map, fields_by_name, components_by_name);
    }
    dd_map
}

fn verify_child_node(child_node: &Element, parent_node: &Element) {
    if child_node.attributes.is_empty() {
        panic!(
            "Malformed data dictionary: Found text-only node containing '{}'",
            child_node.get_text().unwrap_or_default().trim()
        );
    }
    if !child_node.attributes.contains_key("name") {
        let message_type_name = parent_node
            .attributes
            .get("name")
            .map(|s| s.clone())
            .unwrap_or_else(|| parent_node.name.clone());
        panic!(
            "Malformed data dictionary: Found '{}' node without 'name' within parent '{}/{}'",
            child_node.name, parent_node.name, message_type_name
        );
    }
}

fn parse_msg_element(
    node: &Element,
    dd_map: &mut DDMap,
    fields_by_name: &BTreeMap<String, DDField>,
    components_by_name: &BTreeMap<String, Element>,
) {
    parse_msg_element_inner(node, &mut GoM::Map(dd_map), fields_by_name, components_by_name, None);
}

fn parse_msg_element_inner(
    node: &Element,
    dd_map: &mut GoM<'_>,
    fields_by_name: &BTreeMap<String, DDField>,
    components_by_name: &BTreeMap<String, Element>,
    component_required: Option<bool>,
) {
    let message_type_name = node
        .attributes
        .get("name")
        .map(|s| s.clone())
        .unwrap_or_else(|| node.name.clone());

    if node.children.is_empty() {
        return;
    }

    for child_node in node.children.iter() {
        let child_node = child_node.as_element().unwrap();
        verify_child_node(child_node, node);

        let name_attribute = child_node.attributes.get("name").unwrap().clone();

        match child_node.name.as_str() {
            "field" | "group" => {
                if !fields_by_name.contains_key(&name_attribute) {
                    panic!(
                        "Field '{}' is not defined in <fields> section.",
                        name_attribute
                    );
                }
                let dd_field = fields_by_name.get(&name_attribute).unwrap().clone();
                //bool required = (childNode.Attributes["required"]?.Value == "Y") && componentRequired.GetValueOrDefault(true);
                let required = child_node.attributes.get("required").map(|v| v == "Y").unwrap_or(false)
                    && component_required.unwrap_or(true);

                if required {
                    dd_map.required_fields.insert(dd_field.tag);
                }

                if !dd_map.is_field(dd_field.tag) {
                    dd_map.fields.insert(dd_field.tag, dd_field.clone());
                }

                //TODO check if ddmap is a ddgroup and set delim!
                if let GoM::Group(grp) = dd_map {
                    if grp.delim == 0 {
                        grp.delim = dd_field.tag;
                    }
                }

                if child_node.name == "group" {
                    let mut dd_grp = DDGroup::default();
                    dd_grp.num_fld = dd_field.tag;

                    if required {
                        dd_grp.required = true;
                    }

                    {
                        let mut dd_map = GoM::Group(&mut dd_grp);
                        parse_msg_element_inner(child_node, &mut dd_map, fields_by_name, components_by_name, None);
                    }

                    dd_map.groups.insert(dd_field.tag, dd_grp);
                }
            }
            "component" => {
                let component_node = components_by_name
                    .get(&name_attribute)
                    .unwrap()
                    .clone();

                let required = child_node.attributes.get("required").map(|v| v == "Y").unwrap_or(false);
                parse_msg_element_inner(&component_node, dd_map, fields_by_name, components_by_name, Some(required));
            }
            _ => panic!(
                "Malformed data dictionary: child node type should be one of {{field,group,component}} but is '{}' within parent '{}/{}'",
                child_node.name,
                node.name,
                message_type_name
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DataDictionary;

    #[test]
    pub fn fix40() {
        //let result = DataDictionary::load("../../../spec/FIX43.xml");
        let result = DataDictionary::load_from_string(include_str!("../../../spec/FIX40.xml"));
        println!("{:?}", result);
        assert!(result.is_ok());
    }

    #[test]
    pub fn fix41() {
        //let result = DataDictionary::load("../../../spec/FIX43.xml");
        let result = DataDictionary::load_from_string(include_str!("../../../spec/FIX41.xml"));
        println!("{:?}", result);
        assert!(result.is_ok());
    }

    #[test]
    pub fn fix42() {
        //let result = DataDictionary::load("../../../spec/FIX43.xml");
        let result = DataDictionary::load_from_string(include_str!("../../../spec/FIX42.xml"));
        println!("{:?}", result);
        assert!(result.is_ok());
    }

    #[test]
    pub fn fix43() {
        //let result = DataDictionary::load("../../../spec/FIX43.xml");
        let result = DataDictionary::load_from_string(include_str!("../../../spec/FIX43.xml"));
        println!("{:?}", result);
        assert!(result.is_ok());
        let dd = result.unwrap();
        let newordersingle = dd.messages().get("D");
        assert!(newordersingle.is_some());
        let handlinst = dd.fields_by_name.get("HandlInst");
        assert!(handlinst.is_some());
        let handlinst_in_message = newordersingle.unwrap().fields.contains_key(&handlinst.unwrap().tag);
        println!("{:?}", handlinst_in_message);
        assert!(handlinst_in_message)
    }

    #[test]
    pub fn fix44() {
        let result = DataDictionary::load_from_string(include_str!("../../../spec/FIX44.xml"));
        println!("{:?}", result);
        assert!(result.is_ok());
    }

    #[test]
    pub fn fix50() {
        //let result = DataDictionary::load("../../../spec/FIX43.xml");
        let result = DataDictionary::load_from_string(include_str!("../../../spec/FIX50.xml"));
        println!("{:?}", result);
        assert!(result.is_ok());
    }

    #[test]
    pub fn fix50sp1() {
        //let result = DataDictionary::load("../../../spec/FIX43.xml");
        let result = DataDictionary::load_from_string(include_str!("../../../spec/FIX50SP1.xml"));
        println!("{:?}", result);
        assert!(result.is_ok());
    }

    #[test]
    pub fn fix50sp2() {
        //let result = DataDictionary::load("../../../spec/FIX43.xml");
        let result = DataDictionary::load_from_string(include_str!("../../../spec/FIX50SP2.xml"));
        println!("{:?}", result);
        assert!(result.is_ok());
    }

    #[test]
    pub fn fixt11() {
        //let result = DataDictionary::load("../../../spec/FIX43.xml");
        let result = DataDictionary::load_from_string(include_str!("../../../spec/FIXT11.xml"));
        println!("{:?}", result);
        assert!(result.is_ok());
    }
}
