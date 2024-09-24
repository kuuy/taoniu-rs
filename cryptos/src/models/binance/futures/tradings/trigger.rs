use chrono::{prelude::Utc, DateTime};
use diesel::{Queryable, Selectable};
use serde::{Deserialize, Serialize};

use crate::schema::binance::futures::tradings::triggers::*;

#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = triggers)]
pub struct Trigger {
  pub id: String,
  pub symbol: String,
  pub trigger_id: String,
  pub buy_price: f64,
  pub sell_price: f64,
  pub buy_quantity: f64,
  pub sell_quantity: f64,
  pub buy_order_id: i64,
  pub sell_order_id: i64,
  pub status: i32,
  pub version: i64,
  pub remark: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl Trigger {
  pub fn new(
    id: String,
    symbol: String,
    trigger_id: String,
    buy_price: f64,
    sell_price: f64,
    buy_quantity: f64,
    sell_quantity: f64,
    buy_order_id: i64,
    sell_order_id: i64,
    status: i32,
    version: i64,
    remark: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
  ) -> Self {
    Self {
      id: id,
      symbol: symbol,
      trigger_id: trigger_id,
      buy_price: buy_price,
      sell_price: sell_price,
      buy_quantity: buy_quantity,
      sell_quantity: sell_quantity,
      buy_order_id: buy_order_id,
      sell_order_id: sell_order_id,
      status: status,
      version: version,
      remark: remark,
      created_at: created_at,
      updated_at: updated_at,
    }
  }
}