use chrono::{DateTime, Utc};
use diesel::{Queryable, Selectable, Insertable};
use diesel_as_jsonb::AsJsonb;
use serde::{Deserialize, Serialize};

use crate::schema::binance::futures::strategies::*;

#[derive(Debug, Serialize, Deserialize, AsJsonb)]
pub struct Context {}

#[derive(Queryable, Selectable, Insertable, Deserialize, Serialize, Debug)]
#[diesel(table_name = strategies)]
pub struct Strategy {
  pub id: String,
  pub symbol: String,
  pub interval: String,
  pub indicator: String,
  pub price: f64,
  pub signal: i32,
  pub timestamp: i64,
  pub remark: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl Strategy {
  pub fn new(
    id: String,
    symbol: String,
    interval: String,
    indicator: String,
    price: f64,
    signal: i32,
    timestamp: i64,
    remark: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
  ) -> Self {
    Self {
      id: id,
      symbol: symbol,
      interval: interval,
      indicator: indicator,
      price: price,
      signal: signal,
      timestamp: timestamp,
      remark: remark,
      created_at: created_at,
      updated_at: updated_at,
    }
  }
}