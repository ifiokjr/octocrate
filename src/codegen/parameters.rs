use super::{
  parsed::{
    structs::{Struct, StructField, StructName},
    ParsedStruct,
  },
  schema_parser::SchemaParser,
};
use crate::schema::parameters::Parameter;

#[derive(Debug, Clone)]
pub struct Parameters {
  pub parameters: Vec<Parameter>,
}

impl Parameters {
  pub fn new(parameters: Vec<Parameter>) -> Self {
    Self { parameters }
  }

  pub fn parse(&self, struct_name: &String) -> Option<Struct> {
    if self.parameters.is_empty() {
      return None;
    }

    let struct_name = StructName::from(struct_name);
    let mut new_struct = Struct::new(struct_name.to_string());

    let mut schema_parser = SchemaParser::new();

    for parameter in &self.parameters {
      let generated_struct = schema_parser.parse(
        &format!("{}_{}", struct_name.to_string(), parameter.name),
        &parameter.schema,
      );

      match &generated_struct {
        ParsedStruct::Type {
          type_name,
          reference,
        } => {
          let mut field = StructField::new_with_description(
            parameter.name.clone(),
            type_name.clone(),
            parameter.description.clone(),
          );

          field.reference = reference.clone();

          new_struct.add_field(field);
        }
        ParsedStruct::Enum(enum_) => {
          let mut field = StructField::new_with_description(
            parameter.name.clone(),
            enum_.name.to_string(),
            parameter.description.clone(),
          );

          field.reference(enum_.clone());

          new_struct.add_field(field);
        }
        ParsedStruct::Struct(struct_) => {
          let mut field = StructField::new_with_description(
            parameter.name.clone(),
            struct_.name.to_string(),
            parameter.description.clone(),
          );

          field.reference(struct_.clone());

          new_struct.add_field(field);
        }
      }
    }

    Some(new_struct)
  }
}
