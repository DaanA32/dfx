use crate::field_map::FieldBase;
use crate::field_map::FieldMap;
use crate::field_map::FieldMapError;
use crate::field_map::Group;
use crate::field_map::Tag;
use crate::fields;
use crate::message::Message;
use serde::Deserializer;
use serde::Serializer;
use serde::{Deserialize, Serialize};
use std::any::TypeId;
use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::Deref;
use std::ops::DerefMut;

#[derive(Clone, Debug)]
pub enum MessageValidationError {
    UnsupportedVersion(String),
    RepeatedTag(Tag),
    NoTagValue(Tag),
    InvalidMessageType(String),
    MissingGroupDefinition(),
    RequiredTagMissing(Tag),
    InvalidTagNumber(Tag),
    IncorrectTagValue(Tag),
    TagNotDefinedForMessage(Tag, String),
    RepeatingGroupCountMismatch(Tag),
    InvalidStructure(Tag),
    FieldMapError(FieldMapError),
    DictionaryParseException(String),
}

impl From<FieldMapError> for MessageValidationError {
    fn from(e: FieldMapError) -> Self {
        MessageValidationError::FieldMapError(e)
    }
}

#[derive(Clone, Debug)]
pub enum SpecError {}

#[derive(Clone, Debug, Default)]
pub struct DataDictionary {
    check_fields_have_values: bool,
    check_fields_out_of_order: bool,
    check_user_defined_fields: bool,
    allow_unknown_message_fields: bool,
    version: Option<String>,
    fields_by_tag: HashMap<Tag, DDField>,
    fields_by_name: HashMap<String, DDField>,
    messages: HashMap<String, DDMap>,
    // spec: FixSpec,
    header: DDMap,
    trailer: DDMap,
}

