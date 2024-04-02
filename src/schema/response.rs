use super::schema::Schema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Response {
  #[serde(rename = "statusCode")]
  pub status_code: String,
  pub schema: Option<Schema>,
}
