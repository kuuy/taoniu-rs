use redis::aio::MultiplexedConnection;
use diesel::prelude::*;

use crate::common::*;
use crate::models::binance::spot::scalping::*;
use crate::models::binance::spot::scalping::schema::dsl::*;

#[derive(Default)]
pub struct ScalpingRepository {}

impl<'a> ScalpingRepository {
  pub fn scan(&self, ctx: &'a mut Ctx<'_>) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut conn = ctx.db.get().unwrap();
    let symbols = schema
      .filter(status.eq(1))
      .select(symbol)
      .load::<String>(&mut conn)?;
    Ok(symbols)
  }
}
