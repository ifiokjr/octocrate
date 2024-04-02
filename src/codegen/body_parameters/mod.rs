mod generated_struct;

use std::ops::Deref;

use super::{
  function_name::FunctionName,
  parsed::structs::{Struct, StructField},
};
use crate::schema::body_parameters::BodyParameter;
use generated_struct::GeneratedStruct;

#[derive(Debug, Clone)]
pub struct BodyParameters {
  pub body_parameters: Vec<BodyParameter>,
}

impl BodyParameters {
  pub fn new(body_parameters: Vec<BodyParameter>) -> Self {
    Self { body_parameters }
  }

  pub fn parse(&self, title: &String) -> Option<Struct> {
    if self.body_parameters.is_empty() {
      return None;
    }

    let description = format!("Body parameters for {}", title.to_lowercase());

    let title = FunctionName::from(title);
    let struct_name = format!("{}_request", title.deref());
    let mut new_struct = Struct::new_with_description(struct_name, description);

    for body_parameter in &self.body_parameters {
      let generated_struct = self.parse_body_parameter(body_parameter, &String::new());

      match &generated_struct {
        GeneratedStruct::Type(_) => {
          let field = StructField::new_with_description(
            body_parameter.name.clone(),
            generated_struct.to_full_type(),
            body_parameter.description.clone(),
          );

          new_struct.add_field(field);
        }
        GeneratedStruct::Struct {
          struct_: child_struct,
          ..
        } => {
          let mut field = StructField::new_with_description(
            body_parameter.name.clone(),
            generated_struct.to_full_type(),
            body_parameter.description.clone(),
          );

          field.reference(child_struct.clone());

          new_struct.add_field(field);
        }
      }
    }

    Some(new_struct)
  }

  fn parse_body_parameter(
    &self,
    parameter: &BodyParameter,
    name_prefix: &String,
  ) -> GeneratedStruct {
    let struct_name = if name_prefix.is_empty() {
      parameter.name.clone()
    } else {
      format!("{}_{}", name_prefix, parameter.name)
    };

    match &parameter.child_params_group {
      Some(child_params_group) => {
        let mut new_struct =
          Struct::new_with_description(struct_name.clone(), parameter.description.clone());

        for child_param in child_params_group {
          let generated_struct = self.parse_body_parameter(child_param, &struct_name);

          match &generated_struct {
            GeneratedStruct::Type(_) => {
              let field = StructField::new_with_description(
                child_param.name.clone(),
                generated_struct.to_full_type(),
                child_param.description.clone(),
              );

              new_struct.add_field(field);
            }
            GeneratedStruct::Struct {
              struct_: child_struct,
              ..
            } => {
              let mut field = StructField::new_with_description(
                child_param.name.clone(),
                generated_struct.to_full_type(),
                child_param.description.clone(),
              );

              field.reference(child_struct.clone());

              new_struct.add_field(field);
            }
          }
        }

        GeneratedStruct::struct_(new_struct, parameter.type_.clone())
      }
      None => GeneratedStruct::type_(parameter.type_.clone()),
    }
  }
}

// #[cfg(test)]
// mod codegen_body_parameters_tests {
//   use super::*;

