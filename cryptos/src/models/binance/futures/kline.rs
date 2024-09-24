use chrono::{prelude::Utc, DateTime};
use diesel::{Queryable, Selectable, Insertable};
use serde::{Deserialize, Serialize};

use crate::schema::binance::futures::klines::*;

#[derive(Queryable, Selectable, Insertable, Deserialize, Serialize, Debug)]
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

impl Kline {
  pub fn new(
    id: String,
    symbol: String,
    interval: String,
    open: f64,
    close: f64,
    high: f64,
    low: f64,
    volume: f64,
    quota: f64,
    timestamp: i64,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
  ) -> Self {
    Self {
      id: id,
      symbol: symbol,
      interval: interval,
      open: open,
      close: close,
      high: high,
      low: low,
      volume: volume,
      quota: quota,
      timestamp: timestamp,
      created_at: created_at,
      updated_at: updated_at,
    }
  }
}