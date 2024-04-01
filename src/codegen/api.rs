use super::{
  body_parameters::BodyParameters, parameters::Parameters, schema_parser::ParsedStruct,
  structs::Struct,
};
use crate::schema::{
  api::{APISchema, Method},
  response::Response,
};

#[derive(Clone, Debug)]
pub struct API {
  pub name: String,
  pub path: String,
  pub method: Method,
  pub parameters: Parameters,
  pub body: Option<Struct>,
  pub query: Parameters,
  pub response: Response,
}

impl From<APISchema> for API {
  fn from(api_schema: APISchema) -> Self {
    let mut query = vec![];
    let mut parameters = vec![];

    for parameter in api_schema.parameters {
      if parameter.position.is_query() {
        query.push(parameter);
      } else {
        parameters.push(parameter);
      }
    }

    let response = api_schema.code_examples[0].response.clone();

    let body_parameters = BodyParameters::new(api_schema.body_parameters);

    let body = body_parameters.generate_struct(&api_schema.title);

    API {
      name: api_schema.title,
      path: api_schema.path,
      method: api_schema.method,
      query: Parameters::new(query),
      parameters: Parameters::new(parameters),
      body,
      response,
    }
  }
}
