use std::collections::HashMap;

use diesel::prelude::*;

use crate::common::*;
use crate::schema::binance::spot::triggers::*;

#[derive(Default)]
pub struct TriggersRepository {}

impl TriggersRepository {
  pub async fn count(ctx: Ctx, conditions: &mut HashMap<&str, MixValue>) -> Result<i64, Box<dyn std::error::Error>> {
    let pool = ctx.pool.read().await;
    let mut conn = pool.get().unwrap();
    let mut query = triggers::table.into_boxed();
    if let Some(MixValue::String(symbol)) = conditions.get("symbol") {
      query = query.filter(triggers::symbol.eq(&symbol[..]));
    }
    let result = query
      .count()
      .get_result(&mut conn)?;
    Ok(result)
  }

  pub async fn listings(ctx: Ctx, conditions: &mut HashMap<&str, MixValue>, current: i64, page_size: i64) -> Result<Vec<(String, String, f64, f64, f64, f64, i64, i64, f64, i64, i32)>, Box<dyn std::error::Error>> {
    let pool = ctx.pool.read().await;
    let mut conn = pool.get().unwrap();
    let mut query = triggers::table.into_boxed();
    if let Some(MixValue::String(symbol)) = conditions.get("symbol") {
      query = query.filter(triggers::symbol.eq(&symbol[..]));
    }
    let result = query
      .select((
        triggers::id,
        triggers::symbol,
        triggers::capital,
        triggers::price,
        triggers::take_price,
        triggers::stop_price,
        triggers::take_order_id,
        triggers::stop_order_id,
        triggers::profit,
        triggers::timestamp,
        triggers::status,
      ))
      .order(triggers::timestamp.desc())
      .offset((current - 1) * page_size)
      .limit(page_size)
      .load::<(String, String, f64, f64, f64, f64, i64, i64, f64, i64, i32)>(&mut conn)?;
    Ok(result)
  }

  pub async fn scan(ctx: Ctx) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let pool = ctx.pool.read().await;
    let mut conn = pool.get().unwrap();
    let symbols = triggers::table
      .filter(triggers::status.eq_any([1, 2]))
      .select(triggers::symbol)
      .load::<String>(&mut conn)?;
    Ok(symbols)
  }
}
