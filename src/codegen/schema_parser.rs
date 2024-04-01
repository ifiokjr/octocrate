use super::{
  schema_types::SchemaTypes,
  structs::{Enum, EnumField, Referable, Struct, StructName},
};
use crate::{codegen::structs::StructField, schema::schema::Schema};
use std::sync::Arc;

#[derive(Debug)]
pub enum ParsedStruct {
  Struct(Struct),
  Enum(Enum),
  Type {
    type_name: String,
    reference: Option<Arc<Box<dyn Referable>>>,
  },
}

pub struct SchemaParser {
  prefixs: Vec<String>,
}

impl SchemaParser {
  pub fn new() -> Self {
    Self { prefixs: vec![] }
  }

  pub fn parse(&mut self, prefix_name: &String, schema: &Schema) -> ParsedStruct {
    let prefix_name = StructName::from(prefix_name);
    self.prefixs.push(prefix_name.to_string());

    let parsed = self.parse_schema(schema);

    self.prefixs.pop();

    parsed
  }

  fn parse_schema(&mut self, schema: &Schema) -> ParsedStruct {
    if schema.enum_.is_some() {
      return self.parse_enum(&schema);
    }

    if let Some(one_of) = &schema.one_of {
      return self.parse_one_of_like(one_of);
    }

    if let Some(all_of) = &schema.all_of {
      return self.parse_one_of_like(all_of);
    }

    if let Some(any_of) = &schema.any_of {
      return self.parse_one_of_like(any_of);
    }

    if schema.items.is_some() {
      return self.parse_items(&schema);
    }

    if schema.properties.is_some() {
      return self.parse_properties(&schema);
    }

    let schema_types = if let Some(types) = &schema.type_ {
      SchemaTypes::from(types)
    } else {
      SchemaTypes::default()
    };

    let type_name = schema_types.to_full_type();

    ParsedStruct::Type {
      type_name,
      reference: None,
    }
  }

  fn parse_properties(&mut self, schema: &Schema) -> ParsedStruct {
    let struct_name = schema.title.clone().unwrap_or(self.prefixs.join(""));
    let mut struct_ = Struct::new(struct_name);

    let properties = schema.properties.as_ref().unwrap();

    for (name, schema) in properties {
      let name = schema.title.clone().unwrap_or(name.clone());
      let parsed = self.parse(&name, schema);

      match parsed {
        ParsedStruct::Struct(generated) => {
          let mut struct_field = StructField::new(name.clone(), generated.name.to_string());

          if let Some(description) = &schema.description {
            struct_field.set_description(description.clone());
          }

          struct_field.reference(generated);

          struct_.add_field(struct_field);
        }
        ParsedStruct::Enum(generated) => {
          let mut struct_field = StructField::new(name.clone(), generated.name.to_string());

          if let Some(description) = &schema.description {
            struct_field.set_description(description.clone());
          }

          struct_field.reference(generated);

          struct_.add_field(struct_field);
        }
        ParsedStruct::Type {
          type_name,
          reference,
        } => {
          let mut struct_field = StructField::new(name.clone(), type_name);

          if let Some(description) = &schema.description {
            struct_field.set_description(description.clone());
          }

          struct_field.reference = reference;

          struct_.add_field(struct_field);
        }
      }
    }

    if let Some(description) = &schema.description {
      struct_.set_description(description.clone());
    }

    if let Some(required) = &schema.required {
      for field in struct_.fields.iter_mut() {
        if !required.contains(&field.name) {
          field.type_name = format!("Option<{}>", field.type_name);
        }
      }
    }

    ParsedStruct::Struct(struct_)
  }

