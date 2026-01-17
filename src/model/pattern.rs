use serde::{Deserialize, Serialize};

/// Pattern struct
#[derive(Debug, Deserialize, Serialize)]
pub struct Pattern {
  pub multipliers: Vec<f64>,
}