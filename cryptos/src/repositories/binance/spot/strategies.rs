use diesel::prelude::*;
use chrono::Local;

use crate::common::*;
use crate::schema::binance::spot::symbols::*;
use crate::models::binance::spot::symbol::Filters;

#[derive(Default)]
pub struct StrategiesRepository {}

impl StrategiesRepository {
  pub async fn atr<T>(
    ctx: Ctx,
    symbol: T,
    interval: T,
    period: i32,
    limit: i64,
  ) -> Result<(), Box<dyn std::error::Error>>
  where
    T: AsRef<str>
  {
    let _ = ctx.clone();
    let symbol = symbol.as_ref();
    let interval = interval.as_ref();

    let _ = Local::now().format("%m%d").to_string();

    println!("atr {symbol:} {interval:} {period:} {limit:}");

    Ok(())
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
