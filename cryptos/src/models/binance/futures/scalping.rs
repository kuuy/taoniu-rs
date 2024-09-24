use chrono::{prelude::Utc, DateTime};
use diesel::{Queryable, Selectable};
use serde::{Deserialize, Serialize};

use crate::schema::binance::futures::scalping::*;

#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = scalping)]
pub struct Scalping {
  pub id: String,
  pub symbol: String,
  pub capital: f64,
  pub price: f64,
  pub take_price: f64,
  pub stop_price: f64,
  pub take_order_id: i64,
  pub stop_order_id: i64,
  pub profit: f64,
  pub timestamp: i64,
  pub status: i32,
  pub version: i64,
  pub remark: String,
  pub expired_at: DateTime<Utc>,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl Scalping {
  pub fn new(
    id: String,
    symbol: String,
    capital: f64,
    price: f64,
    take_price: f64,
    stop_price: f64,
    take_order_id: i64,
    stop_order_id: i64,
    profit: f64,
    timestamp: i64,
    status: i32,
    version: i64,
    remark: String,
    expired_at: DateTime<Utc>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
  ) -> Self {
    Self {
      id: id,
      symbol: symbol,
      capital: capital,
      price: price,
      take_price: take_price,
      stop_price: stop_price,
      take_order_id: take_order_id,
      stop_order_id: stop_order_id,
      profit: profit,
      timestamp: timestamp,
      status: status,
      version: version,
      remark: remark,
      expired_at: expired_at,
      created_at: created_at,
      updated_at: updated_at,
    }
  }
}