  fn parse_one_of_like(&mut self, schemas: &Vec<Schema>) -> ParsedStruct {
    let mut parsed = vec![];

    let mut schema_types = SchemaTypes::default();

    for schema in schemas {
      if let Some(type_) = &schema.type_ {
        schema_types.merge(&SchemaTypes::from(type_));
      }

      parsed.push(self.parse_schema(schema));
    }

    let struct_or_enum_count = parsed
      .iter()
      .filter(|p| match p {
        ParsedStruct::Struct(_) => true,
        ParsedStruct::Enum(_) => true,
        _ => false,
      })
      .count();

    match struct_or_enum_count {
      0 => {
        let type_name = schema_types.to_full_type();
        ParsedStruct::Type {
          type_name,
          reference: None,
        }
      }
      1 => {
        let struct_or_enum = parsed
          .iter()
          .find(|p| match p {
            ParsedStruct::Struct(_) => true,
            ParsedStruct::Enum(_) => true,
            _ => false,
          })
          .unwrap();

        match struct_or_enum {
          ParsedStruct::Struct(struct_) => {
            let struct_name = struct_.name.to_string();
            let full_type = schema_types.to_full_type_with_object(&struct_name);

            if struct_name == full_type {
              return ParsedStruct::Struct(struct_.clone());
            }

            ParsedStruct::Type {
              type_name: full_type,
              reference: Some(Arc::new(Box::new(struct_.clone()) as Box<dyn Referable>)),
            }
          }
          ParsedStruct::Enum(enum_) => {
            let enum_name = enum_.name.to_string();
            let full_type = schema_types.to_full_type_with_object(&enum_name);

            if enum_name == full_type {
              return ParsedStruct::Enum(enum_.clone());
            }

            ParsedStruct::Type {
              type_name: full_type,
              reference: Some(Arc::new(Box::new(enum_.clone()) as Box<dyn Referable>)),
            }
          }
          _ => unreachable!(),
        }
      }
      _ => {
        let struct_name = self.prefixs.join("");
        let mut enum_ = Enum::new(struct_name);
        let mut is_optional = false;

        for p in parsed {
          match p {
            ParsedStruct::Struct(struct_) => {
              let mut enum_field = EnumField::new(struct_.name.to_string());

              if let Some(description) = struct_.description.clone() {
                enum_field.set_description(description.to_string());
              }

              enum_field.reference(struct_.clone());

              enum_field.set_type_name(struct_.name.to_string());

              enum_.add_field(enum_field);
            }
            ParsedStruct::Enum(generated) => {
              let mut enum_field = EnumField::new(generated.name.to_string());

              if let Some(description) = generated.description.clone() {
                enum_field.set_description(description.to_string());
              }

              enum_field.set_type_name(generated.name.to_string());

              enum_field.reference(generated.clone());

              enum_.add_field(enum_field);
            }
            ParsedStruct::Type {
              type_name,
              reference,
            } => {
              if type_name == "Null" {
                is_optional = true;
              } else {
                let mut enum_field = EnumField::new(type_name.clone());

                if let Some(reference) = reference {
                  enum_field.reference = Some(reference);
                }

                enum_field.set_type_name(type_name.clone());

                enum_.add_field(enum_field);
              }
            }
          }
        }

        if is_optional {
          let type_name = schema_types.to_full_type_with_object(&enum_.name.to_string());

          return ParsedStruct::Type {
            type_name,
            reference: Some(Arc::new(Box::new(enum_) as Box<dyn Referable>)),
          };
        }

        ParsedStruct::Enum(enum_)
      }
    }
  }

  fn parse_items(&mut self, schema: &Schema) -> ParsedStruct {
    let items = schema.items.as_ref().unwrap();

    let schema_types = if let Some(types) = &schema.type_ {
      SchemaTypes::from(types)
    } else {
      SchemaTypes::default()
    };

    let generated = self.parse_schema(items);

    let (type_name, reference) = match generated {
      ParsedStruct::Struct(struct_) => {
        let type_name = struct_.name.to_string();
        let reference = Some(Arc::new(Box::new(struct_) as Box<dyn Referable>));

        (type_name, reference)
      }
      ParsedStruct::Enum(enum_) => {
        let type_name = enum_.name.to_string();
        let reference = Some(Arc::new(Box::new(enum_) as Box<dyn Referable>));

        (type_name, reference)
      }
      ParsedStruct::Type {
        type_name,
        reference,
      } => (type_name, reference),
    };

    let full_type = schema_types.to_full_type_with_object(&type_name);

    return ParsedStruct::Type {
      type_name: full_type,
      reference,
    };
  }

  fn parse_enum(&mut self, schema: &Schema) -> ParsedStruct {
    let enum_name = self.prefixs.join("");
    let mut enum_ = Enum::new(enum_name);
    let mut is_optional = false;

    if let Some(description) = &schema.description {
      enum_.set_description(description.clone());
    }

    if let Some(enum_values) = &schema.enum_ {
      for enum_value in enum_values {
        if let Some(enum_value) = &enum_value {
          if enum_value == "null" {
            is_optional = true;
            continue;
          }

          let mut enum_field = EnumField::new(enum_value.to_string());
          let enum_value = StructName::from(enum_value);
          enum_field.set_type_name(enum_value.to_string());

          enum_.add_field(enum_field);
        } else {
          is_optional = true;
        }
      }
    }

    if is_optional {
      return ParsedStruct::Type {
        type_name: format!("Option<{}>", enum_.name.to_string()),
        reference: Some(Arc::new(Box::new(enum_) as Box<dyn Referable>)),
      };
    }

    ParsedStruct::Enum(enum_)
  }
}

#[cfg(test)]
mod schema_tests {
  use super::*;

