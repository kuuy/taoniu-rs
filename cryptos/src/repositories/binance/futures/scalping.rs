use redis::aio::MultiplexedConnection;
use diesel::prelude::*;

use crate::common::*;
use crate::models::binance::futures::scalping::*;
use crate::models::binance::futures::scalping::schema::dsl::*;

#[derive(Default)]
pub struct ScalpingRepository {}

impl<'a> ScalpingRepository {
  pub fn scan(&self, ctx: &'a mut Ctx<'_>) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut conn = ctx.db.get().unwrap();
    let symbols = schema
      .filter(status.eq_any([1, 2]))
      .select(symbol)
      .load::<String>(&mut conn)?;
    Ok(symbols)
  }
}
