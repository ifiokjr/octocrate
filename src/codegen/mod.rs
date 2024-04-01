mod api;
mod body_parameters;
// mod file_tree;
mod function_name;
mod parameters;
mod schema_parser;
mod schema_types;
mod structs;

use std::collections::HashMap;

pub struct ParseContext {
  parsed_structs: HashMap<String, structs::Struct>,
}

pub trait Parser {
  fn parse(&self, context: &mut ParseContext);
}

// use crate::schema::{api::APISchema, schema::Schema};
// use std::collections::HashMap;

// struct Category(String, String);

// pub struct Codegen {
//   structs: HashMap<String, Schema>,
//   apis: HashMap<Category, APISchema>,
// }
