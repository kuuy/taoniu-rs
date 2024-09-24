use chrono::{prelude::Utc, DateTime};
use diesel::{Queryable, Selectable};
use serde::{Deserialize, Serialize};

use crate::schema::binance::spot::analysis::tradings::scalping::*;

#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = scalping)]
pub struct Scalping {
  pub id: String,
  pub day: String,
  pub buys_count: i64,
  pub sells_count: i64,
  pub buys_amount: f64,
  pub sells_amount: f64,
  pub profit: f64,
  pub additive_profit: f64,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl Scalping {
  pub fn new(
    id: String,
    day: String,
    buys_count: i64,
    sells_count: i64,
    buys_amount: f64,
    sells_amount: f64,
    profit: f64,
    additive_profit: f64,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
  ) -> Self {
    Self {
      id: id,
      day: day,
      buys_count: buys_count,
      sells_count: sells_count,
      buys_amount: buys_amount,
      sells_amount: sells_amount,
      profit: profit,
      additive_profit: additive_profit,
      created_at: created_at,
      updated_at: updated_at,
    }
  }
}