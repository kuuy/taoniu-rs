use diesel::prelude::*;

use crate::common::*;
use crate::schema::binance::futures::symbols::*;
use crate::models::binance::futures::symbol::*;

#[derive(Default)]
pub struct SymbolsRepository {}

impl SymbolsRepository {
  pub fn flush(_: Ctx) -> Result<(), Box<dyn std::error::Error>> {
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
      .count()
      .get_result(&mut conn)?;
    Ok(count)
  }

  pub async fn pairs<T>(ctx: Ctx, symbol: T) -> Result<(String, String), Box<dyn std::error::Error>>
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();

    let pool = ctx.pool.read().await;
    let mut conn = pool.get().unwrap();

    match symbols::table
      .select((symbols::base_asset, symbols::quote_asset))
      .filter(symbols::symbol.eq(symbol))
      .first::<(String, String)>(&mut conn) {
      Ok(result) => Ok(result),
      Err(err) => return Err(err.into()),
    }
  }

  pub async fn filters<T>(ctx: Ctx, symbol: T) -> Result<(f64, f64, f64), Box<dyn std::error::Error>> 
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();

    let pool = ctx.pool.read().await;
    let mut conn = pool.get().unwrap();

    let (tick_size, step_size, notional): (f64, f64, f64);
    match symbols::table
      .select(symbols::filters)
      .filter(symbols::symbol.eq(symbol))
      .first::<Filters>(&mut conn) {
      Ok(filters) => {
        let values: Vec<&str> = filters.price.split(",").collect();
        tick_size = values[2].parse::<f64>().unwrap();
        let values: Vec<&str> = filters.quote.split(",").collect();
        step_size = values[2].parse::<f64>().unwrap();
        notional = filters.notional.parse::<f64>().unwrap();
      }
      Err(err) => return Err(err.into()),
    };

    Ok((tick_size, step_size, notional))
  }
}
