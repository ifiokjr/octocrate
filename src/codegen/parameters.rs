use crate::schema::parameters::Parameter;

#[derive(Debug, Clone)]
pub struct Parameters {
  pub parameters: Vec<Parameter>,
}

impl Parameters {
  pub fn new(parameters: Vec<Parameter>) -> Self {
    Self { parameters }
  }
}
