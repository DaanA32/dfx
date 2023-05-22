use std::{collections::HashMap, ops::Deref, ops::DerefMut};
use std::fs::File;
use std::io::Read;
use xmltree::Element;

#[derive(Default, Debug, Clone)]
pub struct DDField {
    tag: i32,
    name: String,
    enums: HashMap<String, String>,
    field_type: String,
}

#[derive(Default, Debug, Clone)]
pub struct DDMap {
    req_fields: Vec<i32>,
    fields: HashMap<i32, DDField>,
    groups: HashMap<i32, DDGrp>,
}

impl DDMap {
    pub fn is_field(&self, tag: i32) -> bool {
        self.fields.contains_key(&tag)
    }
}

#[derive(Default, Debug, Clone)]
pub struct DDGrp {
    num_fld: i32,
    required: bool,
    delim: i32,
    map: DDMap,
}

impl Deref for DDGrp {
    type Target = DDMap;
    fn deref(&self) -> &Self::Target {
        &self.map
    }
}
impl DerefMut for DDGrp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

#[derive(Debug, Clone)]
pub struct DataDictionary {
    major_version: String,
    minor_version: String,
    version: String,
    fields_by_tag: HashMap<i32, DDField>,
    fields_by_name: HashMap<String, DDField>,
    messages: HashMap<String, DDMap>,
    components_by_name: HashMap<String, Element>,
    check_fields_out_of_order: bool,
    check_fields_have_values: bool,
    check_user_defined_fields: bool,
    allow_unknown_message_fields: bool,
    header: DDMap,
    trailer: DDMap,
}

impl DataDictionary {
    pub fn new() -> DataDictionary {
        DataDictionary {
            major_version: String::new(),
            minor_version: String::new(),
            version: String::new(),
            fields_by_tag: HashMap::new(),
            fields_by_name: HashMap::new(),
            messages: HashMap::new(),
            components_by_name: HashMap::new(),
            check_fields_out_of_order: true,
            check_fields_have_values: true,
            check_user_defined_fields: true,
            allow_unknown_message_fields: false,
            header: DDMap::default(),
            trailer: DDMap::default(),
        }
    }

    pub fn load(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        self.load_from_string(&contents)
    }

    pub fn load_from_string(&mut self, contents: &str) -> Result<(), Box<dyn std::error::Error>> {
        let root_doc = Element::parse(contents.as_bytes())?;

        let (major_version, minor_version, version) = get_version_info(&root_doc);
        let (fields_by_tag, fields_by_name) = parse_fields(&root_doc);
        let components_by_name = cache_components(&root_doc);
        let messages = parse_messages(&root_doc, &fields_by_name, &components_by_name);
        let header = parse_header(&root_doc, &fields_by_name, &components_by_name);
        let trailer = parse_trailer(&root_doc, &fields_by_name, &components_by_name);

        self.major_version = major_version;
        self.minor_version = minor_version;
        self.version = version;
        self.fields_by_tag = fields_by_tag;
        self.fields_by_name = fields_by_name;
        self.components_by_name = components_by_name;
        self.messages = messages;
        self.header = header;
        self.trailer = trailer;

        Ok(())
    }

}

