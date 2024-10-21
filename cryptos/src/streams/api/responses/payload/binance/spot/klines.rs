use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct KlinesFlushPayload {
  pub open: f64,
  pub close: f64,
  pub high: f64,
  pub low: f64,
  pub volume: f64,
  pub quota: f64,
  pub timestamp: i64,
}
