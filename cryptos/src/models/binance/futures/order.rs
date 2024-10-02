use chrono::{prelude::Utc, DateTime};
use diesel::{Queryable, Selectable, Insertable};
use serde::{Deserialize, Serialize};

use crate::schema::binance::futures::orders::*;

#[derive(Queryable, Selectable, Insertable, Deserialize, Serialize, Debug)]
#[diesel(table_name = orders)]
pub struct Order {
  pub id: String,
  pub symbol: String,
  pub order_id: i64,
  pub order_type: String,
  pub position_side: String,
  pub side: String,
  pub price: f64,
  pub avg_price: f64,
  pub activate_price: f64,
  pub stop_price: f64,
  pub price_rate: f64,
  pub quantity: f64,
  pub executed_quantity: f64,
  pub open_time: i64,
  pub update_time: i64,
  pub working_type: String,
  pub price_protect: bool,
  pub reduce_only: bool,
  pub close_position: bool,
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
    position_side: String,
    side: String,
    price: f64,
    avg_price: f64,
    stop_price: f64,
    quantity: f64,
    executed_quantity: f64,
    open_time: i64,
    update_time: i64,
    working_type: String,
    price_protect: bool,
    reduce_only: bool,
    close_position: bool,
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
      position_side: position_side,
      side: side,
      price: price,
      avg_price: avg_price,
      activate_price: 0.0,
      stop_price: stop_price,
      price_rate: 0.0,
      quantity: quantity,
      executed_quantity: executed_quantity,
      open_time: open_time,
      update_time: update_time,
      working_type: working_type,
      price_protect: price_protect,
      reduce_only: reduce_only,
      close_position: close_position,
      status: status,
      remark: remark,
      created_at: created_at,
      updated_at: updated_at,
    }
  }
}