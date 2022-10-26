use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

#[derive(Default, Clone, Debug)]
pub struct FieldMap {
    fields: HashMap<Tag, Field>,
    groups: HashMap<Tag, Vec<Group>>,
    repeated_tags: Vec<Field>,
    field_order: FieldOrder,
}

pub type Tag = u32;
pub type Total = u32;
pub type Length = u32;
pub type FieldOrder = Vec<u32>;
pub type Field = FieldBase;

#[derive(Default, Clone, Debug)]
pub struct Group(FieldMap);
impl Group {
    pub fn new(tag: Tag, delim: Tag) -> Self {
        todo!();
    }
    pub fn field(&self) -> Tag {
        todo!();
    }
}
impl Deref for Group {
    type Target = FieldMap;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Group {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[derive(Default, Clone, Debug)]
pub struct FieldBase(Tag, String);

impl FieldBase {
    pub fn new(tag: Tag, value: String) -> Self {
        FieldBase(tag, value)
    }
    pub fn tag(&self) -> Tag {
        self.0
    }
    pub fn value(&self) -> &String {
        &self.1
    }
}

impl FieldMap {

    pub fn from_field_order(field_order: FieldOrder) -> Self {
        let fields = HashMap::default();
        let groups = HashMap::default();
        let repeated_tags = Vec::default();
        FieldMap {
            fields,
            groups,
            repeated_tags,
            field_order
        }
    }

    pub fn from_field_map(src: FieldMap) -> Self {
        src.clone()
    }

    pub fn set_field_base(&mut self, field: FieldBase, overwrite: Option<bool>) -> bool {
        if matches!(overwrite, Some(b) if b) {
            return false;
        }
        self.fields.insert(field.tag(), field);
        return true;
    }

    pub fn set_field(&mut self, tag: Tag, value: &str) {
        let field_base = FieldBase(tag, value.into());
        self.set_field_base(field_base, None);
    }

    pub fn get_field(&self, tag: Tag) -> &FieldBase {
        &self.fields[&tag]
    }
    pub fn get_int(&self, tag: Tag) -> u32 {
        self.fields[&tag].value().parse().unwrap()
    }
    pub fn get_field_mut(&mut self, tag: Tag) -> &mut FieldBase {
        self.fields.get_mut(&tag).unwrap()
    }
    pub fn is_field_set(&self, tag: Tag) -> bool {
        self.fields.contains_key(&tag)
    }
    pub fn remove_field(&mut self, tag: Tag) {
        self.fields.remove(&tag);
    }

    // Groups
    pub fn add_group(&mut self, tag: Tag, group: &Group, set_count: Option<bool>) {
        // if (!_groups.ContainsKey(group.Field))
        //     _groups.Add(group.Field, new List<Group>());
        // _groups[group.Field].Add(group);
        if !self.groups.contains_key(&group.field()) {
            self.groups.insert(group.field(), Vec::new());
        }
        self.groups.get_mut(&group.field()).unwrap().push(group.clone());

        // if (autoIncCounter)
        if set_count.is_none() || set_count.unwrap() {
            // increment group size

            // int groupsize = _groups[group.Field].Count;
            let groupsize = self.groups[&group.field()].len();
            // int counttag = group.Field;
            let counttag = group.field();
            // IntField count = null;
            // count = new IntField(couttag, groupsize);
            let count = FieldBase::new(counttag, format!("{}", groupsize));

            // this.SetField(count, true);
            self.set_field_base(count, Some(true));

        }
    }
    /// index: Index in group starting at 1
    /// field: Field Tag (Tag of field which contains count of group)
    pub fn get_group(&self, index: u32, field: Tag) -> Group {
        todo!("{:?} {:?}", index, field);
    }
    /// index: Index in group starting at 1
    /// field: Field Tag (Tag of field which contains count of group)
    pub fn remove_group(&mut self, index: u32, field: Tag) {
        todo!("{:?} {:?}", index, field);
    }
    pub fn replace_group(&mut self, tag: Tag, group: &FieldMap, set_count: Option<bool>) {
        todo!("{:?} {:?} {:?}", tag, group, set_count)
    }

    pub fn group_tags(&self) -> Vec<Tag> {
        todo!()
    }

    pub fn group_count(&self, tag: Tag) -> usize {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        todo!()
    }

    pub fn calculate_total(&self) -> Total {
        todo!();
    }

    pub fn len(&self) -> Length {
        todo!();
    }

    pub fn repeated_tags(&self) -> Vec<Tag> {
        todo!();
    }

    pub fn repeated_tags_mut(&self) -> &mut Vec<FieldBase> {
        todo!();
    }

    pub fn entries(&self) -> Vec<(Tag, FieldBase)> {
        todo!();
    }

    pub fn clear(&mut self) {
        self.fields.clear();
        self.groups.clear();
    }
}
