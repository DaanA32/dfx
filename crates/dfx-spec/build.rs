use std::path::Path;

use dfx_base::data_dictionary::{DDField, DDGroup, DDMap, DataDictionary};
use heck::{ToPascalCase, ToSnakeCase};
use indoc::indoc;

fn field_type(field_type: &std::sync::Arc<str>) -> &'static str {
    match field_type.as_ref() {
        "CHAR" => "char",
        "INT" => "i64",
        "LENGTH" | "NUMINGROUP" | "SEQNUM" => "usize",
        "AMT" | "PERCENTAGE" | "PRICE" | "QTY" | "PRICEOFFSET" | "FLOAT" => "Decimal",
        "TZTIMESTAMP" | "UTCTIMESTAMP" | "TIME" => "DateTime",
        "UTCDATE" | "UTCDATEONLY" | "DATE" => "Date",
        "UTCTIMEONLY" => "Time",
        "BOOLEAN" => "bool",
        //String
        "COUNTRY"
        | "CURRENCY"
        | "DATA"
        | "DAYOFMONTH"
        | "EXCHANGE"
        | "LANGUAGE"
        | "LOCALMKTDATE"
        | "MONTHYEAR"
        | "MULTIPLECHARVALUE"
        | "MULTIPLESTRINGVALUE"
        | "MULTIPLEVALUESTRING"
        | "STRING"
        | "TZTIMEONLY"
        | "XMLDATA" => "&str",
        v => panic!("unknown type {v}"),
    }
}

fn generate_field(field: &DDField) -> String {
    format!(
        indoc!(
            r"
            use std::borrow::Cow;

            use dfx_base::field_map::Tag;
            use dfx_base::field_map::Field;
            use dfx_base::fields::ConversionError;
            #[allow(unused)]
            use dfx_base::fields::converters::*;

            /// {field_name}
            #[derive(Clone, Debug, PartialEq, Eq)]
            pub struct {field_name}<'a> {{
                inner: Cow<'a, Field>
            }}

            impl<'a> {field_name}<'a> {{
                pub fn new(value: {field_type}) -> Self {{
                    let field = Field::new( {field_name}::tag(), value );
                    Self {{
                        inner: Cow::Owned(field)
                    }}
                }}
                pub const fn tag() -> Tag {{
                    {tag}
                }}
                pub fn value(&self) -> {field_type} {{
                    // This will not panic due to the constraints on Field::new and the TryFrom impl
                    self.inner.as_value().unwrap()
                }}
            }}

            impl<'a> std::convert::TryFrom<&'a Field> for {field_name}<'a> {{
                type Error = ConversionError;
                fn try_from(field: &'a Field) -> Result<Self, ConversionError> {{
                    if field.tag() != Self::tag() {{
                        return Err(ConversionError::InvalidTag {{ tag: field.tag(), expected: Self::tag() }});
                    }}
                    let _t: {field_type} = field.as_value()?;
                    Ok(Self {{ inner: Cow::Borrowed(field) }})
                }}
            }}
            impl<'a> std::convert::TryFrom<Field> for {field_name}<'a> {{
                type Error = ConversionError;
                fn try_from(field: Field) -> Result<Self, ConversionError> {{
                    if field.tag() != Self::tag() {{
                        return Err(ConversionError::InvalidTag {{ tag: field.tag(), expected: Self::tag() }});
                    }}
                    let _t: {field_type} = field.as_value()?;
                    Ok(Self {{ inner: Cow::Owned(field) }})
                }}
            }}
            impl<'a> Into<&'a Field> for &'a {field_name}<'a> {{
                fn into(self) -> &'a Field {{
                    self.inner.as_ref()
                }}
            }}
            impl<'a> Into<Field> for &'a {field_name}<'a> {{
                fn into(self) -> Field {{
                    self.inner.as_ref().clone()
                }}
            }}
            impl<'a> Into<Field> for {field_name}<'a> {{
                fn into(self) -> Field {{
                    self.inner.into_owned()
                }}
            }}
            "
        ),
        field_name = field.name().to_pascal_case(),
        tag = field.tag(),
        field_type = field_type(field.field_type()),
    )
}

fn generate_message_field(field: &DDField) -> String {
    format!(
        r"
    pub fn {field_name}<'b: 'a>(&'b self) -> Option<{field_type}<'b>> {{
        self.inner.get_field({field_type}::tag()).map(|v| v.try_into().ok()).flatten()
    }}
    pub fn set_{field_name}<'b: 'a>(&mut self, {field_name}: {field_type}<'b>) {{
        self.inner.to_mut().set_field({field_name});
    }}
        ",
        field_name = field.name().to_snake_case(),
        field_type = field.name().to_pascal_case(),
    )
}

fn generate_message_group(group: &DDGroup) -> String {
    format!(
        r"
    pub fn {field_name}(&self) -> Option<{field_type}> {{
        todo!()
    }}
    pub fn set_{field_name}(&mut self, _{field_name}: {field_type}) {{
        todo!()
    }}
        ",
        field_name = group.name().to_snake_case(),
        field_type = group.name().to_pascal_case(),
    )
}

fn generate_groups(message: &DDMap) -> String {
    let mut s = String::new();
    for group in message.groups().values() {
        s.push_str(
            format!(
                r"
pub struct {group_name} {{

}}
",
                group_name = group.name().to_pascal_case(),
            )
            .as_str(),
        );
    }
    s
}

