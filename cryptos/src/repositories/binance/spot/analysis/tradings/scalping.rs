use chrono::NaiveDate;
use diesel::prelude::*;

use crate::common::*;
use crate::schema::binance::spot::analysis::tradings::scalping::*;

#[derive(Default)]
pub struct ScalpingRepository {}

impl ScalpingRepository {
  pub async fn count(ctx: Ctx) -> Result<i64, Box<dyn std::error::Error>> {
    let pool = ctx.pool.read().await;
    let mut conn = pool.get().unwrap();
    let count = scalping::table
      .count()
      .get_result(&mut conn)?;
    Ok(count)
  }

  pub async fn listings(ctx: Ctx, current: i64, page_size: i64) -> Result<Vec<(String, NaiveDate, i32, i32, f64, f64, f64, f64)>, Box<dyn std::error::Error>> {
    let pool = ctx.pool.read().await;
    let mut conn = pool.get().unwrap();
    let analysis = scalping::table
      .select((
        scalping::id,
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
      .load::<(String, NaiveDate, i32, i32, f64, f64, f64, f64)>(&mut conn)?;
    Ok(analysis)
  }
}
