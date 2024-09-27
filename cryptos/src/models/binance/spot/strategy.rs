use chrono::{prelude::Utc, DateTime};
use diesel::{Queryable, Selectable, Insertable};
use serde::{Deserialize, Serialize};

use crate::schema::binance::spot::strategies::*;

#[derive(Queryable, Selectable, Insertable, Deserialize, Serialize, Debug)]
#[diesel(table_name = strategies)]
pub struct Strategy {
  pub id: String,
  pub symbol: String,
  pub indicator: String,
  pub interval: String,
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
    indicator: String,
    interval: String,
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
      indicator: indicator,
      interval: interval,
      price: price,
      signal: signal,
      timestamp: timestamp,
      remark: remark,
      created_at: created_at,
      updated_at: updated_at,
    }
  }
}