fn generate_message_fields_groups(message: &DDMap) -> String {
    let mut s = String::new();
    if message.fields().is_empty() && message.groups().is_empty() {
        s.push_str(
            r"
pub fn value(&self) -> &dfx_base::field_map::FieldMap {
    &self.inner
}
"
            .to_string()
            .as_str(),
        );
    } else {
        for field in message.fields().values() {
            s.push_str(generate_message_field(field).as_str());
        }
        for group in message.groups().values() {
            s.push_str(generate_message_group(group).as_str());
        }
    }
    s
}

fn generate_message_factory(version: &str, data_dictionary: &DataDictionary) -> String {
    format!(
        indoc!(
            r#"
            use dfx_base::message_factory::MessageFactory;

            #[derive(Debug, Clone, Copy)]
            pub struct {version}MessageFactory;

            impl MessageFactory for {version}MessageFactory {{
                fn get_supported_begin_strings(&self) -> Vec<String> {{
                    vec![String::from("{version_upper}")]
                }}

                fn create(&self, begin_string: &str, msg_type: &str) -> Result<dfx_base::message::Message, dfx_base::message_factory::MessageFactoryError> {{
                    // check if begin string == {version_upper}
                    todo!("{{begin_string}} {{msg_type}}")
                }}

                fn create_group(&self, begin_string: &str, msg_type: &str, group_counter_tag: dfx_base::field_map::Tag) -> Option<dfx_base::field_map::Group> {{
                    {create_group}
                }}
            }}

            "#
        ),
        version = version.to_pascal_case(),
        version_upper = version.to_pascal_case().to_uppercase(),
        create_group = generate_message_factory_create_group(data_dictionary),
    )
}

fn generate_message_factory_create_group(_data_dictionary: &DataDictionary) -> String {
    let mut function = String::new();
    function.push_str(
        r#"// TODO function
        todo!("{begin_string} {msg_type} {group_counter_tag}")"#
            .to_string()
            .as_str(),
    );
    // TODO
    function
}

fn generate_message(message: &DDMap, version: &str) -> String {
    format!(
        indoc!(
            r"
            use std::borrow::Cow;

            use dfx_base::message::Message;
            {import_fields}

            /// {message_name}
            #[derive(Clone, Debug)]
            pub struct {message_name}<'a> {{
                inner: Cow<'a, Message>
            }}

            impl<'a> {message_name}<'a> {{
                //TODO implement
                {functions}
            }}

            {groups}
            "
        ),
        import_fields = if message.fields().is_empty() && message.groups().is_empty() {
            String::new()
        } else {
            format!("use crate::{version}::fields::*;")
        },
        message_name = message.name().to_pascal_case(),
        functions = generate_message_fields_groups(message),
        groups = generate_groups(message)
    )
}

//TODO move to codegen crate?
fn codegen(filename: &str) {
    // let out_dir = std::env::var_os("out_dir").unwrap();
    let out_dir = "src/";
    let data_dictionary =
        DataDictionary::from_file(filename).expect("Unable to read filename {filename}");

    let version = data_dictionary.version().unwrap();
    let _version_mod_name = version.to_ascii_lowercase().replace('.', "");

    let out_dir = Path::new(&out_dir).join(version.to_ascii_lowercase().replace('.', ""));
    if std::fs::read_dir(&out_dir).is_err() {
        std::fs::create_dir(&out_dir).unwrap();
    }

    let module_path = out_dir.join("mod.rs");
    let mut module = String::with_capacity(8192);
    module.push_str(format!("pub mod r#{name};\n", name = "message_factory").as_str());
    // module.push_str(format!("pub mod r#{name};\n", name = "fields").as_str());
    // module.push_str(format!("pub mod r#{name};\n", name = "messages").as_str());
    std::fs::write(&module_path, module).unwrap();

    let module_path = out_dir.join("message_factory.rs");
    let mut module = String::with_capacity(8192);
    module.push_str(&generate_message_factory(version, &data_dictionary));
    std::fs::write(&module_path, module).unwrap();

    // // fields
    // let fields_dir = out_dir.join("fields");
    // if std::fs::read_dir(&fields_dir).is_err() {
    //     std::fs::create_dir(&fields_dir).unwrap();
    // }

    // let module_path = fields_dir.join("mod.rs");
    // let mut module = String::with_capacity(8192);
    // for (name, field) in data_dictionary.fields_by_name() {
    //     let dest_path = fields_dir.join(format!("{}.rs", name.to_snake_case()));
    //     std::fs::write(dest_path, generate_field(field)).unwrap();
    //     module.push_str(format!("mod r#{name};\npub use r#{name}::*;\n", name = name.to_snake_case()).as_str())
    // }
    // std::fs::write(&module_path, module).unwrap();

    // // messages
    // let fields_dir = out_dir.join("messages");
    // if std::fs::read_dir(&fields_dir).is_err() {
    //     std::fs::create_dir(&fields_dir).unwrap();
    // }

    // let module_path = fields_dir.join("mod.rs");
    // let mut module = String::with_capacity(8192);
    // for (_, message) in data_dictionary.messages() {
    //     let dest_path = fields_dir.join(format!("{}.rs", message.name().to_snake_case()));
    //     std::fs::write(dest_path, generate_message(message, &version_mod_name)).unwrap();
    //     module.push_str(format!("mod r#{name};\npub use r#{name}::*;\n", name = message.name().to_snake_case()).as_str())
    // }
    // std::fs::write(&module_path, module).unwrap();
}

fn main() {
    println!("cargo:rerun-if-changed=crate/dfx-spec/build.rs");
    println!("cargo:rerun-if-changed=spec/");

    codegen("../../spec/FIX44.xml");
}
