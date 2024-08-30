use chrono::{DateTime, Utc};
use diesel::{Queryable, Selectable};
use serde::{Deserialize, Serialize};

use crate::schema::binance::spot::scalping::*;

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