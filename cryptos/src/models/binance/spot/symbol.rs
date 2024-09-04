use chrono::{DateTime, Utc};
use diesel::{Queryable, Selectable};
use diesel_as_jsonb::AsJsonb;
use serde::{Deserialize, Serialize};

use crate::schema::binance::spot::symbols::*;

#[derive(Debug, Serialize, Deserialize, AsJsonb)]
pub struct Filters {
  pub price: String,
  pub quote: String,
}

#[derive(Debug, Serialize, Deserialize, AsJsonb)]
pub struct Depth {
  pub asks: Vec<Vec<String>>,
  pub bids: Vec<Vec<String>>,
  #[serde(alias = "lastUpdateId")]
  pub last_update_id: i64,
}

#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = symbols)]
pub struct Symbol {
  pub id: String,
  pub symbol: String,
  pub base_asset: String,
  pub quote_asset: String,
  pub filters: Filters,
  pub depth: Depth,
  pub is_spot: bool,
  pub is_margin: bool,
  pub status: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl Symbol {
  pub fn new(
    id: String,
    symbol: String,
    base_asset: String,
    quote_asset: String,
    filters: Filters,
    depth: Depth,
    is_spot: bool,
    is_margin: bool,
    status: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
  ) -> Self {
    Self {
      id: id,
      symbol: symbol,
      base_asset: base_asset,
      quote_asset: quote_asset,
      filters: filters,
      depth: depth,
      is_spot: is_spot,
      is_margin: is_margin,
      status: status,
      created_at: created_at,
      updated_at: updated_at,
    }
  }
}