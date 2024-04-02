use super::{api::ParsedAPIModule, Referable};
use std::{collections::HashMap, sync::Arc};

pub struct ParsedAPITree {
  pub global_types: HashMap<String, Arc<Box<dyn Referable>>>,
  pub modules: HashMap<String, HashMap<String, ParsedAPIModule>>,
}

impl ParsedAPITree {
  pub fn new() -> Self {
    ParsedAPITree {
      global_types: HashMap::new(),
      modules: HashMap::new(),
    }
  }

  pub fn add_global_type<T>(&mut self, name: &String, referable: T)
  where
    T: Referable + 'static,
  {
    self
      .global_types
      .insert(name.clone(), Arc::new(Box::new(referable)));
  }

  pub fn add_boxed_global_type(&mut self, name: String, referable: Arc<Box<dyn Referable>>) {
    self.global_types.insert(name, referable);
  }

  pub fn add_module(&mut self, category: String, module: ParsedAPIModule) {
    self
      .modules
      .entry(category)
      .or_insert_with(HashMap::new)
      .insert(module.name.clone(), module);
  }
}
