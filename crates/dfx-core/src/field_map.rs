use chrono::DateTime;
use chrono::Utc;

use crate::fields::converters::IntoBytes;
use crate::fields::converters::TryFrom;
use crate::fields::ConversionError;
use crate::message::Message;
use crate::tags;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

#[derive(Default, Clone, Debug)]
pub struct FieldMap {
    fields: BTreeMap<Tag, Field>,
    groups: HashMap<Tag, Vec<Group>>,
    // fields: HashMap<Tag, Field>,
    // groups: HashMap<Tag, Vec<Group>>,
    repeated_tags: Vec<Field>,
    _field_order: FieldOrder,
}

pub type Tag = i32;
pub type Total = u32;
pub type Length = u32;
pub type FieldOrder = Vec<Tag>;
pub(crate) type FieldBase = Field;
pub type FieldValue = Vec<u8>;

#[derive(Clone, Debug)]
pub enum FieldMapError {
    FieldNotFound(Tag),
    ConversionError(ConversionError),
}

impl From<ConversionError> for FieldMapError {
    fn from(err: ConversionError) -> Self {
        FieldMapError::ConversionError(err)
    }
}

#[derive(Default, Clone, Debug)]
pub struct Group {
    delim: Tag,
    field: Tag,
    map: FieldMap,
    field_order: Option<FieldOrder>,
}
impl Group {
    pub fn new(field: Tag, delim: Tag) -> Self {
        Group {
            delim,
            field,
            map: FieldMap::default(),
            field_order: None,
        }
    }
    pub fn delim(&self) -> Tag {
        self.delim
    }
    pub fn field(&self) -> Tag {
        self.field
    }
    pub fn calculate_string(&self) -> String {
        if let Some(order) = &self.field_order {
            todo!("calculate order: {:?}", order)
        } else {
            let order: Vec<Tag> = vec![self.delim];
            self.map.calculate_string(Some(order))
        }
    }
}
impl Deref for Group {
    type Target = FieldMap;
    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for Group {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}
#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct Field(Tag, FieldValue);

impl Field {
    pub fn new<'a, T: IntoBytes<FieldValue> + TryFrom<&'a FieldValue, Error = ConversionError>>(tag: Tag, value: T) -> Self {
        Field(tag, value.as_bytes())
    }
    pub fn from_bytes(tag: Tag, value: Vec<u8>) -> Self {
        Field(tag, value)
    }
    pub fn tag(&self) -> Tag {
        self.0
    }
    pub fn value(&self) -> &Vec<u8> {
        &self.1
    }
    pub(crate) fn string_value(&self) -> Result<String, ConversionError> {
        self.as_value()
    }
    pub(crate) fn to_string_field(&self) -> String {
        format!("{}={}", self.tag(), self.as_value::<&str>().ok().unwrap_or(""))
    }
    pub fn as_value<'a, T>(&'a self) -> Result<T, ConversionError>
    where
        T: TryFrom<&'a FieldValue, Error = ConversionError>,
    {
        TryFrom::try_from(&self.1)
    }

    pub fn get_total(&self) -> u32 {
        self.to_string_field()
            .as_bytes()
            .iter()
            .map(|b| *b as u32)
            .sum::<u32>()
            + 1 //incl SOH
    }
    pub fn bytes_len(&self) -> u32 {
        self.to_string_field().as_bytes().len() as u32 + 1 //incl SOH
    }

    pub(crate) fn to_usize(&self) -> Option<usize> {
        self.string_value().ok().map(|v| match v.parse::<usize>() {
            Ok(value) => Some(value),
            Err(_) => None,
        }).flatten()
    }
}

impl FieldMap {
    pub fn from_field_order(_field_order: FieldOrder) -> Self {
        let fields = Default::default();
        let groups = Default::default();
        // let fields = HashMap::default();
        // let groups = HashMap::default();
        let repeated_tags = Vec::default();
        FieldMap {
            fields,
            groups,
            repeated_tags,
            _field_order,
        }
    }

