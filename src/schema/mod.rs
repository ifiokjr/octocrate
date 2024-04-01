pub mod api;
pub mod body_parameters;
pub mod parameters;
pub mod response;
pub mod schema;

use std::collections::HashMap;

pub type Source = HashMap<String, HashMap<String, Vec<api::APISchema>>>;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_source() {
    let json = include_str!("../../resources/schema.json");

    let source: Source = serde_json::from_str(json).unwrap();

    let commits = source.get("commits").unwrap();

    let commits = commits.get("commits").unwrap();
  }
}
