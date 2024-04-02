use super::super::parsed::structs::Struct;
use crate::schema::body_parameters::BodyParameterType;

pub enum GeneratedStruct {
  Struct {
    struct_: Struct,
    type_: BodyParameterType,
  },
  Type(BodyParameterType),
}

impl GeneratedStruct {
  pub fn struct_(struct_: Struct, type_: BodyParameterType) -> Self {
    GeneratedStruct::Struct { struct_, type_ }
  }

  pub fn type_(type_: BodyParameterType) -> Self {
    GeneratedStruct::Type(type_)
  }

  pub fn to_full_type(&self) -> String {
    match self {
      GeneratedStruct::Struct { struct_, type_ } => {
        let type_name = struct_.name.to_string();

        match type_ {
          BodyParameterType::Object => type_name,
          BodyParameterType::Array => format!("Vec<{}>", type_name),
          BodyParameterType::ObjectOrNull => format!("Option<{}>", type_name),
          BodyParameterType::ArrayOfObjects => format!("Vec<{}>", type_name),
          BodyParameterType::ArrayOfObjectsOrNull => format!("Option<Vec<{}>>", type_name),
          _ => {
            unreachable!(
              "This type is not implemented: {:?} for struct: {:#?}",
              type_, struct_
            )
          }
        }
      }
      GeneratedStruct::Type(type_) => match type_ {
        BodyParameterType::String => "String".to_string(),
        BodyParameterType::Number => "f64".to_string(),
        BodyParameterType::Integer => "i64".to_string(),
        BodyParameterType::Boolean => "bool".to_string(),
        BodyParameterType::StringOrNull => "Option<String>".to_string(),
        BodyParameterType::BooleanOrNull => "Option<bool>".to_string(),
        BodyParameterType::NumberOrNull => "Option<f64>".to_string(),
        BodyParameterType::IntegerOrNull => "Option<i64>".to_string(),
        BodyParameterType::ArrayOfStrings => "Vec<String>".to_string(),
        BodyParameterType::ArrayOfIntegers => "Vec<i64>".to_string(),
        BodyParameterType::Array => "Vec<serde_json::Value>".to_string(),
        BodyParameterType::StringOrNumber => "StringOrNumber".to_string(),
        BodyParameterType::StringOrInteger => "StringOrNumber".to_string(),
        BodyParameterType::BooleanOrString => "StringOrBool".to_string(),
        BodyParameterType::ObjectOrNull => "Option<serde_json::Value>".to_string(),
        BodyParameterType::ObjectOrString => "serde_json::Value".to_string(),
        BodyParameterType::Object => "serde_json::Value".to_string(),
        BodyParameterType::ArrayOfStringsOrNull => "Option<Vec<String>>".to_string(),
        // TODO: StringOrNumber
        BodyParameterType::NullOrStringOrInteger => "Option<serde_json::Value>".to_string(),
        BodyParameterType::NullOrStringOrArray => "serde_json::Value".to_string(),
        BodyParameterType::ObjectOrArrayOrString => "serde_json::Value".to_string(),
        _ => {
          unreachable!("This type is not implemented: {:?}", type_)
        }
      },
    }
  }
}
