use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Reservoir {
  pub head_pattern: Option<Box<str>>,
}