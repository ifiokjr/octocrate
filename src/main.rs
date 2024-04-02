mod codegen;
mod schema;

use codegen::Codegen;
use schema::Source;

fn main() {
  let json = include_str!("../resources/schema.json");

  let source: Source = serde_json::from_str(json).unwrap();

  let mut codegen = Codegen::new();

  codegen.parse(source);

  codegen.generate();
}
