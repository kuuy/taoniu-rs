use chrono::{DateTime, Utc};
use diesel::{Queryable, Selectable, Insertable};
use serde::{Deserialize, Serialize};

use crate::schema::binance::futures::positions::*;

#[derive(Queryable, Selectable, Insertable, Deserialize, Serialize, Debug)]
#[diesel(table_name = positions)]
pub struct Position {
  pub id: String,
  pub symbol: String,
  pub side: i32,
  pub leverage: i32,
  pub capital: f64,
  pub notional: f64,
  pub entry_price: f64,
  pub entry_quantity: f64,
  pub timestamp: i64,
  pub status: i32,
  pub version: i64,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl Position {
  pub fn new(
    id: String,
    symbol: String,
    side: i32,
    leverage: i32,
    capital: f64,
    notional: f64,
    entry_price: f64,
    entry_quantity: f64,
    timestamp: i64,
    status: i32,
    version: i64,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
  ) -> Self {
    Self {
      id: id,
      symbol: symbol,
      side: side,
      leverage: leverage,
      capital: capital,
      notional: notional,
      entry_price: entry_price,
      entry_quantity: entry_quantity,
      timestamp: timestamp,
      status: status,
      version: version,
      created_at: created_at,
      updated_at: updated_at,
    }
  }
}