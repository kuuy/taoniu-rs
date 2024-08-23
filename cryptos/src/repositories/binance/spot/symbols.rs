use redis::aio::MultiplexedConnection;
use diesel::prelude::*;

use crate::common::*;
use crate::models::binance::spot::symbols::*;
use crate::models::binance::spot::symbols::dsl::symbols;

#[derive(Default)]
pub struct SymbolsRepository {}

impl<'a> SymbolsRepository {
  pub fn flush(&self, ctx: &'a mut Ctx<'_>) -> Result<(), Box<dyn std::error::Error>> {
    println!("symbols flush");
    if 1 > 0 {
      return Err(Box::from("symbols repository flush failed"))
    }
    Ok(())
  }

  pub fn count(&self, ctx: &'a mut Ctx<'_>) -> Result<i64, Box<dyn std::error::Error>> {
    println!("symbols count");
    let pool = Db::new(1).expect("db connect failed");
    let mut conn = pool.get().unwrap();
    let count = symbols.filter(status.eq("TRADING"))
      .filter(is_spot.eq(true))
      .count()
      .get_result(&mut conn)?;
    Ok(count)
  }
}
