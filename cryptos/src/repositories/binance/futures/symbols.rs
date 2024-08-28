use diesel::prelude::*;

use crate::common::*;
use crate::models::binance::futures::symbol::schema::dsl::*;

#[derive(Default)]
pub struct SymbolsRepository {}

impl SymbolsRepository {
  pub fn flush(&self, _: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    println!("symbols flush");
    if 1 > 0 {
      return Err(Box::from("symbols repository flush failed"))
    }
    Ok(())
  }

  pub async fn count(&self, ctx: Ctx) -> Result<i64, Box<dyn std::error::Error>> {
    let pool = ctx.pool.read().unwrap();
    let mut conn = pool.get().unwrap();
    let count = schema
      .filter(status.eq("TRADING"))
      .count()
      .get_result(&mut conn)?;
    Ok(count)
  }
}
