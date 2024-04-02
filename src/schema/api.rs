use super::{body_parameters::BodyParameter, parameters::Parameter, response::Response};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Method {
  GET,
  POST,
  PUT,
  PATCH,
  DELETE,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct APICodeExampleSchema {
  pub key: String,
  pub response: Response,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct APISchema {
  pub title: String,
  pub category: String,
  #[serde(rename = "subcategory")]
  pub sub_category: String,
  #[serde(rename = "requestPath")]
  pub path: String,
  #[serde(rename = "verb")]
  pub method: Method,
  pub parameters: Vec<Parameter>,
  #[serde(rename = "bodyParameters")]
  pub body_parameters: Vec<BodyParameter>,
  #[serde(rename = "codeExamples")]
  pub code_examples: Vec<APICodeExampleSchema>,
}

impl ToString for Method {
  fn to_string(&self) -> String {
    match &self {
      Method::GET => "GET".to_string(),
      Method::POST => "POST".to_string(),
      Method::PUT => "PUT".to_string(),
      Method::PATCH => "PATCH".to_string(),
      Method::DELETE => "DELETE".to_string(),
    }
  }
}
