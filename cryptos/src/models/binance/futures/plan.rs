use chrono::{prelude::Utc, DateTime};
use diesel::{Queryable, Selectable, Insertable};
use serde::{Deserialize, Serialize};

use crate::schema::binance::futures::plans::*;

#[derive(Queryable, Selectable, Insertable, Deserialize, Serialize, Debug)]
#[diesel(table_name = plans)]
pub struct Plan {
  pub id: String,
  pub symbol: String,
  pub interval: String,
  pub side: i32,
  pub price: f64,
  pub quantity: f64,
  pub amount: f64,
  pub timestamp: i64,
  pub status: i32,
  pub remark: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl Plan {
  pub fn new(
    id: String,
    symbol: String,
    interval: String,
    side: i32,
    price: f64,
    quantity: f64,
    amount: f64,
    timestamp: i64,
    status: i32,
    remark: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
  ) -> Self {
    Self {
      id: id,
      symbol: symbol,
      interval: interval,
      side: side,
      price: price,
      quantity: quantity,
      amount: amount,
      timestamp: timestamp,
      status: status,
      remark: remark,
      created_at: created_at,
      updated_at: updated_at,
    }
  }
}