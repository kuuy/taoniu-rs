use diesel::prelude::*;

use crate::common::*;
use crate::schema::binance::spot::symbols::*;
use crate::models::binance::spot::symbol::*;

pub struct SymbolsRepository {}

impl SymbolsRepository {
  pub fn flush(ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    println!("symbols flush");
    if 1 > 0 {
      return Err(Box::from("symbols repository flush failed"))
    }
    Ok(())
  }

  pub async fn count(ctx: Ctx) -> Result<i64, Box<dyn std::error::Error>> {
    let pool = ctx.pool.read().await;
    let mut conn = pool.get().unwrap();
    let count = symbols::table
      .filter(symbols::status.eq("TRADING"))
      .filter(symbols::is_spot.eq(true))
      .count()
      .get_result(&mut conn)?;
    Ok(count)
  }
}
