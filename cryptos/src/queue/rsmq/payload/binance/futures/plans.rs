use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PlansFlushPayload<T> {
  pub symbol: T,
  pub interval: T,
}

impl<T> PlansFlushPayload<T>
where
  T: AsRef<str>
{
  pub fn new(symbol: T, interval: T) -> Self {
    Self {
      symbol: symbol,
      interval: interval,
    }
  }
}