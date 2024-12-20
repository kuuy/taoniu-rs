use chrono::NaiveDate;
use diesel::prelude::*;

use crate::common::*;
use crate::schema::binance::futures::analysis::tradings::triggers::*;

#[derive(Default)]
pub struct TriggersRepository {}

impl TriggersRepository {
  pub async fn count(ctx: Ctx) -> Result<i64, Box<dyn std::error::Error>> {
    let pool = ctx.pool.read().await;
    let mut conn = pool.get().unwrap();
    let count = triggers::table
      .count()
      .get_result(&mut conn)?;
    Ok(count)
  }

  pub async fn listings(ctx: Ctx, current: i64, page_size: i64) -> Result<Vec<(String, NaiveDate, i32, i32, f64, f64, f64, f64)>, Box<dyn std::error::Error>> {
    let pool = ctx.pool.read().await;
    let mut conn = pool.get().unwrap();
    let analysis = triggers::table
      .select((
        triggers::id,
        triggers::day,
        triggers::buys_count,
        triggers::sells_count,
        triggers::buys_amount,
        triggers::sells_amount,
        triggers::profit,
        triggers::additive_profit,
      ))
      .order(triggers::day.desc())
      .offset((current - 1) * page_size)
      .limit(page_size)
      .load::<(String, NaiveDate, i32, i32, f64, f64, f64, f64)>(&mut conn)?;
    Ok(analysis)
  }
}