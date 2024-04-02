use super::super::{
  body_parameters::BodyParameters,
  parameters::Parameters,
  parsed::structs::{Referable, Struct},
  parsed::ParsedStruct,
  schema_parser::SchemaParser,
};
use crate::schema::api::{APISchema, Method};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct ParsedAPI {
  pub name: String,
  pub path: String,
  pub method: Method,
  pub parameters: Option<Struct>,
  pub body: Option<Struct>,
  pub query: Option<Struct>,
  pub response: Option<ParsedStruct>,
}

#[derive(Clone, Debug)]
pub struct ParsedAPIModule {
  pub name: String,
  pub types: Vec<Arc<Box<dyn Referable>>>,
  pub apis: Vec<ParsedAPI>,
}

impl ParsedAPIModule {
  pub fn new(name: String) -> Self {
    ParsedAPIModule {
      name,
      types: vec![],
      apis: vec![],
    }
  }

  pub fn add_type<T>(&mut self, referable: T)
  where
    T: Referable + 'static,
  {
    self.types.push(Arc::new(Box::new(referable)));
  }

  pub fn add_api(&mut self, api: ParsedAPI) {
    self.apis.push(api);
  }
}

impl From<APISchema> for ParsedAPI {
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

    // Response
    let response = match api_schema.code_examples.first() {
      Some(code_example) => {
        let response = code_example.response.clone();
        let response = response.schema.map(|schema| {
          let mut schema_parser = SchemaParser::new();

          schema_parser.parse(&format!("{}Response", api_schema.title), &schema)
        });

        response
      }
      None => None,
    };

    // Body parameters
    let body_parameters = BodyParameters::new(api_schema.body_parameters);
    let body = body_parameters.parse(&api_schema.title);

    // Query parameters
    let query = Parameters::new(query);
    let query = query.parse(&format!("{}Query", api_schema.title));

    // Parameters
    let parameters = Parameters::new(parameters);
    let parameters = parameters.parse(&format!("{}Parameters", api_schema.title));

    ParsedAPI {
      name: api_schema.title,
      path: api_schema.path,
      method: api_schema.method,
      query,
      parameters,
      body,
      response,
    }
  }
}
