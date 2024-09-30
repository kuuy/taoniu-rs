use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PlansUpdatePayload<T> {
  pub id: T,
  pub side: i32,
  pub amount: f64,
}

impl<T> PlansUpdatePayload<T>
where
  T: AsRef<str>
{
  pub fn new(id: T, side: i32, amount: f64) -> Self {
    Self {
      id: id,
      side: side,
      amount: amount,
    }
  }
}