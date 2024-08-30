use chrono::{DateTime, Utc};
use diesel::{Queryable, Selectable};
use serde::{Deserialize, Serialize};

use crate::schema::binance::spot::tradings::scalping::*;

#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = scalping)]
pub struct Scalping {
  pub id: String,
  pub symbol: String,
  pub scalping_id: f64,
  pub plan_id: f64,
  pub buy_price: f64,
  pub sell_price: f64,
  pub buy_quantity: i64,
  pub sell_quantity: i64,
  pub buy_order_id: f64,
  pub sell_order_id: i64,
  pub status: i32,
  pub version: i64,
  pub remark: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}