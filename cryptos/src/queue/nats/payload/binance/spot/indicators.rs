use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct IndicatorsUpdatePayload<T> {
  pub symbol: T,
  pub interval: T,
}

impl<T> IndicatorsUpdatePayload<T>
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