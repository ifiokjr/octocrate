use super::schema::Schema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Response {
  #[serde(rename = "statusCode")]
  status_code: String,
  schema: Option<Schema>,
}
