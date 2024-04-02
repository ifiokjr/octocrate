mod api;
mod body_parameters;
mod function_name;
mod parameters;
mod parsed;
mod schema_parser;
mod schema_types;

use crate::schema::Source;
use parsed::{
  api::{ParsedAPI, ParsedAPIModule},
  ParsedStruct,
};

pub struct ParseContext {
  api_tree: parsed::api_tree::ParsedAPITree,
}

// pub trait Parser {
//   fn parse(&self, context: &mut ParseContext);
// }

pub struct Codegen {
  context: ParseContext,
}

impl Codegen {
  pub fn new() -> Self {
    Codegen {
      context: ParseContext {
        api_tree: parsed::api_tree::ParsedAPITree::new(),
      },
    }
  }

  pub fn parse(&mut self, source: Source) {
    let start_time = std::time::Instant::now();
    let categories = source.keys();

    for category in categories {
      println!("Parsing category: {}", category);
      let subcategory = source.get(category).unwrap();

      let subcategories = subcategory.keys();

      for subcategory_name in subcategories {
        println!("Parsing subcategory: {}", subcategory_name);
        let apis = subcategory.get(subcategory_name).unwrap();
        let mut module = ParsedAPIModule::new(subcategory_name.clone());

        for api in apis {
          let parsed_api = ParsedAPI::from(api.clone());

          if let Some(body) = &parsed_api.body {
            module.add_type(body.clone());
          }

          if let Some(query) = &parsed_api.query {
            module.add_type(query.clone());
          }

          if let Some(response) = &parsed_api.response {
            match response {
              ParsedStruct::Enum(enum_) => {
                self
                  .context
                  .api_tree
                  .add_global_type(&enum_.name, enum_.clone());
              }
              ParsedStruct::Struct(struct_) => {
                self
                  .context
                  .api_tree
                  .add_global_type(&struct_.name, struct_.clone());
              }
              ParsedStruct::Type {
                type_name,
                reference,
              } => {
                if let Some(referable) = reference {
                  self
                    .context
                    .api_tree
                    .add_boxed_global_type(type_name.clone(), referable.clone());
                }
              }
            }
          }

          module.add_api(parsed_api);
        }

        self.context.api_tree.add_module(category.clone(), module);
      }
    }

    println!("Parsing took: {:?}", start_time.elapsed());
  }

  pub fn generate(&self) {
    unimplemented!("Codegen::generate")
  }
}
