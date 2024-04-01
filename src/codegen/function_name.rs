use regex::Regex;
use std::ops::Deref;

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionName(String);

impl From<&String> for FunctionName {
  fn from(value: &String) -> Self {
    let value = match value.find(" for ") {
      Some(index) => {
        let (left, _) = value.split_at(index);

        left.to_string()
      }
      None => value.clone(),
    };

    let regexp = r#"(a |an |the )"#;

    let re = Regex::new(regexp).unwrap();

    let value = re.replace_all(&value, "").to_string();

    let regexp = r#"\s+"#;

    let re = Regex::new(regexp).unwrap();

    let value = re.replace_all(value.trim(), "_").to_string();

    Self(value.to_lowercase())
  }
}

impl Deref for FunctionName {
  type Target = String;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_function_name() {
    let value = "a function name".to_string();

    let function_name = FunctionName::from(&value);

    assert_eq!(function_name, FunctionName("function_name".to_string()));

    let value = "List workflow run artifacts".to_string();

    let function_name = FunctionName::from(&value);

    assert_eq!(
      function_name,
      FunctionName("list_workflow_run_artifacts".to_string())
    );

    let value = "Get an artifact".to_string();

    let function_name = FunctionName::from(&value);

    assert_eq!(function_name, FunctionName("get_artifact".to_string()));

    let value = "List artifacts for a repository".to_string();

    let function_name = FunctionName::from(&value);

    assert_eq!(function_name, FunctionName("list_artifacts".to_string()));
  }
}
