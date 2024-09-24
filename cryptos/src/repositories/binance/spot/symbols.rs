use diesel::prelude::*;

use crate::common::*;
use crate::schema::binance::spot::symbols::*;
use crate::models::binance::spot::symbol::*;

pub struct SymbolsRepository {}

impl SymbolsRepository {
  pub async fn get<T>(
    ctx: Ctx,
    symbol: T,
  ) -> Result<Option<Symbol>, Box<dyn std::error::Error>>
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();

    let pool = ctx.pool.read().await;
    let mut conn = pool.get().unwrap();

    match symbols::table
      .select(Symbol::as_select())
      .filter(symbols::symbol.eq(symbol))
      .first(&mut conn) {
      Ok(symbol) => Ok(Some(symbol)),
      Err(diesel::result::Error::NotFound) => Ok(None),
      Err(e) => Err(e.into()),
    }
  }

  pub async fn flush(ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    println!("symbols flush");
    let _ = ctx.clone();
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

  pub async fn filters<T>(ctx: Ctx, symbol: T) -> Result<(f64, f64), Box<dyn std::error::Error>> 
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();

    let pool = ctx.pool.read().await;
    let mut conn = pool.get().unwrap();

    let (tick_size, step_size): (f64, f64);
    match symbols::table
      .select(symbols::filters)
      .filter(symbols::symbol.eq(symbol))
      .first::<Filters>(&mut conn) {
      Ok(filters) => {
        let values: Vec<&str> = filters.price.split(",").collect();
        tick_size = values[2].parse::<f64>().unwrap();
        let values: Vec<&str> = filters.quote.split(",").collect();
        step_size = values[2].parse::<f64>().unwrap();
      },
      Err(e) => return Err(e.into()),
    };

    Ok((tick_size, step_size))
  }
}
