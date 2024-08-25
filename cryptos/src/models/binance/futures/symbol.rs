use chrono::{DateTime, TimeZone};
use diesel::{Queryable, Selectable};
use serde::{Deserialize, Serialize};

diesel::table! {
  #[sql_name = "binance_futures_symbols"]
  schema (id) {
    id -> Varchar,
    symbol -> Varchar,
    status -> VarChar,
  }
}

#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = schema)]
pub struct Symbol {
  pub id: String,
  pub symbol: String,
  pub status: String,
}