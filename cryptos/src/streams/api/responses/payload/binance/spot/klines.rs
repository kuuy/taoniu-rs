use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct KlinesFlushPayload<T>
where
  T: AsRef<str>
{
  pub result: Vec<(i64, T, T, T, T, T, i64, T, u64, T, T, T)>,
}
