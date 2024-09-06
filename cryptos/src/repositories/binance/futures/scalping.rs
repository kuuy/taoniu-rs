use diesel::prelude::*;

use crate::common::*;
use crate::models::binance::futures::scalping::*;
use crate::schema::binance::futures::scalping::*;

#[derive(Default)]
pub struct ScalpingRepository {}

impl ScalpingRepository {
  pub async fn scan(ctx: Ctx) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let pool = ctx.pool.read().await;
    let mut conn = pool.get().unwrap();
    let symbols = scalping::table
      .filter(scalping::status.eq_any([1, 2]))
      .select(scalping::symbol)
      .load::<String>(&mut conn)?;
    Ok(symbols)
  }
}
