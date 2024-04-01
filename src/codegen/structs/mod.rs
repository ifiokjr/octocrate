use regex::Regex;
use std::{fmt::Debug, ops::Deref, sync::Arc};

pub trait Referable: Debug {}

#[derive(Clone, Debug, PartialEq)]
pub struct Description(String);

// TODO: rename https://github.com/serde-rs/serde/blob/master/serde_derive/src/internals/case.rs
#[derive(Clone, Debug, PartialEq)]
pub struct StructName(String);

#[derive(Clone, Debug)]
pub struct StructField {
  pub name: String,
  pub description: Option<Description>,
  pub type_name: String,
  pub reference: Option<Arc<Box<dyn Referable>>>,
}

#[derive(Clone, Debug)]
pub struct Struct {
  pub name: StructName,
  pub description: Option<Description>,
  pub fields: Vec<StructField>,
}

#[derive(Clone, Debug)]
pub struct EnumField {
  pub name: String,
  /// type name
  /// String(String) / Data(Data)
  pub type_name: Option<String>,
  pub description: Option<Description>,
  pub reference: Option<Arc<Box<dyn Referable>>>,
}

#[derive(Clone, Debug)]
pub struct Enum {
  pub name: StructName,
  pub description: Option<Description>,
  pub fields: Vec<EnumField>,
}

impl Struct {
  pub fn new_with_description(name: String, description: String) -> Self {
    Self {
      name: StructName::from(&name),
      description: Some(Description::from(&description)),
      fields: vec![],
    }
  }

  pub fn new(name: String) -> Self {
    Self {
      name: StructName::from(&name),
      description: None,
      fields: vec![],
    }
  }

  pub fn add_field(&mut self, field: StructField) {
    self.fields.push(field);
  }

  pub fn set_description(&mut self, description: String) {
    self.description = Some(Description::from(&description));
  }
}

impl StructField {
  pub fn new_with_description(name: String, type_name: String, description: String) -> Self {
    Self {
      name,
      description: Some(Description::from(&description)),
      type_name,
      reference: None,
    }
  }

  pub fn new(name: String, type_name: String) -> Self {
    Self {
      name,
      description: None,
      type_name,
      reference: None,
    }
  }

  pub fn set_description(&mut self, description: String) {
    self.description = Some(Description::from(&description));
  }

  pub fn reference<T>(&mut self, reference: T)
  where
    T: Referable + 'static,
  {
    self.reference = Some(Arc::new(Box::new(reference)));
  }
}

impl Enum {
  pub fn new(name: String) -> Self {
    Self {
      name: StructName::from(&name),
      fields: vec![],
      description: None,
    }
  }

  pub fn add_field(&mut self, field: EnumField) {
    self.fields.push(field);
  }

  pub fn set_description(&mut self, description: String) {
    self.description = Some(Description::from(&description));
  }
}

impl EnumField {
  pub fn new(name: String) -> Self {
    Self {
      name,
      type_name: None,
      reference: None,
      description: None,
    }
  }

  pub fn reference<T>(&mut self, reference: T)
  where
    T: Referable + 'static,
  {
    self.reference = Some(Arc::new(Box::new(reference)));
  }

  pub fn set_description(&mut self, description: String) {
    self.description = Some(Description::from(&description));
  }

  pub fn set_type_name(&mut self, type_name: String) {
    self.type_name = Some(type_name);
  }
}

impl From<&String> for Description {
  fn from(description: &String) -> Self {
    // Replace <T> / </T> with empty
    let regexp = r#"(</?[a-z]+>)"#;

    let re = Regex::new(regexp).unwrap();

    let cleaned = re.replace_all(&description, "").trim().to_string();

    Self(cleaned)
  }
}

impl From<&String> for StructName {
  fn from(name: &String) -> Self {
    let mut pascal = String::new();
    let mut capitalize = true;

    for ch in name.chars() {
      if ch == '_' {
        capitalize = true;
      } else if capitalize {
        pascal.push(ch.to_ascii_uppercase());
        capitalize = false;
      } else {
        pascal.push(ch);
      }
    }

    Self(pascal)
  }
}

impl Deref for StructName {
  type Target = String;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl Deref for Description {
  type Target = String;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl Referable for Struct {}

impl Referable for Enum {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_description_from_string() {
    let description = Description::from(&"<p> Hello, <strong>world!</strong> </p>".to_string());
    assert_eq!(description.0, "Hello, world!");
  }

  #[test]
  fn test_struct_name_from_string() {
    let struct_name = StructName::from(&"hello_world".to_string());
    assert_eq!(struct_name.0, "HelloWorld");
  }
}