  #[test]
  fn test_schema_parser() {
    let json = r#"{
      "type": "array",
      "items": {
        "title": "Tag",
        "description": "Tag",
        "type": "object",
        "properties": {
          "name": {
            "type": "string",
            "examples": [
              "v0.1"
            ]
          },
          "commit": {
            "type": "object",
            "properties": {
              "sha": {
                "type": "string"
              },
              "url": {
                "type": "string",
                "format": "uri"
              }
            },
            "required": [
              "sha",
              "url"
            ]
          },
          "zipball_url": {
            "type": "string",
            "format": "uri",
            "examples": [
              "https://github.com/octocat/Hello-World/zipball/v0.1"
            ]
          },
          "tarball_url": {
            "type": "string",
            "format": "uri",
            "examples": [
              "https://github.com/octocat/Hello-World/tarball/v0.1"
            ]
          },
          "node_id": {
            "type": "string"
          }
        },
        "required": [
          "name",
          "node_id",
          "commit",
          "zipball_url",
          "tarball_url"
        ]
      }
    }"#;

    let schema: Schema = serde_json::from_str(json).unwrap();

    let mut parser = SchemaParser::new();

    let generated = parser.parse(&"Tag".to_string(), &schema);

    match generated {
      ParsedStruct::Type {
        type_name,
        reference,
      } => {
        assert_eq!(type_name, "Vec<Tag>");
        assert!(reference.is_some());
      }
      _ => panic!("Expected struct"),
    }
  }

  #[test]
  fn test_schema_parser_with_properties() {
    let json = r#"{
      "type": "object",
      "required": [
        "artifacts"
      ],
      "properties": {
        "total_count": {
          "type": "integer"
        },
        "artifacts": {
          "type": "array",
          "items": {
            "title": "Artifact",
            "description": "An artifact",
            "type": "object",
            "properties": {
              "id": {
                "type": "integer",
                "examples": [
                  5
                ]
              },
              "node_id": {
                "type": "string",
                "examples": [
                  "MDEwOkNoZWNrU3VpdGU1"
                ]
              },
              "name": {
                "description": "The name of the artifact.",
                "type": "string",
                "examples": [
                  "AdventureWorks.Framework"
                ]
              },
              "size_in_bytes": {
                "description": "The size in bytes of the artifact.",
                "type": "integer",
                "examples": [
                  12345
                ]
              },
              "url": {
                "type": "string",
                "examples": [
                  "https://api.github.com/repos/github/hello-world/actions/artifacts/5"
                ]
              },
              "archive_download_url": {
                "type": "string",
                "examples": [
                  "https://api.github.com/repos/github/hello-world/actions/artifacts/5/zip"
                ]
              },
              "expired": {
                "description": "Whether or not the artifact has expired.",
                "type": "boolean"
              },
              "created_at": {
                "type": [
                  "string",
                  "null"
                ],
                "format": "date-time"
              },
              "expires_at": {
                "type": [
                  "string",
                  "null"
                ],
                "format": "date-time"
              },
              "updated_at": {
                "type": [
                  "string",
                  "null"
                ],
                "format": "date-time"
              },
              "workflow_run": {
                "type": [
                  "object",
                  "null"
                ],
                "properties": {
                  "id": {
                    "type": "integer",
                    "examples": [
                      10
                    ]
                  },
                  "repository_id": {
                    "type": "integer",
                    "examples": [
                      42
                    ]
                  },
                  "head_repository_id": {
                    "type": "integer",
                    "examples": [
                      42
                    ]
                  },
                  "head_branch": {
                    "type": "string",
                    "examples": [
                      "main"
                    ]
                  },
                  "head_sha": {
                    "type": "string",
                    "examples": [
                      "009b8a3a9ccbb128af87f9b1c0f4c62e8a304f6d"
                    ]
                  }
                }
              }
            },
            "required": [
              "id",
              "node_id",
              "name",
              "size_in_bytes",
              "url",
              "archive_download_url",
              "expired",
              "created_at",
              "expires_at",
              "updated_at"
            ]
          }
        }
      }
    }"#;

    let schema: Schema = serde_json::from_str(json).unwrap();

    let mut parser = SchemaParser::new();

    let _generated = parser.parse(&"Response".to_string(), &schema);

    // println!("{:#?}", generated);
  }

  #[test]
  fn test_schema_parser_with_enum() {
    let json = r#"{
      "type": "object",
      "properties": {
        "state": {
          "type": "string",
          "enum": [
            "open",
            "closed",
            "null"
          ]
        }
      }
    }"#;

    let schema: Schema = serde_json::from_str(json).unwrap();

    let mut parser = SchemaParser::new();

    let _generated = parser.parse(&"Response".to_string(), &schema);

    // println!("{:#?}", generated);
  }
}