    pub fn from_field_map(src: &FieldMap) -> Self {
        src.clone()
    }

    pub fn set_field_base(&mut self, field: Field, overwrite: Option<bool>) -> bool {
        if matches!(overwrite, Some(b) if !b) && self.fields.contains_key(&field.tag()) {
            return false;
        }
        self.fields.insert(field.tag(), field);
        true
    }

    pub fn set_field_deref<F: Deref<Target = Field> + Clone>(
        &mut self,
        field: F,
        overwrite: Option<bool>,
    ) -> bool {
        if matches!(overwrite, Some(b) if b) {
            return false;
        }
        let field: &Field = &field;
        self.fields.insert(field.tag(), field.clone());
        true
    }

    pub fn set_tag_value<'a, T: IntoBytes<FieldValue>>(&mut self, tag: Tag, value: T) {
        let field_base = Field(tag, value.as_bytes());
        self.set_field_base(field_base, None);
    }

    pub fn set_field<'a, T: Into<Field>>(&mut self, field: T) {
        self.set_field_base(field.into(), None);
    }

    pub fn get_field(&self, tag: Tag) -> Option<&Field> {
        self.fields.get(&tag)
    }

    // VALUES
    pub fn get_int(&self, tag: Tag) -> Result<u32, FieldMapError> {
        match self.fields.get(&tag) {
            None => Err(FieldMapError::FieldNotFound(tag)),
            Some(value) => Ok(value.as_value()?),
        }
    }
    pub fn get_string(&self, tag: Tag) -> Result<String, FieldMapError> {
        match self.fields.get(&tag) {
            None => Err(FieldMapError::FieldNotFound(tag)),
            Some(value) => Ok(value.string_value()?),
        }
    }
    pub fn get_string_unchecked(&self, tag: Tag) -> String {
        self.fields[&tag].string_value().unwrap().into() // explicit_unchecked
    }
    pub fn get_bool(&self, tag: Tag) -> bool {
        self.fields[&tag].string_value().ok() == Some("Y".into())
    }
    pub fn get_datetime(&self, tag: Tag) -> Result<DateTime<Utc>, ConversionError> {
        self.fields[&tag].as_value()
    }
    // VALUES

    pub fn get_field_mut(&mut self, tag: Tag) -> Option<&mut Field> {
        self.fields.get_mut(&tag)
    }
    pub fn is_field_set(&self, tag: Tag) -> bool {
        self.fields.contains_key(&tag)
    }
    pub fn remove_field(&mut self, tag: Tag) {
        self.fields.remove(&tag);
    }

    // Groups
    pub fn add_group(&mut self, _tag: Tag, group: &Group, set_count: Option<bool>) {
        // if (!_groups.ContainsKey(group.Field))
        //     _groups.Add(group.Field, new List<Group>());
        // _groups[group.Field].Add(group);
        self.groups.entry(group.field()).or_insert_with(Vec::new);
        self.groups
            .get_mut(&group.field())
            .unwrap() //checked
            .push(group.clone());

        // if (autoIncCounter)
        if set_count.unwrap_or(true) {
            // increment group size

            // int groupsize = _groups[group.Field].Count;
            let groupsize = self.groups[&group.field()].len();
            // int counttag = group.Field;
            let counttag = group.field();
            // IntField count = null;
            // count = new IntField(couttag, groupsize);
            let count = Field::new(counttag, format!("{}", groupsize));

            // this.SetField(count, true);
            self.set_field_base(count, Some(true));
        }
    }
    /// index: Index in group starting at 1
    /// field: Field Tag (Tag of field which contains count of group)
    pub fn get_group(&self, index: u32, field: Tag) -> Result<&Group, FieldMapError> {
        // if (!_groups.ContainsKey(field))
        //     throw new FieldNotFoundException(field);
        if !self.groups.contains_key(&field) {
            println!("contains_key");
            return Err(FieldMapError::FieldNotFound(field));
        }
        // if (num <= 0)
        //     throw new FieldNotFoundException(field);
        if index == 0 {
            println!("index == 0");
            return Err(FieldMapError::FieldNotFound(field));
        }
        // if (_groups[field].Count < num)
        //     throw new FieldNotFoundException(field);
        if self.groups[&field].len() < index as usize {
            println!("self.groups[&field].len() < index as usize");
            println!("{} < {}", self.groups[&field].len(), index as usize);
            return Err(FieldMapError::FieldNotFound(field));
        }

        // return _groups[field][num - 1];
        Ok(&self.groups[&field][(index-1) as usize])
    }
    /// index: Index in group starting at 1
    /// field: Field Tag (Tag of field which contains count of group)
    pub fn get_group_mut(&mut self, index: u32, field: Tag) -> Result<&mut Group, FieldMapError> {
        // if (!_groups.ContainsKey(field))
        //     throw new FieldNotFoundException(field);
        if !self.groups.contains_key(&field) {
            return Err(FieldMapError::FieldNotFound(field));
        }
        // if (num <= 0)
        //     throw new FieldNotFoundException(field);
        if index == 0 {
            return Err(FieldMapError::FieldNotFound(field));
        }
        // if (_groups[field].Count < num)
        //     throw new FieldNotFoundException(field);
        if self.groups[&field].len() < index as usize {
            return Err(FieldMapError::FieldNotFound(field));
        }

        //TODO (index - 1) try into usize => field not found
        Ok(&mut self.groups.get_mut(&field).ok_or_else(|| FieldMapError::FieldNotFound(field))?[(index as usize - 1)])
    }
    /// index: Index in group starting at 1
    /// field: Field Tag (Tag of field which contains count of group)
    pub fn remove_group(&mut self, index: u32, field: Tag) -> Result<(), FieldMapError> {
        // if (!_groups.ContainsKey(field))
        //     throw new FieldNotFoundException(field);
        if !self.groups.contains_key(&field) {
            return Err(FieldMapError::FieldNotFound(field));
        }
        // if (num <= 0)
        //     throw new FieldNotFoundException(field);
        if index == 0 {
            return Err(FieldMapError::FieldNotFound(field));
        }
        // if (_groups[field].Count < num)
        //     throw new FieldNotFoundException(field);
        if self.groups[&field].len() < index as usize {
            return Err(FieldMapError::FieldNotFound(field));
        }

        // if (_groups[field].Count.Equals(1))
        if self.groups[&field].len() == 1 {
            //     _groups.Remove(field);
            self.groups.remove(&field);
        // else
        } else {
            //     _groups[field].RemoveAt(num - 1);
            self.groups
                .get_mut(&field)
                .ok_or_else(|| FieldMapError::FieldNotFound(field))?
                //TODO (index - 1) try into usize => field not found
                .remove((index as usize) - 1);
        }
        Ok(())
    }
    pub fn replace_group(
        &mut self,
        index: Tag,
        field: Tag,
        group: Group,
    ) -> Result<Group, FieldMapError> {
        // if (!_groups.ContainsKey(field))
        //     throw new FieldNotFoundException(field);
        if !self.groups.contains_key(&field) {
            println!("key error: {:?}, {index} {field} {group:?}", self);
            return Err(FieldMapError::FieldNotFound(field));
        }
        // if (num <= 0)
        //     throw new FieldNotFoundException(field);
        if index == 0 {
            println!("index: {:?}, {index} {field} {group:?}", self);
            return Err(FieldMapError::FieldNotFound(field));
        }
        // if (_groups[field].Count < num)
        //     throw new FieldNotFoundException(field);
        if self.groups[&field].len() < index as usize {
            println!("< index: {:?}, {index} {field} {group:?}", self);
            return Err(FieldMapError::FieldNotFound(field));
        }

        // return _groups[field][num - 1] = group;
        let group_ref = self
            .groups
            .get_mut(&field)
            .ok_or_else(|| FieldMapError::FieldNotFound(field))?
            //TODO (index - 1) try into usize => field not found
            .get_mut(index as usize - 1)
            .ok_or_else(|| FieldMapError::FieldNotFound(field))?;
        let group = std::mem::replace(group_ref, group);

        Ok(group)
    }

    pub fn group_tags(&self) -> impl Iterator<Item = &Tag> {
        self.groups.keys()
    }

    pub fn group_count(&self, field: Tag) -> Result<usize, FieldMapError> {
        if !self.groups.contains_key(&field) {
            println!("group_count_err");
            return Err(FieldMapError::FieldNotFound(field));
        }
        Ok(self.groups[&field].len())
    }

    pub fn is_empty(&self) -> bool {
        self.fields.len() == 0 && self.groups.len() == 0
    }

    pub fn calculate_total(&self) -> Total {
        // int total = 0;
        let mut total = 0;
        // foreach (Fields.IField field in _fields.Values)
        for field in self.fields.values() {
            //  if (field.Tag != Fields.Tags.CheckSum)
            if field.tag() != tags::CheckSum {
                //      total += field.getTotal();
                total += field.get_total();
            }
        }

        // foreach (Fields.IField field in this.RepeatedTags)
        for field in self.repeated_tags() {
            //  if (field.Tag != Fields.Tags.CheckSum)
            if field.tag() != tags::CheckSum {
                //      total += field.getTotal();
                total += field.get_total();
            }
        }

        // foreach (List<Group> groupList in _groups.Values)
        for group_list in self.groups.values() {
            //     foreach (Group group in groupList)
            for group in group_list {
                //         total += group.CalculateTotal();
                total += group.calculate_total();
            }
        }
        // return total;
        total
    }

    pub fn len(&self) -> Length {
        // int total = 0;
        let mut total = 0;
        // foreach (Fields.IField field in _fields.Values)
        for field in self.fields.values() {
            //     if (field != null
            //         && field.Tag != Tags.BeginString
            //         && field.Tag != Tags.BodyLength
            //         && field.Tag != Tags.CheckSum)
            if field.tag() != tags::CheckSum
                && field.tag() != tags::BeginString
                && field.tag() != tags::BodyLength
            {
                //      total += field.getLength();
                total += field.bytes_len();
            }
        }

        // foreach (Fields.IField field in this.RepeatedTags)
        for field in self.repeated_tags() {
            //     if (field != null
            //         && field.Tag != Tags.BeginString
            //         && field.Tag != Tags.BodyLength
            //         && field.Tag != Tags.CheckSum)
            if field.tag() != tags::CheckSum
                && field.tag() != tags::BeginString
                && field.tag() != tags::BodyLength
            {
                //      total += field.getLength();
                total += field.bytes_len();
            }
        }

        // foreach (List<Group> groupList in _groups.Values)
        for group_list in self.groups.values() {
            //     foreach (Group group in groupList)
            for group in group_list {
                //         total += group.CalculateLength();
                total += group.len();
            }
        }

        total
    }

    pub fn repeated_tags(&self) -> &Vec<Field> {
        &self.repeated_tags
    }

    pub fn repeated_tags_mut(&mut self) -> &mut Vec<Field> {
        &mut self.repeated_tags
    }

    pub fn entries<'a>(&'a self) -> impl Iterator<Item = (&'a Tag, &Field)> {
        self.fields.iter()
    }

    pub fn clear(&mut self) {
        self.fields.clear();
        self.groups.clear();
    }

    pub fn calculate_string(&self, prefields: Option<FieldOrder>) -> String {
        // HashSet<int> groupCounterTags = new HashSet<int>(_groups.Keys);
        let group_counter_tags: BTreeSet<&Tag> = self.group_tags().collect();
        let prefields = prefields.unwrap_or_default();
        let mut sb = String::new();

        // foreach (int preField in preFields)
        for prefield in &prefields {
            //     if (IsSetField(preField))
            if self.is_field_set(*prefield) {
                //         sb.Append(preField + "=" + GetString(preField)).Append(Message.SOH);
                sb.push_str(
                    format!(
                        "{}={}{}",
                        prefield,
                        self.get_string_unchecked(*prefield),
                        Message::SOH
                    )
                    .as_str(),
                );
                //         if (groupCounterTags.Contains(preField))
                if group_counter_tags.contains(prefield) {
                    //             List<Group> glist = _groups[preField];
                    let glist = &self.groups[prefield];
                    //             foreach (Group g in glist)
                    for g in glist {
                        //                 sb.Append(g.CalculateString());
                        sb.push_str(&g.calculate_string());
                    }
                }
            }
        }

        // foreach (Fields.IField field in _fields.Values)
        for field in self.fields.values() {
            //     if (groupCounterTags.Contains(field.Tag))
            if group_counter_tags.contains(&field.tag()) {
                //         continue;
                continue;
            }
            //     if (preFields.Contains(field.Tag))
            if prefields.contains(&field.tag()) {
                //         continue; //already did this one
                continue; //already did this one
            }
            //     sb.Append(field.Tag.ToString() + "=" + field.ToString());
            //     sb.Append(Message.SOH);
            sb.push_str(
                format!("{}={}{}", field.tag(), field.string_value().unwrap(), Message::SOH).as_str(),
            );
        }

        // foreach(int counterTag in _groups.Keys)
        for counter_tag in self.groups.keys() {
            //     if (preFields.Contains(counterTag))
            if prefields.contains(counter_tag) {
                //         continue; //already did this one
                continue; //already did this one
            }

            //     List<Group> groupList = _groups[counterTag];
            let grouplist = &self.groups[counter_tag];
            //     if (groupList.Count == 0)
            if grouplist.is_empty() {
                //         continue; //probably unnecessary, but it doesn't hurt to check
                continue; //probably unnecessary, but it doesn't hurt to check
            }

            //     sb.Append(_fields[counterTag].toStringField());
            //     sb.Append(Message.SOH);
            sb.push_str(
                format!(
                    "{}{}",
                    self.fields[counter_tag].to_string_field(),
                    Message::SOH
                )
                .as_str(),
            );

            //     foreach (Group group in groupList)
            for group in grouplist {
                //         sb.Append(group.CalculateString());
                sb.push_str(&group.calculate_string());
            }
        }

        sb
    }
}

