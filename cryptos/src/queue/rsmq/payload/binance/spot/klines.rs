use serde::{Deserialize, Serialize};

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
