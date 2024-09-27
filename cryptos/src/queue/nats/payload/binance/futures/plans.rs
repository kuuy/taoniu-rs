use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PlansUpdatePayload<T> {
  pub id: T,
  pub amount: f64,
}

impl<T> PlansUpdatePayload<T>
where
  T: AsRef<str>
{
  pub fn new(id: T, amount: f64) -> Self {
    Self {
      id: id,
      amount: amount,
    }
  }
}