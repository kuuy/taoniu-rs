use std::collections::HashMap;

use diesel::prelude::*;

use crate::common::*;
use crate::models::binance::futures::scalping::*;
use crate::schema::binance::futures::scalping::*;

pub mod plans;

#[derive(Default)]
pub struct ScalpingRepository {}

impl ScalpingRepository {
  pub async fn get<T>(
    ctx: Ctx,
    symbol: T,
    side: i32,
  ) -> Result<Option<Scalping>, Box<dyn std::error::Error>>
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();

    let pool = ctx.pool.read().await;
    let mut conn = pool.get().unwrap();

    match scalping::table
      .select(Scalping::as_select())
      .filter(scalping::symbol.eq(symbol))
      .filter(scalping::side.eq(side))
      .filter(scalping::status.eq(1))
      .first(&mut conn) {
        Ok(result) => Ok(Some(result)),
        Err(diesel::result::Error::NotFound) => Ok(None),
        Err(err) => Err(err.into()),
      }
  }

  pub async fn count(ctx: Ctx, conditions: &mut HashMap<&str, MixValue>) -> Result<i64, Box<dyn std::error::Error>> {
    let pool = ctx.pool.read().await;
    let mut conn = pool.get().unwrap();
    let mut query = scalping::table.into_boxed();
    if let Some(MixValue::String(symbol)) = conditions.get("symbol") {
      query = query.filter(scalping::symbol.eq(&symbol[..]));
    }
    let count = query
      .count()
      .get_result(&mut conn)?;
    Ok(count)
  }

  pub async fn listings(ctx: Ctx, conditions: &mut HashMap<&str, MixValue>, current: i64, page_size: i64) -> Result<Vec<(String, String, i32, f64, f64, f64, f64, i64, i64, f64, i64, i32)>, Box<dyn std::error::Error>> {
    let pool = ctx.pool.read().await;
    let mut conn = pool.get().unwrap();
    let mut query = scalping::table.into_boxed();
    if let Some(MixValue::String(symbol)) = conditions.get("symbol") {
      query = query.filter(scalping::symbol.eq(&symbol[..]));
    }
    let scalping = query
      .select((
        scalping::id,
        scalping::symbol,
        scalping::side,
        scalping::capital,
        scalping::price,
        scalping::take_price,
        scalping::stop_price,
        scalping::take_order_id,
        scalping::stop_order_id,
        scalping::profit,
        scalping::timestamp,
        scalping::status,
      ))
      .order(scalping::timestamp.desc())
      .offset((current - 1) * page_size)
      .limit(page_size)
      .load::<(String, String, i32, f64, f64, f64, f64, i64, i64, f64, i64, i32)>(&mut conn)?;
    Ok(scalping)
  }

  pub async fn scan(ctx: Ctx, side: i32) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let pool = ctx.pool.read().await;
    let mut conn = pool.get().unwrap();
    let symbols = scalping::table
      .filter(scalping::side.eq(side))
      .filter(scalping::status.eq_any([1, 2]))
      .select(scalping::symbol)
      .load::<String>(&mut conn)?;
    Ok(symbols)
  }
}
