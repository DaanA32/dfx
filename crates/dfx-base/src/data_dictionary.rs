use crate::data_dictionary_provider;
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
use xmltree::ParseError;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashSet;
use std::fs::File;
use std::num::ParseIntError;
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
    IoError(std::io::Error),
    ParseError(ParseError),
    Missing { entry_type: String, name: String },
    InvalidVersionType { version_type: String },
    ParseIntError(ParseIntError),
}

impl From<serde_xml_rs::Error> for DataDictionaryError {
    fn from(error: serde_xml_rs::Error) -> Self {
        Self::DeserializeError(error)
    }
}
impl From<std::io::Error> for DataDictionaryError {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(error)
    }
}
impl From<ParseError> for DataDictionaryError {
    fn from(error: ParseError) -> Self {
        Self::ParseError(error)
    }
}
impl From<ParseIntError> for DataDictionaryError {
    fn from(error: ParseIntError) -> Self {
        Self::ParseIntError(error)
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
    header: DDMap,
    trailer: DDMap,
}


impl DataDictionary {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<DataDictionary, DataDictionaryError> {
        let path: &Path = path.as_ref();
        let mut reader = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", path.display(), why),
            Ok(file) => file,
        };

        let mut contents = String::new();
        reader.read_to_string(&mut contents)?;
        let dd = DataDictionary::load_from_string(&contents)?;
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

        if let Some(dictionary) = session_data_dictionary {
            if let Some(version) = dictionary.version() {
                if version != begin_string {
                    return Err(MessageValidationError::UnsupportedVersion {
                        expected: version.into(),
                        actual: begin_string.into(),
                    });
                }
            }
        }

        let check_order_session = session_data_dictionary
            .map(|d| d.check_fields_out_of_order())
            .unwrap_or(false);
        let check_order_app = app_data_dictionary.check_fields_out_of_order();
        if check_order_session || check_order_app {
            message.has_valid_structure()?;
        }

        if app_data_dictionary.version().is_some() {
            app_data_dictionary.check_msg_type(msg_type)?;
            app_data_dictionary.check_has_required(message, msg_type)?;
        }

        if let Some(dictionary) = session_data_dictionary {
            dictionary.iterate(message.header(), msg_type)?;
            dictionary.iterate(message.trailer(), msg_type)?;
        }

        app_data_dictionary.iterate(message, msg_type)?;
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
        for field in self.header.required_fields() {
            if !message.header().is_field_set(*field) {
                return Err(MessageValidationError::TagException(TagException::required_tag_missing(*field)));
            }
        }

        for field in self.trailer.required_fields() {
            if !message.trailer().is_field_set(*field) {
                return Err(MessageValidationError::TagException(TagException::required_tag_missing(*field)));
            }
        }