impl DataDictionary {
    pub fn new(
        check_fields_have_values: bool,
        check_fields_out_of_order: bool,
        check_user_defined_fields: bool,
        allow_unknown_message_fields: bool,
        spec: FixSpec,
    ) -> Result<DataDictionary, SpecError> {
        let dd_fields = spec.fields.values.values().map(|f| {
            let tag: Tag = f.number.parse().unwrap();
            let name = f.name.clone();
            let field_type = f.type_name.clone();
            let enum_dictionary = f
                .values
                .iter()
                .map(|v| (v.enum_values.clone(), v.description.clone()))
                .collect();
            DDField::new(tag, name, enum_dictionary, field_type)
        });
        let fields_by_tag = dd_fields.clone().map(|d| (d.tag, d)).collect();
        let fields_by_name = dd_fields.clone().map(|d| (d.name.clone(), d)).collect();

        let components_by_name = spec
            .components
            .values
            .iter()
            .map(|(k, v)| (k.clone(), v.values.clone()))
            .collect();

        let version = Some(format!("{}{}.{}", spec.type_name, spec.major, spec.minor));

        let messages = spec
            .messages
            .values
            .values()
            .map(|m| {
                let key = m.msgtype.clone();
                let ddmap = parse_msg_el(
                    DDMap::default(),
                    &m.fields,
                    &fields_by_name,
                    &components_by_name,
                    None,
                )
                .unwrap();
                (key, ddmap)
            })
            .collect();
        let header = parse_msg_el(
            DDMap::default(),
            &spec.header.values,
            &fields_by_name,
            &components_by_name,
            None,
        )
        .unwrap();
        let trailer = parse_msg_el(
            DDMap::default(),
            &spec.trailer.values,
            &fields_by_name,
            &components_by_name,
            None,
        )
        .unwrap();

        Ok(Self {
            check_fields_have_values,
            check_fields_out_of_order,
            check_user_defined_fields,
            allow_unknown_message_fields,
            version,
            fields_by_tag,
            fields_by_name,
            messages,
            header,
            trailer,
        })
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
            if matches!(dictionary.version(), Some(version) if version == begin_string) {
                return Err(MessageValidationError::UnsupportedVersion(
                    begin_string.into(),
                ));
            }
        }

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
            message.has_valid_structure()?;
        }

        // if ((null != appDataDict) && (null != appDataDict.Version))
        // {
        //     appDataDict.CheckMsgType(msgType);
        //     appDataDict.CheckHasRequired(message, msgType);
        // }
        if app_data_dictionary.version().is_some() {
            app_data_dictionary.check_msg_type(msg_type)?;
            app_data_dictionary.check_has_required(message, msg_type)?;
        }

        // if (!bodyOnly)
        // {
        //     sessionDataDict.Iterate(message.Header, msgType);
        //     sessionDataDict.Iterate(message.Trailer, msgType);
        // }
        if let Some(dictionary) = session_data_dictionary {
            dictionary.iterate(message.header(), msg_type)?;
            dictionary.iterate(message.trailer(), msg_type)?;
        }

        // appDataDict.Iterate(message, msgType);
        app_data_dictionary.iterate(message, msg_type)?;
        Ok(())
    }

    fn check_msg_type(&self, msg_type: &str) -> Result<(), MessageValidationError> {
        if self.messages.contains_key(msg_type) {
            Ok(())
        } else {
            Err(MessageValidationError::InvalidMessageType(msg_type.into()))
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
                return Err(MessageValidationError::RequiredTagMissing(*field));
            }
        }

        // foreach (int field in Trailer.ReqFields)
        // {
        //     if (!message.Trailer.IsSetField(field))
        //         throw new RequiredTagMissing(field);
        // }
        for field in self.trailer.required_fields() {
            if !message.trailer().is_field_set(*field) {
                return Err(MessageValidationError::RequiredTagMissing(*field));
            }
        }

        // foreach (int field in Messages[msgType].ReqFields)
        // {
        //     if (!message.IsSetField(field))
        //         throw new RequiredTagMissing(field);
        // }
        for field in self.messages[msg_type].required_fields() {
            if !message.is_field_set(*field) {
                return Err(MessageValidationError::RequiredTagMissing(*field));
            }
        }
        Ok(())
    }
    fn check_has_no_repeated_tags(map: &FieldMap) -> Result<(), MessageValidationError> {
        if !map.repeated_tags().is_empty() {
            Err(MessageValidationError::RepeatedTag(
                map.repeated_tags().get(0).unwrap().tag(),
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
            Err(MessageValidationError::NoTagValue(field.tag()))
        } else {
            Ok(())
        }
    }
    fn check_valid_format(&self, _field: &FieldBase) -> Result<(), MessageValidationError> {
        // TODO check format based on type received.
        println!("DataDictionary.check_valid_format(): TODO check format based on type received.");
        Ok(())
    }
    fn check_valid_tag_number(&self, tag: Tag) -> Result<(), MessageValidationError> {
        // if (AllowUnknownMessageFields)
        //     return;
        // if (!FieldsByTag.ContainsKey(tag))
        // {
        //     throw new InvalidTagNumber(tag);
        // }
        if !self.allow_unknown_message_fields && !self.fields_by_tag.contains_key(&tag) {
            return Err(MessageValidationError::InvalidTagNumber(tag));
        }
        Ok(())
    }
    fn check_value(&self, field: &FieldBase) -> Result<(), MessageValidationError> {
        match self.fields_by_tag.get(&field.tag()) {
            Some(fld) => {
                if fld.has_enums() {
                    if fld.is_multiple_value_field_with_enums() {
                        let string_value = field.string_value();
                        let splitted = string_value.split(' ');
                        for value in splitted {
                            if !fld.enums().contains_key(value) {
                                return Err(MessageValidationError::IncorrectTagValue(field.tag()));
                            }
                        }
                        Ok(())
                    } else if !fld.enums().contains_key(&field.string_value()) {
                        // println!("{:?}", field);
                        // println!("{:?}", fld.enums());
                        Err(MessageValidationError::IncorrectTagValue(field.tag()))
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
        if !self.allow_unknown_message_fields {
            return Ok(());
        }

        if matches!(self.messages.get(msg_type), Some(dd) if dd.fields.contains_key(&field.tag())) {
            return Ok(());
        }
        Err(MessageValidationError::TagNotDefinedForMessage(
            field.tag(),
            msg_type.into(),
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
            Err(MessageValidationError::TagNotDefinedForMessage(
                field.tag(),
                msg_type.into(),
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
            && map.get_int(field.tag())? as usize != map.group_count(field.tag())?
        {
            return Err(MessageValidationError::RepeatingGroupCountMismatch(
                field.tag(),
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
        if !self.check_user_defined_fields && field.tag() >= fields::limits::USER_MIN {
            return false;
        }
        true
    }

    fn iterate(&self, message: &FieldMap, msg_type: &str) -> Result<(), MessageValidationError> {
        // DataDictionary.CheckHasNoRepeatedTags(map);
        DataDictionary::check_has_no_repeated_tags(message)?;

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
                return Err(MessageValidationError::RepeatedTag(field.tag()));
            }
            // CheckHasValue(field);
            self.check_has_value(field)?;

            // if (!string.IsNullOrEmpty(this.Version))
            if matches!(&self.version, Some(version) if !version.is_empty()) {
                // CheckValidFormat(field);
                self.check_valid_format(field)?;

                // if (ShouldCheckTag(field))
                if self.should_check_tag(field) {
                    // CheckValidTagNumber(field.Tag);
                    self.check_valid_tag_number(field.tag())?;

                    // CheckValue(field);
                    self.check_value(field)?;
                    // if (!Message.IsHeaderField(field.Tag, this) && !Message.IsTrailerField(field.Tag, this))
                    if !Message::is_header_field(field.tag(), Some(self))
                        && !Message::is_trailer_field(field.tag(), Some(self))
                    {
                        // CheckIsInMessage(field, msgType);
                        self.check_is_in_message(field, msg_type)?;
                        // CheckGroupDefinitionCount(field, map, msgType);
                        self.check_group_count(field, message, msg_type)?;
                    }
                }
            }

            // lastField = field.Tag;
            last_field = field.tag();
        }

        // check contents of each group
        // foreach (int groupTag in map.GetGroupDefinitionTags())
        for tag in message.group_tags() {
            // for (int i = 1; i <= map.GroupDefinitionCount(groupTag); i++)
            for i in 1..=message.group_count(*tag)? {
                // GroupDefinition g = map.GetGroupDefinition(i, groupTag);
                // DDGrp ddg = this.Messages[msgType].GetGroupDefinition(groupTag);
                // IterateGroupDefinition(g, ddg, msgType);

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
        if group_definition.is_none() {
            return Err(MessageValidationError::MissingGroupDefinition());
        }
        let group_definition = group_definition.unwrap();
        // DataDictionary.CheckHasNoRepeatedTags(group);
        DataDictionary::check_has_no_repeated_tags(group)?;

        // int lastField = 0;
        let mut last_field = 0;
        // foreach (KeyValuePair<int, Fields.IField> kvp in group)
        for (_, v) in group.entries() {
            let field = v;

            // if (lastField != 0 && field.Tag == lastField)
            //     throw new RepeatedTag(lastField);
            if last_field != 0 && field.tag() == last_field {
                return Err(MessageValidationError::RepeatedTag(field.tag()));
            }
            // CheckHasValue(field);
            self.check_has_value(field)?;

            // if (!string.IsNullOrEmpty(this.Version))
            if matches!(&self.version, Some(version) if version.is_empty()) {
                // CheckValidFormat(field);
                self.check_valid_format(field)?;

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

        // check contents of each nested group
        // foreach (int groupTag in map.GetGroupTags())
        for tag in group.group_tags() {
            // for (int i = 1; i <= map.GroupCount(groupTag); i++)
            for i in 1..=group.group_count(*tag)? {
                // Group g = group.GetGroup(i, groupTag);
                // DDGrp ddg = ddgroup.GetGroup(groupTag);
                // IterateGroup(g, ddg, msgType);

                let g = group.get_group(i as u32, *tag)?;
                let ddg = group_definition.get_group(*tag);
                self.iterate_group(g, ddg, msg_type)?;
            }
        }

        Ok(())
    }

    pub fn get_map_for_message(&self, msg_type: &str) -> Option<&DDMap> {
        self.messages.get(msg_type)
    }

    pub fn get_field_by_name(&self, field_name: &str) -> Option<&DDField> {
        self.fields_by_name.get(field_name)
    }

    pub(crate) fn is_length_field(&self, tag: u32) -> bool {
        match self.fields_by_tag.get(&tag) {
            Some(field) => field.field_type() == "LENGTH" && field.name() != "BodyLength",
            None => false,
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct DDMap {
    fields: HashMap<Tag, DDField>,
    groups: HashMap<Tag, DDGroup>,
    required_fields: HashSet<Tag>,
}
impl DDMap {
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
    pub fn required_fields(&self) -> &HashSet<Tag> {
        &self.required_fields
    }
    pub fn required_fields_mut(&mut self) -> &mut HashSet<Tag> {
        &mut self.required_fields
    }
    pub fn add_required_field(&mut self, tag: Tag) {
        self.required_fields.insert(tag);
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

fn parse_msg_el<D: AsDDMap + 'static>(
    ddmap: D,
    parts: &HashMap<String, MessagePart>,
    fields_by_name: &HashMap<String, DDField>,
    components_by_name: &HashMap<String, HashMap<String, MessagePart>>,
    component_required: Option<bool>,
) -> Result<D, MessageValidationError> {
    let mut org = ddmap;
    let ddmap = &mut org.as_map_mut();
    for v in parts.values() {
        match v {
            MessagePart::Field(field) => {
                if !fields_by_name.contains_key(&field.name) {
                    return Err(MessageValidationError::DictionaryParseException(format!(
                        "Field '{}' is not defined in <fields> section.",
                        field.name
                    )));
                }
                let dd_field = &fields_by_name[&field.name];
                let required = field.required == "Y" && component_required.unwrap_or(true);
                if required {
                    ddmap.add_required_field(dd_field.tag());
                }

                if !ddmap.is_field(dd_field.tag()) {
                    ddmap.add_field(dd_field.clone());
                }

                // if this is in a group whose delim is unset, then this must be the delim (i.e. first field)
                // if (ddmap is DDGrp ddGroup && ddGroup.Delim == 0)
                if TypeId::of::<D>() == TypeId::of::<DDGroup>() {
                    unsafe {
                        let casted: &mut DDGroup = std::mem::transmute_copy(ddmap);
                        if casted.delim() == 0 {
                            casted.delim = dd_field.tag();
                            //     ddGroup.Delim = fld.Tag;
                        }
                    }
                }
            }
            MessagePart::GroupDefinition(group) => {
                if !fields_by_name.contains_key(&group.name) {
                    return Err(MessageValidationError::DictionaryParseException(format!(
                        "Field '{}' is not defined in <fields> section.",
                        group.name
                    )));
                }
                let dd_field = &fields_by_name[&group.name];
                let required = group.required == "Y" && component_required.unwrap_or(true);
                if required {
                    ddmap.add_required_field(dd_field.tag());
                }

                if !ddmap.is_field(dd_field.tag()) {
                    ddmap.add_field(dd_field.clone());
                }

                // if this is in a group whose delim is unset, then this must be the delim (i.e. first field)
                // if (ddmap is DDGrp ddGroup && ddGroup.Delim == 0)
                if TypeId::of::<D>() == TypeId::of::<DDGroup>() {
                    unsafe {
                        let casted: &mut DDGroup = std::mem::transmute_copy(ddmap);
                        if casted.delim() == 0 {
                            casted.delim = dd_field.tag();
                        }
                    }
                }
                // ddgrp grp = new ddgrp();
                let mut grp = DDGroup {
                    num_fld: dd_field.tag(),
                    ..Default::default()
                };
                // if (required)
                if required {
                    //     grp.required = true;
                    grp.required = true;
                }

                // parsemsgel(childnode, grp);
                let grp =
                    parse_msg_el(grp, &group.fields, fields_by_name, components_by_name, None)?;
                // ddmap.groups.add(fld.tag, grp);
                ddmap.groups.insert(dd_field.tag(), grp);
            }
            MessagePart::Component(component) => {
                let component_fields = components_by_name.get(&component.name).unwrap();
                let component = parse_msg_el(
                    org,
                    component_fields,
                    fields_by_name,
                    components_by_name,
                    Some(component.required == "Y"),
                )?;
                return Ok(component);
            }
        }
    }
    Ok(org)
}

#[derive(Debug, Clone)]
pub struct DDField {
    // public int Tag;
    tag: Tag,
    // public String Name;
    name: String,
    // public Dictionary<String, String> EnumDict;
    enum_dictionary: HashMap<String, String>,
    // public String FixFldType;
    field_type: String,
    // TODO type?
    // public Type FieldType;
    is_multiple_value_field_with_enums: bool,
}
impl DDField {
    pub fn new(
        // public int Tag;
        tag: Tag,
        // public String Name;
        name: String,
        // public Dictionary<String, String> EnumDict;
        enum_dictionary: HashMap<String, String>,
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
    pub fn enums(&self) -> &HashMap<String, String> {
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
    num_fld: u32,
    delim: u32,
    required: bool,
    map: DDMap,
}
impl DDGroup {
    pub fn num_fld(&self) -> u32 {
        self.num_fld
    }
    pub fn delim(&self) -> u32 {
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

// ------------------------------
//            FIX SPEC

#[derive(Serialize, Deserialize, Debug)]
pub struct FixSpec {
    #[serde(rename = "type")]
    type_name: String,
    major: String,
    minor: String,
    servicepack: String,
    header: Header,
    messages: Messages,
    trailer: Trailer,
    components: Components,
    fields: Fields,
}

// #[derive(Serialize, Deserialize, Debug)]
// pub enum Section {
//     #[serde(rename = "header")]
//     Header(Header),
//     #[serde(rename = "messages")]
//     Messages(Messages),
//     #[serde(rename = "trailer")]
//     Trailer(Trailer),
//     #[serde(rename = "components")]
//     Components(Components),
//     #[serde(rename = "fields")]
//     Fields(Fields),
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Body {
//     #[serde(rename = "header")]
//     header: Header,
//     #[serde(rename = "messages")]
//     messages: Messages,
//     #[serde(rename = "trailer")]
//     trailer: Trailer,
//     #[serde(rename = "components")]
//     components: Components,
//     #[serde(rename = "fields")]
//     fields: Fields,
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct Header {
    // #[serde(default, rename = "$value")]
    // values: Vec<MessagePart>,
    #[serde(
        default,
        rename = "$value",
        serialize_with = "ser_peer_public",
        deserialize_with = "de_peer_public"
    )]
    values: HashMap<String, MessagePart>,
}
impl Deref for Header {
    type Target = HashMap<String, MessagePart>;
    fn deref(&self) -> &Self::Target {
        &self.values
    }
}
impl DerefMut for Header {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.values
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Messages {
    // #[serde(default, rename = "$value")]
    // values: Vec<Message>,
    #[serde(
        default,
        rename = "$value",
        serialize_with = "ser_peer_public",
        deserialize_with = "de_peer_public"
    )]
    values: HashMap<String, MessageDefinition>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Fields {
    #[serde(
        default,
        rename = "$value",
        serialize_with = "ser_peer_public",
        deserialize_with = "de_peer_public"
    )]
    values: HashMap<String, FieldDefinition>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Components {
    // #[serde(default, rename = "$value")]
    // values: Vec<ComponentDefinition>
    #[serde(
        default,
        rename = "$value",
        serialize_with = "ser_peer_public",
        deserialize_with = "de_peer_public"
    )]
    values: HashMap<String, ComponentDefinition>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Trailer {
    // #[serde(default, rename = "$value")]
    // values: Vec<MessagePart>,
    #[serde(
        default,
        rename = "$value",
        serialize_with = "ser_peer_public",
        deserialize_with = "de_peer_public"
    )]
    values: HashMap<String, MessagePart>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Field {
    name: String,
    required: String,
}

impl Named for Field {
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Component {
    name: String,
    required: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ComponentDefinition {
    name: String,
    #[serde(
        default,
        rename = "$value",
        serialize_with = "ser_peer_public",
        deserialize_with = "de_peer_public"
    )]
    values: HashMap<String, MessagePart>,
}

impl Named for ComponentDefinition {
    fn name(&self) -> &str {
        &self.name
    }
}

pub trait Named {
    fn name(&self) -> &str;
}

fn ser_peer_public<S, V: Named + Serialize>(
    peer_public: &HashMap<String, V>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let map = peer_public.iter().map(|(_, v)| v);
    serializer.collect_seq(map)
}

fn de_peer_public<'de, D, V: Named + Deserialize<'de>>(
    deserializer: D,
) -> Result<HashMap<String, V>, D::Error>
where
    D: Deserializer<'de>,
{
    let v = Vec::<V>::deserialize(deserializer)?;
    Ok(v.into_iter().map(|v| (v.name().into(), v)).collect())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MessagePart {
    #[serde(rename = "field")]
    Field(Field),
    #[serde(rename = "component")]
    Component(Component),
    #[serde(rename = "group")]
    GroupDefinition(GroupDefinition),
}

impl Named for MessagePart {
    fn name(&self) -> &str {
        match self {
            MessagePart::Field(f) => &f.name,
            MessagePart::Component(f) => &f.name,
            MessagePart::GroupDefinition(f) => &f.name,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageDefinition {
    name: String,
    msgtype: String,
    msgcat: Option<String>,
    #[serde(
        default,
        rename = "$value",
        serialize_with = "ser_peer_public",
        deserialize_with = "de_peer_public"
    )]
    fields: HashMap<String, MessagePart>,
}

impl Named for MessageDefinition {
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupDefinition {
    name: String,
    required: String,
    // #[serde(rename = "$value")]
    // fields: Vec<Field>,
    #[serde(
        default,
        rename = "$value",
        serialize_with = "ser_peer_public",
        deserialize_with = "de_peer_public"
    )]
    fields: HashMap<String, MessagePart>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FieldDefinition {
    number: String,
    name: String,
    #[serde(rename = "type")]
    type_name: String,
    #[serde(default, rename = "$value")]
    values: Vec<FieldValue>,
    // #[serde(default, rename = "$value", serialize_with = "ser_peer_public", deserialize_with = "de_peer_public")]
    // values: HashMap<String, FieldValue>,
}

impl Named for FieldDefinition {
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FieldValue {
    #[serde(rename = "enum")]
    enum_values: String,
    description: String,
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use crate::data_dictionary::*;
    #[test]
    fn test_field_value() {
        let data = "<value enum='U' description='UP' />";
        let fd = serde_xml_rs::from_str(data);
        println!("{:?}", fd);
        assert!(fd.is_ok());
        let fd: FieldValue = fd.unwrap();
        assert!(fd.enum_values == "U");
        assert!(fd.description == "UP");
    }

    #[test]
    fn test_field_definition() {
        let data = "<field number='554' name='Password' type='STRING' />";
        let fd = serde_xml_rs::from_str(data);
        println!("{:?}", fd);
        assert!(fd.is_ok());
        let fd: FieldDefinition = fd.unwrap();
        assert!(fd.number == "554");
        assert!(fd.name == "Password");
        assert!(fd.type_name == "STRING");
        assert!(fd.values.is_empty());
    }

    #[test]
    fn test_field_definition_with_values() {
        let data = r#"
            <field number='8013' name='CancelOrdersOnDisconnect' type='CHAR'>
                <value enum='S' description='SESSION' />
                <value enum='Y' description='PROFILE' />
            </field>"#;
        let fd = serde_xml_rs::from_str(data);
        println!("{:?}", fd);
        assert!(fd.is_ok());
        let fd: FieldDefinition = fd.unwrap();
        assert!(fd.number == "8013");
        assert!(fd.name == "CancelOrdersOnDisconnect");
        assert!(fd.type_name == "CHAR");
        assert!(fd.values.len() == 2);
    }

    #[test]
    fn test_header_field() {
        let data = "<field name='BeginString' required='Y' />";
        let fd = serde_xml_rs::from_str(data);
        println!("{:?}", fd);
        assert!(fd.is_ok());
        let fd: Field = fd.unwrap();
        assert!(fd.name == "BeginString");
        assert!(fd.required == "Y");
    }

    #[test]
    fn test_header() {
        let data = r#"<header>
                <field name='BeginString' required='Y' />
                <component name='InstrmtLegIOIGrp' required='N' />
            </header>"#;
        let fd = serde_xml_rs::from_str(data);
        println!("{:?}", fd);
        assert!(fd.is_ok());
        let fd: Header = fd.unwrap();
        assert!(fd.values.len() == 2);
    }

    #[test]
    fn test_message_group() {
        let data = r#"<message name='NewOrderBatch' msgtype='U6'>
                <field name='BatchID' required='Y' />
                <group name='NoOrders' required='Y'>
                    <field name='ClOrdID' required='Y' />
                    <field name='HandlInst' required='N' />
                    <field name='Symbol' required='Y' />
                    <field name='Side' required='Y' />
                    <field name='Price' required='N' />
                    <field name='OrderQty' required='N' />
                    <field name='OrdType' required='Y' />
                    <field name='TimeInForce' required='N' />
                    <field name='SelfTradePrevention' required='N' />
                    <field name='TransactTime' required='N' />
                </group>
            </message>"#;
        let fd = serde_xml_rs::from_str(data);
        println!("{:?}", fd);
        assert!(fd.is_ok());
        let fd: MessageDefinition = fd.unwrap();
        assert!(fd.fields.len() == 2);
    }

    #[test]
    fn test_minimal_spec() {
        let data = r#"
<fix type='FIX' major='4' minor='2' servicepack='0'>
 <header>
  <field name='BeginString' required='Y' />
 </header>
 <messages>
  <message name='Heartbeat' msgtype='0' msgcat='admin'>
   <field name='TestReqID' required='N' />
  </message>
 </messages>
 <components />
 <fields>
  <field number='1' name='Account' type='STRING' />
 </fields>
 <trailer>
  <field name='SignatureLength' required='N' />
 </trailer>
</fix>
"#;
        let fd = serde_xml_rs::from_str(data);
        println!("{:?}", fd);
        assert!(fd.is_ok());
        let _fd: FixSpec = fd.unwrap();
    }

    #[test]
    fn test_xml_fix40() {
        let reader = File::open("spec/FIX40.xml").unwrap();
        let fd = serde_xml_rs::from_reader(reader);
        println!("{:?}", fd);
        assert!(fd.is_ok());
        let _fd: FixSpec = fd.unwrap();
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_xml_fix41() {
        let reader = File::open("spec/FIX41.xml").unwrap();
        let fd = serde_xml_rs::from_reader(reader);
        println!("{:?}", fd);
        assert!(fd.is_ok());
        let _fd: FixSpec = fd.unwrap();
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_xml_fix42() {
        let reader = File::open("spec/FIX42.xml").unwrap();
        let fd = serde_xml_rs::from_reader(reader);
        println!("{:?}", fd);
        assert!(fd.is_ok());
        let _fd: FixSpec = fd.unwrap();
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_xml_fix43() {
        let reader = File::open("spec/FIX43.xml").unwrap();
        let fd = serde_xml_rs::from_reader(reader);
        println!("{:?}", fd);
        assert!(fd.is_ok());
        let _fd: FixSpec = fd.unwrap();
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_xml_fix44() {
        let reader = File::open("spec/FIX44.xml").unwrap();
        let fd = serde_xml_rs::from_reader(reader);
        println!("{:?}", fd);
        assert!(fd.is_ok());
        let _fd: FixSpec = fd.unwrap();
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_xml_fix50() {
        let reader = File::open("spec/FIX50.xml").unwrap();
        let fd = serde_xml_rs::from_reader(reader);
        println!("{:?}", fd);
        assert!(fd.is_ok());
        let _fd: FixSpec = fd.unwrap();
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_xml_fix50SP1() {
        let reader = File::open("spec/FIX50SP1.xml").unwrap();
        let fd = serde_xml_rs::from_reader(reader);
        println!("{:?}", fd);
        assert!(fd.is_ok());
        let _fd: FixSpec = fd.unwrap();
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_xml_fix50SP2() {
        let reader = File::open("spec/FIX50SP2.xml").unwrap();
        let fd = serde_xml_rs::from_reader(reader);
        println!("{:?}", fd);
        assert!(fd.is_ok());
        let _fd: FixSpec = fd.unwrap();
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_xml_fixT11() {
        let reader = File::open("spec/FIXT11.xml").unwrap();
        let fd = serde_xml_rs::from_reader(reader);
        println!("{:?}", fd);
        assert!(fd.is_ok());
        let _fd: FixSpec = fd.unwrap();
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_xml_fixT11_dd() {
        let reader = File::open("spec/FIXT11.xml").unwrap();
        let fd = serde_xml_rs::from_reader(reader);
        //println!("{:?}", fd);
        assert!(fd.is_ok());
        let fd: FixSpec = fd.unwrap();
        let _dd = DataDictionary::new(false, false, false, false, fd);
    }
}
