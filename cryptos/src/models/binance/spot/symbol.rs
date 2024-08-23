use chrono::{DateTime, TimeZone};
use diesel::{Queryable, Selectable};
use serde::{Deserialize, Serialize};

diesel::table! {
  #[sql_name = "binance_spot_symbols"]
  symbols (id) {
    id -> Varchar,
    symbol -> Varchar,
    // base_asset -> VarChar,
    // quote_asset -> VarChar,
    // filters -> Jsonb,
    // depth -> Jsonb,
    is_spot -> Bool,
    is_margin -> Bool,
    status -> VarChar,
    // created_at -> Timestamptz,
    // updated_at -> Timestamptz,
  }
}

#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = symbols)]
pub struct Symbol {
  pub id: String,
  pub symbol: String,
  //<Tz: TimeZone>
  // #[diesel(sql_type = VarChar)]
  // pub base_asset: String,
  // #[diesel(sql_type = VarChar)]
  // pub quote_asset: String,
  // pub filters: Jsonb,
  pub is_spot: bool,
  pub is_margin: bool,
  // #[diesel(sql_type = VarChar)]
  pub status: String,
  // #[diesel(sql_type = Timestamptz)]
  // pub created_at: DateTime<Tz>,
  // #[diesel(sql_type = Timestamptz)]
  // pub updated_at: DateTime<Tz>,
}