        for field in self.messages[msg_type].required_fields() {
            if !message.is_field_set(*field) {
                return Err(MessageValidationError::TagException(TagException::required_tag_missing(*field)));
            }
        }
        Ok(())
    }
    fn check_has_no_repeated_tags(map: &FieldMap) -> Result<(), MessageValidationError> {
        if let Some(field) = map.repeated_tags().get(0) {
            Err(MessageValidationError::TagException(
                TagException::repeated_tag(field.tag()),
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
        if let Some(field_definition) = self.fields_by_tag.get(&field.tag()) {
            let field_type = FieldType::get(field_definition.field_type().as_str());
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

    }
    fn check_valid_tag_number(&self, tag: Tag) -> Result<(), MessageValidationError> {
        if !self.allow_unknown_message_fields && !self.fields_by_tag.contains_key(&tag) {
            return Err(MessageValidationError::TagException(TagException::invalid_tag_number(tag)));
        }
        Ok(())
    }
    fn check_value(&self, field: &FieldBase) -> Result<(), MessageValidationError> {
        match self.fields_by_tag.get(&field.tag()) {
            Some(fld) => {
                if fld.has_enums() {
                    if fld.is_multiple_value_field_with_enums() {
                        let string_value = field.string_value()?;
                        let splitted = string_value.split(' ');
                        for value in splitted {
                            if !fld.enums().contains_key(value) {
                                return Err(MessageValidationError::TagException(
                                    TagException::incorrect_tag_value(field.tag())
                                ));
                            }
                        }
                        Ok(())
                    } else if !fld.enums().contains_key(&field.string_value()?) {
                        Err(MessageValidationError::TagException(
                            TagException::incorrect_tag_value(field.tag())
                        ))
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
        if self.allow_unknown_message_fields {
            return Ok(());
        }

        if matches!(self.messages.get(msg_type), Some(dd) if dd.fields.contains_key(&field.tag())) {
            return Ok(());
        }
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
        if dd_group.is_field(field.tag()) {
            Ok(())
        } else {
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
        if self.is_group(msg_type, field.tag())
            && map.get_int(field.tag())? as usize != map.group_count(field.tag()).unwrap_or(0)
        {
            return Err(MessageValidationError::TagException(
                TagException::repeating_group_count_mismatch(field.tag())
            ));
        }
        Ok(())
    }
    fn is_group(&self, msg_type: &str, tag: Tag) -> bool {
        if self.messages.contains_key(msg_type) {
            return self.messages[msg_type].is_group(tag);
        }
        false
    }
    fn should_check_tag(&self, field: &FieldBase) -> bool {
        if !self.check_user_defined_fields && (field.tag() >= fields::limits::USER_MIN) {
            return false;
        }
        true
    }

    fn iterate(&self, message: &FieldMap, msg_type: &str) -> Result<(), MessageValidationError> {
        DataDictionary::check_has_no_repeated_tags(message)?;

        // check non-group fields
        let mut last_field = 0;
        for (_k, v) in message.entries() {
            let field = v;
            if last_field != 0 && field.tag() == last_field {
                return Err(MessageValidationError::TagException(TagException::repeated_tag(field.tag())));
            }
            self.check_has_value(field)?;

            if !self.version.is_none() && !matches!(&self.version, Some(version) if version.is_empty()) {
                self.check_valid_format(field)?;

                if self.should_check_tag(field) {
                    self.check_valid_tag_number(field.tag())?;

                    self.check_value(field)?;
                    if !Message::is_header_field(field.tag(), Some(self))
                        && !Message::is_trailer_field(field.tag(), Some(self))
                    {
                        self.check_is_in_message(field, msg_type)?;
                        self.check_group_count(field, message, msg_type)?;
                    } else {
                    }
                }
            }

            last_field = field.tag();
        }

        // check contents of each group
        for tag in message.group_tags() {
            for i in 1..=message.group_count(*tag)? {
                let g = message.get_group(i as u32, *tag)?;
                let ddg = self.messages[msg_type].get_group(*tag);
                self.iterate_group(g, ddg, msg_type)?;
            }
        }

        Ok(())
    }

    fn iterate_group(
        &self,
        group: &Group,
        group_definition: Option<&DDGroup>,
        msg_type: &str,
    ) -> Result<(), MessageValidationError> {
        match group_definition {
            Some(group_definition) => {
                DataDictionary::check_has_no_repeated_tags(group)?;

                let mut last_field = 0;
                for (_, v) in group.entries() {
                    let field = v;

                    if last_field != 0 && field.tag() == last_field {
                        return Err(MessageValidationError::TagException(TagException::repeated_tag(last_field)));
                    }
                    self.check_has_value(field)?;

                    if !self.version.is_none() && !matches!(&self.version, Some(version) if version.is_empty()) {
                        self.check_valid_format(field)?;

                        if self.should_check_tag(field) {
                            self.check_valid_tag_number(field.tag())?;

                            self.check_value(field)?;
                            self.check_is_in_group(field, group_definition, msg_type)?;
                            self.check_group_count(field, group, msg_type)?;
                        }
                    }
                    last_field = field.tag();
                }

                // check contents of each nested group
                for tag in group.group_tags() {
                    for i in 1..=group.group_count(*tag)? {
                        let g = group.get_group(i as u32, *tag)?;
                        let ddg = group_definition.get_group(*tag);
                        self.iterate_group(g, ddg, msg_type)?;
                    }
                }

                Ok(())
            }
            None => Ok(()),
        }
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
    tag: Tag,
    name: String,
    enum_dictionary: BTreeMap<String, String>,
    field_type: String,
    is_multiple_value_field_with_enums: bool,
}
impl DDField {

    pub fn from_xml_str(xml_str: &str) -> Self {
        todo!("DataDictionary::from_xml_str({xml_str})")
    }

    pub fn new(
        tag: Tag,
        name: String,
        enum_dictionary: BTreeMap<String, String>,
        field_type: String,
        // TODO type?
        // is_multiple_value_field_with_enums: bool
    ) -> Self {
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
use std::str::FromStr;
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

    pub fn load(&mut self, path: &str) -> Result<Self, DataDictionaryError> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        Self::load_from_string(&contents)
    }

    pub fn load_from_string(contents: &str) -> Result<Self, DataDictionaryError> {
        let root_doc = Element::parse(contents.as_bytes())?;

        let (_major_version, _minor_version, version) = get_version_info(&root_doc)?;
        let (fields_by_tag, fields_by_name) = parse_fields(&root_doc)?;
        let components_by_name = cache_components(&root_doc)?;
        let messages = parse_messages(&root_doc, &fields_by_name, &components_by_name)?;
        let header = parse_header(&root_doc, &fields_by_name, &components_by_name)?;
        let trailer = parse_trailer(&root_doc, &fields_by_name, &components_by_name)?;

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

fn get_version_info(doc: &Element) -> Result<(String, String, String), DataDictionaryError> {
    let major_version = doc.attributes.get("major")
        .ok_or(DataDictionaryError::Missing { entry_type: "attribute".into(), name: "major".into() })?.to_string();
    let minor_version = doc.attributes.get("minor")
        .ok_or(DataDictionaryError::Missing { entry_type: "attribute".into(), name: "minor".into() })?.to_string();
    let version = "FIX".to_string();
    let version_type = doc.attributes.get("type").unwrap_or(&version);
    if version_type != "FIX" && version_type != "FIXT" {
        return Err(DataDictionaryError::InvalidVersionType { version_type: version_type.clone() });
    }
    let version = format!("{}.{}.{}", version_type, major_version, minor_version);
    Ok((major_version, minor_version, version))
}

fn parse_fields(doc: &Element) -> Result<(BTreeMap<i32, DDField>, BTreeMap<String, DDField>), DataDictionaryError> {
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
        let tag_str = field_node.attributes.get("number")
            .ok_or(DataDictionaryError::Missing { entry_type: "attribute".into(), name: "major".into() })?;
        let name = field_node.attributes.get("name")
            .ok_or(DataDictionaryError::Missing { entry_type: "attribute".into(), name: "name".into() })?;
        let field_type = field_node.attributes.get("type")
            .ok_or(DataDictionaryError::Missing { entry_type: "attribute".into(), name: "type".into() })?;

        let tag = tag_str.parse::<i32>()?;
        let mut enums = BTreeMap::new();
        for enum_node in field_node.children.iter()
            .filter_map(|c| c.as_element())
            .filter(|c| c.name == "value")
        {
            let enum_value = enum_node.attributes.get("enum")
                .ok_or(DataDictionaryError::Missing { entry_type: "attribute".into(), name: "enum".into() })?.clone();
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
    return Ok((fields_by_tag, fields_by_name));
}

fn cache_components(doc: &Element) -> Result<BTreeMap<String, Element>, DataDictionaryError> {
    let mut components_by_name: BTreeMap<String, Element> = BTreeMap::new();
    let component_nodes = doc
        .children.iter()
        .filter_map(|c| c.as_element())
        .filter(|c| c.name == "components")
        .flat_map(|node| node.children.iter())
        .filter_map(|c| c.as_element())
        .filter(|node| node.name == "component");

    for component_node in component_nodes {
        let name = component_node.attributes.get("name")
            .ok_or(DataDictionaryError::Missing { entry_type: "attribute".into(), name: "name".into() })?.clone();
        components_by_name.insert(name, component_node.clone());
    }
    Ok(components_by_name)
}

fn parse_messages(doc: &Element, fields_by_name: &BTreeMap<String, DDField>, components_by_name: &BTreeMap<String, Element>) -> Result<BTreeMap<String, DDMap>, DataDictionaryError> {
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
        parse_msg_element(&message_node, &mut dd_map, fields_by_name, components_by_name)?;
        let msg_type = message_node.attributes.get("msgtype")
            .ok_or(DataDictionaryError::Missing { entry_type: "attribute".into(), name: "msgtype".into() })?.clone();
        messages.insert(msg_type, dd_map);
    }
    Ok(messages)
}

fn parse_header(doc: &Element, fields_by_name: &BTreeMap<String, DDField>, components_by_name: &BTreeMap<String, Element>) -> Result<DDMap, DataDictionaryError> {
    let mut dd_map = DDMap::default();
    if let Some(header_node) = doc.get_child("header") {
        parse_msg_element(&header_node, &mut dd_map, fields_by_name, components_by_name)?;
    }
    Ok(dd_map)
}

fn parse_trailer(doc: &Element, fields_by_name: &BTreeMap<String, DDField>, components_by_name: &BTreeMap<String, Element>) -> Result<DDMap, DataDictionaryError> {
    let mut dd_map = DDMap::default();
    if let Some(trailer_node) = doc.get_child("trailer") {
        parse_msg_element(&trailer_node, &mut dd_map, fields_by_name, components_by_name)?;
    }
    Ok(dd_map)
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
) -> Result<(), DataDictionaryError> {
    parse_msg_element_inner(node, &mut GoM::Map(dd_map), fields_by_name, components_by_name, None)
}

fn parse_msg_element_inner(
    node: &Element,
    dd_map: &mut GoM<'_>,
    fields_by_name: &BTreeMap<String, DDField>,
    components_by_name: &BTreeMap<String, Element>,
    component_required: Option<bool>,
) -> Result<(), DataDictionaryError> {
    let message_type_name = node
        .attributes
        .get("name")
        .map(|s| s.clone())
        .unwrap_or_else(|| node.name.clone());

    if node.children.is_empty() {
        return Ok(());
    }

    for child_node in node.children.iter() {
        if let Some(child_node) = child_node.as_element() {
            verify_child_node(child_node, node);

            let name_attribute = child_node.attributes.get("name")
                .ok_or(DataDictionaryError::Missing { entry_type: "attribute".into(), name: "name".into() })?.clone();

            match child_node.name.as_str() {
                "field" | "group" => {
                    if !fields_by_name.contains_key(&name_attribute) {
                        panic!(
                            "Field '{}' is not defined in <fields> section.",
                            name_attribute
                        );
                    }
                    let dd_field = fields_by_name.get(&name_attribute)
                        .ok_or(DataDictionaryError::Missing { entry_type: "field".into(), name: name_attribute.clone() })?.clone();
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
                            parse_msg_element_inner(child_node, &mut dd_map, fields_by_name, components_by_name, None)?;
                        }

                        dd_map.groups.insert(dd_field.tag, dd_grp);
                    }
                }
                "component" => {
                    let component_node = components_by_name
                        .get(&name_attribute)
                        .ok_or(DataDictionaryError::Missing { entry_type: "component".into(), name: name_attribute.clone() })?
                        .clone();

                    let required = child_node.attributes.get("required").map(|v| v == "Y").unwrap_or(false);
                    parse_msg_element_inner(&component_node, dd_map, fields_by_name, components_by_name, Some(required))?;
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
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::DataDictionary;

    #[test]
    pub fn fix40() {
        let result = DataDictionary::load_from_string(include_str!("../../../spec/FIX40.xml"));
        println!("{:?}", result);
        assert!(result.is_ok());
    }

    #[test]
    pub fn fix41() {
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
        if let Ok(dd) = result {
            let newordersingle = dd.messages().get("D");
            let handlinst = dd.fields_by_name.get("HandlInst");
            assert!(newordersingle.is_some());
            assert!(handlinst.is_some());
            match (newordersingle, handlinst) {
                (Some(newordersingle), Some(handlinst)) => {
                    let handlinst_in_message = newordersingle.fields.contains_key(&handlinst.tag);
                    println!("{:?}", handlinst_in_message);
                    assert!(handlinst_in_message)
                }
                _ => (),
            }
        }
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
