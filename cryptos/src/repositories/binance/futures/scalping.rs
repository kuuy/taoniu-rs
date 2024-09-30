use diesel::prelude::*;

use crate::common::*;
use crate::models::binance::futures::scalping::*;
use crate::schema::binance::futures::scalping::*;

pub mod plans;

#[derive(Default)]
pub struct ScalpingRepository {}

impl ScalpingRepository {
  pub async fn get<T>(
    ctx: Ctx,
    symbol: T,
  ) -> Result<Option<Scalping>, Box<dyn std::error::Error>>
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();

    let pool = ctx.pool.read().await;
    let mut conn = pool.get().unwrap();

    match scalping::table
      .select(Scalping::as_select())
      .filter(scalping::symbol.eq(symbol))
      .filter(scalping::status.eq(1))
      .first(&mut conn) {
        Ok(result) => Ok(Some(result)),
        Err(diesel::result::Error::NotFound) => Ok(None),
        Err(e) => Err(e.into()),
      }
  }

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