//   #[test]
//   fn test_body_parameters() {
//     let json = r#"[
//       {
//         "type": "object or null",
//         "name": "required_status_checks",
//         "in": "body",
//         "description": "<p>Require status checks to pass before merging. Set to <code>null</code> to disable.</p>",
//         "isRequired": true,
//         "childParamsGroups": [
//           {
//             "type": "boolean",
//             "name": "strict",
//             "description": "<p>Require branches to be up to date before merging.</p>",
//             "isRequired": true
//           },
//           {
//             "type": "array of strings",
//             "name": "contexts",
//             "description": "<p><strong>Deprecated</strong>: The list of status checks to require in order to merge into this branch. If any of these checks have recently been set by a particular GitHub App, they will be required to come from that app in future for the branch to merge. Use <code>checks</code> instead of <code>contexts</code> for more fine-grained control.</p>",
//             "isRequired": true
//           },
//           {
//             "type": "array of objects",
//             "name": "checks",
//             "description": "<p>The list of status checks to require in order to merge into this branch.</p>",
//             "childParamsGroups": [
//               {
//                 "type": "string",
//                 "name": "context",
//                 "description": "<p>The name of the required check</p>",
//                 "isRequired": true
//               },
//               {
//                 "type": "integer",
//                 "name": "app_id",
//                 "description": "<p>The ID of the GitHub App that must provide this check. Omit this field to automatically select the GitHub App that has recently provided this check, or any app if it was not set by a GitHub App. Pass -1 to explicitly allow any app to set the status.</p>"
//               }
//             ]
//           }
//         ]
//       },
//       {
//         "type": "boolean or null",
//         "name": "enforce_admins",
//         "in": "body",
//         "description": "<p>Enforce all configured restrictions for administrators. Set to <code>true</code> to enforce required status checks for repository administrators. Set to <code>null</code> to disable.</p>",
//         "isRequired": true
//       }
//     ]"#;

//     let body_parameters: Vec<BodyParameter> = serde_json::from_str(json).unwrap();

//     let body_parameters = BodyParameters::new(body_parameters);

//     let generated_struct = body_parameters
//       .generate_struct(&String::from("Create an artifact"))
//       .unwrap();

//     assert_eq!(generated_struct.name.to_string(), "CreateArtifactRequest");
//     assert_eq!(
//       generated_struct.description.as_ref().unwrap().to_string(),
//       "Body parameters for create an artifact"
//     );
//     assert_eq!(generated_struct.fields.len(), 2);

//     let required_status_checks = &generated_struct.fields[0];

//     assert_eq!(required_status_checks.name, "required_status_checks");
//     assert_eq!(
//       required_status_checks.type_name,
//       "Option<RequiredStatusChecks>"
//     );

//     let required_status_checks = required_status_checks.reference.as_ref().unwrap();

//     assert_eq!(
//       required_status_checks.name.to_string(),
//       "RequiredStatusChecks"
//     );
//     assert_eq!(required_status_checks.fields.len(), 3);

//     let strict = &required_status_checks.fields[0];

//     assert_eq!(strict.name, "strict");
//     assert_eq!(strict.type_name, "bool");
//     assert_eq!(
//       strict.description.as_ref().unwrap().to_string(),
//       "Require branches to be up to date before merging."
//     );

//     let contexts = &required_status_checks.fields[1];

//     assert_eq!(contexts.name, "contexts");
//     assert_eq!(contexts.type_name, "Vec<String>");

//     let checks = &required_status_checks.fields[2];

//     assert_eq!(checks.name, "checks");
//     assert_eq!(checks.type_name, "Vec<RequiredStatusChecksChecks>");

//     let checks = &checks.reference.as_ref().unwrap();

//     assert_eq!(checks.name.to_string(), "RequiredStatusChecksChecks");
//     assert_eq!(checks.fields.len(), 2);

//     let context = &checks.fields[0];

//     assert_eq!(context.name, "context");
//     assert_eq!(context.type_name, "String");

//     let app_id = &checks.fields[1];

//     assert_eq!(app_id.name, "app_id");
//     assert_eq!(app_id.type_name, "i64");

//     let enforce_admins = &generated_struct.fields[1];

//     assert_eq!(enforce_admins.name, "enforce_admins");
//     assert_eq!(enforce_admins.type_name, "Option<bool>");

//     let required_status_checks = &generated_struct.fields[0];
//     let required_status_checks = &required_status_checks.reference.as_ref().unwrap();

//     assert_eq!(
//       required_status_checks.name.to_string(),
//       "RequiredStatusChecks"
//     );
//     assert_eq!(required_status_checks.fields.len(), 3);

//     // println!("{:#?}", generated_struct);
//   }
// }
