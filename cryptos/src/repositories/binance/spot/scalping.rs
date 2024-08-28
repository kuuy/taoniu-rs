use diesel::prelude::*;

use crate::common::*;
use crate::models::binance::spot::scalping::schema::dsl::*;

#[derive(Default)]
pub struct ScalpingRepository {}

impl ScalpingRepository {
  pub fn scan(&self, ctx: Ctx) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let pool = ctx.pool.read().unwrap();
    let mut conn = pool.get().unwrap();
    let symbols = schema
      .filter(status.eq_any([1, 2]))
      .select(symbol)
      .load::<String>(&mut conn)?;
    Ok(symbols)
  }
}
