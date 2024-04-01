use super::schema::Schema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Position {
  Query,
  Path,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Parameter {
  pub name: String,
  pub description: String,
  #[serde(rename = "in")]
  pub position: Position,
  pub required: Option<bool>,
  pub schema: Schema,
}

impl Position {
  pub fn is_query(&self) -> bool {
    match self {
      Position::Query => true,
      _ => false,
    }
  }
}

#[cfg(test)]
mod parameters_test {
  use super::super::schema::{SchemaType, SchemaTypeDefination};
  use super::*;

  #[test]
  fn test_parameter() {
    let json = r#"[
      {
        "name": "owner",
        "description": "<p>The account owner of the repository. The name is not case sensitive.</p>",
        "in": "path",
        "required": true,
        "schema": {
          "type": "string"
        }
      },
      {
        "name": "repo",
        "description": "<p>The name of the repository without the <code>.git</code> extension. The name is not case sensitive.</p>",
        "in": "path",
        "required": true,
        "schema": {
          "type": "string"
        }
      },
      {
        "name": "per_page",
        "description": "<p>The number of results per page (max 100). For more information, see \"<a href=\"https://docs.github.com/enterprise-cloud@latest//rest/using-the-rest-api/using-pagination-in-the-rest-api\">Using pagination in the REST API</a>.\"</p>",
        "in": "query",
        "schema": {
          "type": "integer",
          "default": 30
        }
      }
    ]"#;

    let parameters: Vec<Parameter> = serde_json::from_str(json).unwrap();

    assert_eq!(parameters.len(), 3);

    let parameter = &parameters[0];

    assert_eq!(parameter.name, "owner");
    assert_eq!(
      parameter.description,
      "<p>The account owner of the repository. The name is not case sensitive.</p>"
    );
    assert_eq!(parameter.position, Position::Path);
    assert_eq!(parameter.required, Some(true));
    assert_eq!(
      parameter.schema.type_.clone().unwrap(),
      SchemaTypeDefination::String(SchemaType::String.to_string())
    );

    let parameter = &parameters[1];

    assert_eq!(parameter.name, "repo");
    assert_eq!(
      parameter.description,
      "<p>The name of the repository without the <code>.git</code> extension. The name is not case sensitive.</p>"
    );
    assert_eq!(parameter.position, Position::Path);
    assert_eq!(parameter.required, Some(true));
    assert_eq!(
      parameter.schema.type_.clone().unwrap(),
      SchemaTypeDefination::String(SchemaType::String.to_string())
    );

    let parameter = &parameters[2];

    assert_eq!(parameter.name, "per_page");
    assert_eq!(
      parameter.description,
      "<p>The number of results per page (max 100). For more information, see \"<a href=\"https://docs.github.com/enterprise-cloud@latest//rest/using-the-rest-api/using-pagination-in-the-rest-api\">Using pagination in the REST API</a>.\"</p>"
    );
    assert_eq!(parameter.position, Position::Query);
    assert_eq!(parameter.required, None);
    assert_eq!(
      parameter.schema.type_.clone().unwrap(),
      SchemaTypeDefination::String(SchemaType::Integer.to_string())
    );
  }
}
