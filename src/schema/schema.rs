use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SchemaType {
  Null,
  Integer,
  Boolean,
  String,
  Object,
  Array,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum SchemaTypeDefination {
  String(String),
  Array(Vec<String>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Schema {
  pub title: Option<String>,
  #[serde(rename = "type")]
  pub type_: Option<SchemaTypeDefination>,
  // pub format: Option<String>,
  pub items: Option<Box<Schema>>,
  pub properties: Option<BTreeMap<String, Schema>>,
  pub required: Option<Vec<String>>,
  pub description: Option<String>,
  #[serde(rename = "enum")]
  pub enum_: Option<Vec<Option<String>>>,
  /// enum no title
  #[serde(rename = "allOf")]
  pub all_of: Option<Vec<Schema>>,
  /// struct with title or empty object
  #[serde(rename = "anyOf")]
  pub any_of: Option<Vec<Schema>>,
  #[serde(rename = "oneOf")]
  pub one_of: Option<Vec<Schema>>,
}

impl TryFrom<&String> for SchemaType {
  type Error = String;

  fn try_from(value: &String) -> Result<Self, Self::Error> {
    match value.as_str() {
      "null" => Ok(SchemaType::Null),
      "integer" => Ok(SchemaType::Integer),
      "string" => Ok(SchemaType::String),
      "object" => Ok(SchemaType::Object),
      "array" => Ok(SchemaType::Array),
      "boolean" => Ok(SchemaType::Boolean),
      _ => Err(format!("Unknown schema type: {}", value)),
    }
  }
}

impl ToString for SchemaType {
  fn to_string(&self) -> String {
    match self {
      SchemaType::Null => "null".to_string(),
      SchemaType::Integer => "integer".to_string(),
      SchemaType::String => "string".to_string(),
      SchemaType::Object => "object".to_string(),
      SchemaType::Array => "array".to_string(),
      SchemaType::Boolean => "boolean".to_string(),
    }
  }
}

#[cfg(test)]
mod schema_tests {
  use super::*;

  #[test]
  fn test_schema() {
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

    assert_eq!(
      schema.type_.unwrap(),
      SchemaTypeDefination::String(SchemaType::Array.to_string())
    );

    let items = schema.items.clone().unwrap();

    assert_eq!(
      items.type_.unwrap(),
      SchemaTypeDefination::String(SchemaType::Object.to_string())
    );

    let properties = items.properties.unwrap();

    assert_eq!(properties.len(), 5);

    let name = properties.get("name").unwrap();

    assert_eq!(
      name.type_.clone().unwrap(),
      SchemaTypeDefination::String(SchemaType::String.to_string())
    );

    let commit = properties.get("commit").unwrap();

    assert_eq!(
      commit.type_.clone().unwrap(),
      SchemaTypeDefination::String(SchemaType::Object.to_string())
    );

    let commit_properties = commit.properties.as_ref().unwrap();

    assert_eq!(commit_properties.len(), 2);

    let sha = commit_properties.get("sha").unwrap();

    assert_eq!(
      sha.type_.clone().unwrap(),
      SchemaTypeDefination::String(SchemaType::String.to_string())
    );

    let url = commit_properties.get("url").unwrap();

    assert_eq!(
      url.type_.clone().unwrap(),
      SchemaTypeDefination::String(SchemaType::String.to_string())
    );

    let required = commit.required.as_ref().unwrap();

    assert_eq!(required.len(), 2);

    let zipball_url = properties.get("zipball_url").unwrap();

    assert_eq!(
      zipball_url.type_.clone().unwrap(),
      SchemaTypeDefination::String(SchemaType::String.to_string())
    );

    let tarball_url = properties.get("tarball_url").unwrap();

    assert_eq!(
      tarball_url.type_.clone().unwrap(),
      SchemaTypeDefination::String(SchemaType::String.to_string())
    );

    let node_id = properties.get("node_id").unwrap();

    assert_eq!(
      node_id.type_.clone().unwrap(),
      SchemaTypeDefination::String(SchemaType::String.to_string())
    );

    let required = items.required.as_ref().unwrap();

    assert_eq!(
      required,
      &["name", "node_id", "commit", "zipball_url", "tarball_url"]
    );
  }
}
