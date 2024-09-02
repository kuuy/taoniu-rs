use serde::{Deserialize, Deserializer, Serialize};

#[derive(Deserialize, Serialize)]
pub struct KlinesFlushPayload<T> {
  pub interval: T,
}

impl<T> KlinesFlushPayload<T>
where
  T: AsRef<str>
{
  pub fn new(interval: T) -> Self {
    Self {
      interval: interval,
    }
  }
}

#[derive(Deserialize, Serialize)]
pub struct KlinesUpdatePayload {
  symbol: String,
  interval: String,
}