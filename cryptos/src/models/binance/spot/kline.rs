use chrono::{DateTime, Utc};
use diesel::{Queryable, Selectable, Insertable};
use serde::{Deserialize, Serialize};

use crate::schema::binance::spot::klines::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = klines)]
pub struct Kline {
  pub id: String,
  pub symbol: String,
  pub interval: String,
  pub open: f64,
  pub close: f64,
  pub high: f64,
  pub low: f64,
  pub volume: f64,
  pub quota: f64,
  pub timestamp: i64,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}
