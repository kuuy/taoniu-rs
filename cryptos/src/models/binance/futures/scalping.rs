// use chrono::{DateTime, TimeZone};
use diesel::{Queryable, Selectable};
use serde::{Deserialize, Serialize};

diesel::table! {
  #[sql_name = "binance_futures_scalping"]
  schema (id) {
    id -> Varchar,
    symbol -> Varchar,
    status -> Int8,
  }
}

#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = schema)]
pub struct Scalping {
  pub id: String,
  pub symbol: String,
  pub status: u32,
}