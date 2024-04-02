pub mod api;
pub mod api_tree;
pub mod structs;

use std::sync::Arc;
use structs::{Enum, Referable, Struct};

#[derive(Clone, Debug)]
pub enum ParsedStruct {
  Struct(Struct),
  Enum(Enum),
  Type {
    type_name: String,
    reference: Option<Arc<Box<dyn Referable>>>,
  },
}
