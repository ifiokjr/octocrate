use crate::schema::schema::{Schema, SchemaType, SchemaTypeDefination};

pub struct SchemaTypes {
  pub types: Vec<SchemaType>,
  pub is_null: bool,
  pub is_array: bool,
  pub is_object: bool,
  pub is_string: bool,
  pub is_integer: bool,
  pub is_boolean: bool,
}

impl SchemaTypes {
  pub fn new(types: Vec<SchemaType>) -> Self {
    let is_null = types.contains(&SchemaType::Null);
    let is_array = types.contains(&SchemaType::Array);
    let is_object = types.contains(&SchemaType::Object);
    let is_string = types.contains(&SchemaType::String);
    let is_integer = types.contains(&SchemaType::Integer);
    let is_boolean = types.contains(&SchemaType::Boolean);

    SchemaTypes {
      types,
      is_null,
      is_array,
      is_object,
      is_string,
      is_integer,
      is_boolean,
    }
  }

  pub fn to_full_type(&self) -> String {
    match (
      self.is_array,
      self.is_string,
      self.is_integer,
      self.is_null,
      self.is_boolean,
    ) {
      // [string]
      (false, true, false, false, false) => "String".into(),
      // [string, null]
      (false, true, false, true, false) => "Option<String>".into(),
      // [integer]
      (false, false, true, false, false) => "i64".into(),
      // [integer, null]
      (false, false, true, true, false) => "Option<i64>".into(),
      // [boolean]
      (false, false, false, false, true) => "bool".into(),
      // [boolean, null]
      (false, false, false, true, true) => "Option<bool>".into(),
      // [boolean, string]
      // TODO: BooleanOrString
      (false, true, false, false, true) => "StringOrBool".into(),
      _ => {
        unreachable!(
          "Invalid schema types: array: {}, string: {}, integer: {}, null: {}, boolean: {}",
          self.is_array, self.is_string, self.is_integer, self.is_null, self.is_boolean
        )
      }
    }
  }

  pub fn to_full_type_with_object(&self, object_name: &String) -> String {
    match (
      self.is_object,
      self.is_array,
      self.is_string,
      self.is_integer,
      self.is_boolean,
      self.is_null,
    ) {
      // object
      // [object]
      (true, false, false, false, false, false) => object_name.clone().into(),
      // [object, null]
      (true, false, false, false, false, true) => format!("Option<{}>", object_name),
      // [object, array]
      // TODO: ObjectOrArray
      (true, true, false, false, false, false) => format!("ObjectOrArray<{}>", object_name),
      // [object, array, null]
      (true, true, false, false, false, true) => format!("Option<ObjectOrArray<{}>>", object_name),
      // [object, string]
      // TODO: ObjectOrString
      (true, false, true, false, false, false) => format!("ObjectOrString<{}>", object_name),
      // [object, string, null]
      (true, false, true, false, false, true) => format!("Option<ObjectOrString<{}>>", object_name),
      // Object may be a string or number
      // [array]
      (false, true, false, false, false, false) => format!("Vec<{}>", object_name),
      // [array, null]
      (false, true, false, false, false, true) => format!("Option<Vec<{}>>", object_name),
      _ => {
        unreachable!(
          "Invalid schema types: object: {}, array: {}, string: {}, integer: {}, boolean: {}, null: {}",
          self.is_object, self.is_array, self.is_string, self.is_integer, self.is_boolean, self.is_null
        )
      }
    }
  }

  // pub fn is_only_null(&self) -> bool {
  //   self.is_null && !self.is_array && !self.is_object && !self.is_string && !self.is_integer
  // }

  pub fn merge(&mut self, other: &SchemaTypes) {
    self.types.extend(other.types.iter().cloned());
    self.is_null = self.types.contains(&SchemaType::Null);
    self.is_array = self.types.contains(&SchemaType::Array);
    self.is_object = self.types.contains(&SchemaType::Object);
    self.is_string = self.types.contains(&SchemaType::String);
    self.is_integer = self.types.contains(&SchemaType::Integer);
    self.is_boolean = self.types.contains(&SchemaType::Boolean);
  }
}

impl From<&SchemaTypeDefination> for SchemaTypes {
  fn from(value: &SchemaTypeDefination) -> Self {
    let types = match value {
      SchemaTypeDefination::String(type_) => {
        vec![SchemaType::try_from(type_).expect(format!("Unknown schema type: {}", type_).as_str())]
      }
      SchemaTypeDefination::Array(types) => types
        .iter()
        .map(|type_| {
          SchemaType::try_from(type_).expect(format!("Unknown schema type: {}", type_).as_str())
        })
        .collect(),
    };

    SchemaTypes::new(types)
  }
}

impl Default for SchemaTypes {
  fn default() -> Self {
    SchemaTypes {
      types: vec![],
      is_null: false,
      is_array: false,
      is_object: false,
      is_string: false,
      is_integer: false,
      is_boolean: false,
    }
  }
}
