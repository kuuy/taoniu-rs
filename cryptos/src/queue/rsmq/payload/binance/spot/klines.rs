use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct KlinesSyncPayload<T> {
  pub interval: T,
}

impl<T> KlinesSyncPayload<T>
where
  T: AsRef<str>
{
  pub fn new(interval: T) -> Self {
    Self {
      interval: interval,
    }
  }
}