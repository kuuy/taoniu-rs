use serde::{Deserialize, Deserializer, Serialize};

#[derive(Deserialize, Serialize)]
pub struct KlinesUpdatePayload<T> {
  symbol: T,
  interval: T,
}

impl<T> KlinesUpdatePayload<T>
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