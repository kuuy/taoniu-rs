use serde::{Deserialize, Serialize};

diesel::table! {
  symbol (id) {
    id -> Varchar,
    symbol -> Varchar,
    base_asset -> VarChar,
    quote_asset -> VarChar,
    filters -> Jsonb,
    depth -> Jsonb,
    is_spot -> Bool,
    is_margin -> Bool,
    status -> VarChar,
    created_at -> Timestamptz,
    updated_at -> Timestamptz,
  }
}

#[derive(Serialize, Deserialize, Debug)]
#[table_name = "symbols"]
pub struct symbol {
  pub id: String,
  pub name: String,
  pub active: bool,
}