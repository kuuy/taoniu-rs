use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct KlinesFlushPayload<T> {
  pub symbol: T,
  pub interval: T,
  #[serde(rename = "endTime")]
  pub endtime: i64,
  pub limit: i64,
}

impl<T> KlinesFlushPayload<T>
where
  T: AsRef<str>
{
  pub fn new(symbol: T, interval: T, endtime: i64, limit: i64) -> Self {
    Self {
      symbol,
      interval,
      endtime,
      limit,
    }
  }
}