#[cfg(test)]
mod tests {
    use super::FieldValue;
    use super::Tag;
    use std::any::Any;
    use std::any::TypeId;

    trait TagValue: Any {
        fn tag(&self) -> Tag;
        fn value(&self) -> &FieldValue;
        fn as_any(&self) -> &dyn Any;
    }

    #[derive(Debug)]
    struct Test {
        pub tag: Tag,
        pub value: FieldValue,
    }
    impl TagValue for Test {
        fn tag(&self) -> Tag {
            self.tag
        }
        fn value(&self) -> &FieldValue {
            &self.value
        }
        fn as_any(&self) -> &dyn Any {
            self
        }
    }
    impl<T: TagValue> From<T> for Box<dyn TagValue> {
        fn from(tag_value: T) -> Self {
            Box::new(tag_value)
        }
    }

    #[test]
    fn box_test() {
        let boxed = Test {
            tag: 0,
            value: "Hello".into(),
        }
        .into();
        let boxed_vec: Vec<Box<dyn TagValue>> = vec![boxed];
        for value in boxed_vec {
            let value: &dyn TagValue = &*value;
            if value.type_id() == TypeId::of::<Test>() {
                let result = value.as_any().downcast_ref::<Test>();
                assert!(result.is_some());
                println!("{:?}", result.unwrap());
            } else {
                panic!()
            }
        }
    }
}
