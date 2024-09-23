use serde::{Serialize, Deserialize};

pub mod spot;
pub mod futures;
pub mod margin;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
	code: i64,
  #[serde(alias = "msg")]
	message: String,
}

impl std::fmt::Display for ApiError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "code: {} message: {}", self.code, self.message)
  }
}

impl std::error::Error for ApiError {}

pub struct GamblingPlan {
  pub take_price: f64,
  pub take_quantity: f64,
  pub take_amount: f64,
}