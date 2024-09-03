use chrono::{DateTime, Utc};
use diesel::{Queryable, Selectable, Insertable};
use diesel_as_jsonb::AsJsonb;
use serde::{Deserialize, Serialize};

use crate::schema::binance::futures::plans::*;

#[derive(Debug, Serialize, Deserialize, AsJsonb)]
pub struct Context {}

#[derive(Queryable, Selectable, Insertable, Deserialize, Serialize, Debug)]
#[diesel(table_name = plans)]
pub struct Plan {
  pub id: String,
  pub symbol: String,
  pub side: i32,
  pub price: f64,
  pub quantity: f64,
  pub amount: f64,
  pub timestamp: i64,
  pub context: Context,
  pub status: i32,
  pub remark: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl Plan {
  pub fn new(
    id: String,
    symbol: String,
    side: i32,
    price: f64,
    quantity: f64,
    amount: f64,
    timestamp: i64,
    context: Context,
    status: i32,
    remark: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
  ) -> Self {
    Self {
      id: id,
      symbol: symbol,
      side: side,
      price: price,
      quantity: quantity,
      amount: amount,
      timestamp: timestamp,
      context: context,
      status: status,
      remark: remark,
      created_at: created_at,
      updated_at: updated_at,
    }
  }
}