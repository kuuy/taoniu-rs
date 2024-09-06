use std::ops::Sub;
use std::time::Duration;

use diesel::prelude::*;
use diesel::query_builder::QueryFragment;
use diesel::ExpressionMethods;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use chrono::{prelude::Utc, Timelike};

use crate::common::*;
use crate::models::binance::futures::position::*;
use crate::schema::binance::futures::positions::*;

#[derive(Default)]
pub struct PositionsRepository {}

impl PositionsRepository
{
  pub async fn get<T>(
    ctx: Ctx,
    symbol: T,
  ) -> Result<Option<Position>, Box<dyn std::error::Error>>
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();

    let pool = ctx.pool.read().await;
    let mut conn = pool.get().unwrap();

    match positions::table
      .select(Position::as_select())
      .filter(positions::symbol.eq(symbol))
      .first(&mut conn) {
      Ok(Position) => Ok(Some(Position)),
      Err(diesel::result::Error::NotFound) => Ok(None),
      Err(e) => Err(e.into()),
    }
  }

  pub async fn create(
    ctx: Ctx,
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
  ) -> Result<bool, Box<dyn std::error::Error>> {
    let pool = ctx.pool.write().await;
    let mut conn = pool.get().unwrap();

    let now = Utc::now();
    let Position = Position::new(
      id,
      symbol,
      side,
      leverage,
      capital,
      notional,
      entry_price,
      entry_quantity,
      timestamp,
      status,
      0,
      now,
      now,
    );
    match diesel::insert_into(positions::table)
      .values(&Position)
      .execute(&mut conn) {
      Ok(effective_rows) => Ok(effective_rows > 0),
      Err(e) => Err(e.into()),
    }
  }

  pub async fn update<V>(
    ctx: Ctx,
    id: String,
    value: V,
  ) -> Result<bool, Box<dyn std::error::Error>> 
  where
    V: diesel::AsChangeset<Target = positions::table>,
    <V as diesel::AsChangeset>::Changeset: QueryFragment<diesel::pg::Pg>,
  {
    let pool = ctx.pool.write().await;
    let mut conn = pool.get().unwrap();
    match diesel::update(positions::table.find(id)).set(value).execute(&mut conn) {
      Ok(effective_rows) => Ok(effective_rows > 0),
      Err(e) => Err(e.into()),
    }
  }
}
