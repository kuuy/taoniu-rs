// use chrono::{DateTime, TimeZone};
use diesel::{Queryable, Selectable};
use serde::{Deserialize, Serialize};

diesel::table! {
  #[sql_name = "binance_spot_klines"]
  schema (id) {
    id -> Varchar,
    symbol -> Varchar,
  }
}

#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = schema)]
pub struct Kline {
  pub id: String,
  pub symbol: String,
}