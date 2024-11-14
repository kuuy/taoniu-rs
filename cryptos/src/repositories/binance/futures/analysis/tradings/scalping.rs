use std::collections::HashMap;

use chrono::NaiveDate;
use diesel::prelude::*;

use crate::common::*;
use crate::schema::binance::futures::analysis::tradings::scalping::*;

#[derive(Default)]
pub struct ScalpingRepository {}

impl ScalpingRepository {
  pub async fn count(ctx: Ctx, conditions: &mut HashMap<&str, MixValue>) -> Result<i64, Box<dyn std::error::Error>> {
    let pool = ctx.pool.read().await;
    let mut conn = pool.get().unwrap();
    let mut query = scalping::table.into_boxed();
    if let Some(MixValue::Int(side)) = conditions.get("side") {
      query = query.filter(scalping::side.eq(side));
    }
    let result = query
      .count()
      .get_result(&mut conn)?;
    Ok(result)
  }

  pub async fn listings(ctx: Ctx, conditions: &mut HashMap<&str, MixValue>, current: i64, page_size: i64) -> Result<Vec<(String, i32, NaiveDate, i32, i32, f64, f64, f64, f64)>, Box<dyn std::error::Error>> {
    let pool = ctx.pool.read().await;
    let mut conn = pool.get().unwrap();
    let mut query = scalping::table.into_boxed();
    if let Some(MixValue::Int(side)) = conditions.get("side") {
      query = query.filter(scalping::side.eq(side));
    }
    let result = query
      .select((
        scalping::id,
        scalping::side,
        scalping::day,
        scalping::buys_count,
        scalping::sells_count,
        scalping::buys_amount,
        scalping::sells_amount,
        scalping::profit,
        scalping::additive_profit,
      ))
      .order(scalping::day.desc())
      .offset((current - 1) * page_size)
      .limit(page_size)
      .load::<(String, i32, NaiveDate, i32, i32, f64, f64, f64, f64)>(&mut conn)?;
    Ok(result)
  }
}
