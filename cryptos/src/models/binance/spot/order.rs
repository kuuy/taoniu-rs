use chrono::{DateTime, Utc};
use diesel::{Queryable, Selectable, Insertable};
use serde::{Deserialize, Serialize};

use crate::schema::binance::spot::orders::*;

#[derive(Queryable, Selectable, Insertable, Deserialize, Serialize, Debug)]
#[diesel(table_name = orders)]
pub struct Order {
  pub id: String,
  pub symbol: String,
  pub order_id: i64,
  pub order_type: String,
  pub side: String,
  pub price: f64,
  pub avg_price: f64,
  pub stop_price: f64,
  pub quantity: f64,
  pub executed_quantity: f64,
  pub open_time: i64,
  pub update_time: i64,
  pub status: String,
  pub remark: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl Order {
  pub fn new(
    id: String,
    symbol: String,
    order_id: i64,
    order_type: String,
    side: String,
    price: f64,
    avg_price: f64,
    stop_price: f64,
    quantity: f64,
    executed_quantity: f64,
    open_time: i64,
    update_time: i64,
    status: String,
    remark: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
  ) -> Self {
    Self {
      id: id,
      symbol: symbol,
      order_id: order_id,
      order_type: order_type,
      side: side,
      price: price,
      avg_price: avg_price,
      stop_price: stop_price,
      quantity: quantity,
      executed_quantity: executed_quantity,
      open_time: open_time,
      update_time: update_time,
      status: status,
      remark: remark,
      created_at: created_at,
      updated_at: updated_at,
    }
  }
}