impl DataDictionary {
    pub fn messages(&self) -> &HashMap<String, DDMap> {
        &self.messages
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

fn parse_fields(doc: &Element) -> (HashMap<i32, DDField>, HashMap<String, DDField>) {
    let mut fields_by_tag: HashMap<i32, DDField> = HashMap::new();
    let mut fields_by_name: HashMap<String, DDField> = HashMap::new();
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
        let mut enums = HashMap::new();
        if let Some(enum_nodes) = field_node.get_child("value") {
            for enum_node in enum_nodes.children.iter() {
                let enum_value = enum_node.as_element().unwrap().attributes.get("enum").unwrap().clone();
                let description = enum_node.as_element().unwrap().attributes.get("description").map(|s| s.clone()).unwrap_or_default();
                enums.insert(enum_value, description);
            }
        }

        let dd_field = DDField {
            tag,
            name: name.clone(),
            enums,
            field_type: field_type.clone(),
        };

        fields_by_tag.insert(tag, dd_field.clone());
        fields_by_name.insert(name.clone(), dd_field);
    }
    return (fields_by_tag, fields_by_name);
}

fn cache_components(doc: &Element) -> HashMap<String, Element> {
    let mut components_by_name: HashMap<String, Element> = HashMap::new();
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

fn parse_messages(doc: &Element, fields_by_name: &HashMap<String, DDField>, components_by_name: &HashMap<String, Element>) -> HashMap<String, DDMap> {
    let mut messages: HashMap<String, DDMap> = HashMap::new();
    let message_nodes = doc
        .children.iter()
        .filter_map(|c| c.as_element())
        .filter(|c| c.name == "messages")
        .flat_map(|node| node.children.iter())
        .filter_map(|c| c.as_element())
        .filter(|node| node.name == "message");

    for message_node in message_nodes {
        let mut dd_map = DDMap::default();
        parse_msg_el(&message_node, &mut dd_map, fields_by_name, components_by_name);
        let msg_type = message_node.attributes.get("msgtype").unwrap().clone();
        messages.insert(msg_type, dd_map);
    }
    messages
}

fn parse_header(doc: &Element, fields_by_name: &HashMap<String, DDField>, components_by_name: &HashMap<String, Element>) -> DDMap {
    let mut dd_map = DDMap::default();
    if let Some(header_node) = doc.get_child("header") {
        parse_msg_el(&header_node, &mut dd_map, fields_by_name, components_by_name);
    }
    dd_map
}

fn parse_trailer(doc: &Element, fields_by_name: &HashMap<String, DDField>, components_by_name: &HashMap<String, Element>) -> DDMap {
    let mut dd_map = DDMap::default();
    if let Some(trailer_node) = doc.get_child("trailer") {
        parse_msg_el(&trailer_node, &mut dd_map, fields_by_name, components_by_name);
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

fn parse_msg_el(
    node: &Element,
    dd_map: &mut DDMap,
    fields_by_name: &HashMap<String, DDField>,
    components_by_name: &HashMap<String, Element>,
) {
    parse_msg_el_inner(node, dd_map, fields_by_name, components_by_name, None);
}

fn parse_msg_el_inner(
    node: &Element,
    dd_map: &mut DDMap,
    fields_by_name: &HashMap<String, DDField>,
    components_by_name: &HashMap<String, Element>,
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

                let required = child_node.attributes.get("required").map_or_else(
                    || component_required.unwrap_or(true),
                    |value| value == "Y",
                );

                if required {
                    dd_map.req_fields.push(dd_field.tag);
                }

                if !dd_map.is_field(dd_field.tag) {
                    dd_map.fields.insert(dd_field.tag, dd_field.clone());
                }

                if child_node.name == "group" {
                    let mut dd_grp = DDGrp::default();
                    dd_grp.num_fld = dd_field.tag;

                    if required {
                        dd_grp.required = true;
                    }

                    parse_msg_el_inner(child_node, &mut dd_grp, fields_by_name, components_by_name, None);

                    dd_map.groups.insert(dd_field.tag, dd_grp);
                }
            }
            "component" => {
                let component_node = components_by_name
                    .get(&name_attribute)
                    .unwrap()
                    .clone();

                let required = child_node.attributes.get("required").map_or_else(
                    || component_required.unwrap_or(false),
                    |value| value == "Y",
                );

                parse_msg_el_inner(&component_node, dd_map, fields_by_name, components_by_name, Some(required));
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
        let mut dd = DataDictionary::new();
        //let result = dd.load("../../../spec/FIX43.xml");
        let result = dd.load_from_string(include_str!("../../../spec/FIX40.xml"));
        println!("{:?}", result);
        assert!(result.is_ok());
    }

    #[test]
    pub fn fix41() {
        let mut dd = DataDictionary::new();
        //let result = dd.load("../../../spec/FIX43.xml");
        let result = dd.load_from_string(include_str!("../../../spec/FIX41.xml"));
        println!("{:?}", result);
        assert!(result.is_ok());
    }

    #[test]
    pub fn fix42() {
        let mut dd = DataDictionary::new();
        //let result = dd.load("../../../spec/FIX43.xml");
        let result = dd.load_from_string(include_str!("../../../spec/FIX42.xml"));
        println!("{:?}", result);
        assert!(result.is_ok());
    }

    #[test]
    pub fn fix43() {
        let mut dd = DataDictionary::new();
        //let result = dd.load("../../../spec/FIX43.xml");
        let result = dd.load_from_string(include_str!("../../../spec/FIX43.xml"));
        println!("{:?}", result);
        assert!(result.is_ok());
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
        let mut dd = DataDictionary::new();
        //let result = dd.load("../../../spec/FIX43.xml");
        let result = dd.load_from_string(include_str!("../../../spec/FIX44.xml"));
        println!("{:?}", result);
        assert!(result.is_ok());
    }

    #[test]
    pub fn fix50() {
        let mut dd = DataDictionary::new();
        //let result = dd.load("../../../spec/FIX43.xml");
        let result = dd.load_from_string(include_str!("../../../spec/FIX50.xml"));
        println!("{:?}", result);
        assert!(result.is_ok());
    }

    #[test]
    pub fn fix50sp1() {
        let mut dd = DataDictionary::new();
        //let result = dd.load("../../../spec/FIX43.xml");
        let result = dd.load_from_string(include_str!("../../../spec/FIX50SP1.xml"));
        println!("{:?}", result);
        assert!(result.is_ok());
    }

    #[test]
    pub fn fix50sp2() {
        let mut dd = DataDictionary::new();
        //let result = dd.load("../../../spec/FIX43.xml");
        let result = dd.load_from_string(include_str!("../../../spec/FIX50SP2.xml"));
        println!("{:?}", result);
        assert!(result.is_ok());
    }
}
