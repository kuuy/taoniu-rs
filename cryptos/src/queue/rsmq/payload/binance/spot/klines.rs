use serde::{Deserialize, Deserializer};

struct KlinesUpdatePayload {
  symbol: String,
  interval: String,
}