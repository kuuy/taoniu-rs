use chrono::Local;
use diesel::prelude::*;
use redis::AsyncCommands;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

use crate::common::*;
use crate::config::binance::spot::config as Config;
use crate::repositories::binance::spot::tickers::*;
use crate::repositories::binance::spot::symbols::*;
use crate::schema::binance::spot::symbols::*;
use crate::models::binance::spot::symbol::Filters;

#[derive(Default)]
pub struct StrategiesRepository {}

impl StrategiesRepository {
  pub async fn atr<T>(
    ctx: Ctx,
    symbol: T,
    interval: T,
  ) -> Result<(), Box<dyn std::error::Error>>
  where
    T: AsRef<str>
  {
    let mut rdb = ctx.rdb.lock().await.clone();
    let symbol = symbol.as_ref();
    let interval = interval.as_ref();

    let day = Local::now().format("%m%d").to_string();
    let redis_key = format!("{}:{}:{}:{}", Config::REDIS_KEY_INDICATORS, interval, symbol, day);
    let atr: Option<f64> = match rdb.hget(&redis_key, "atr").await {
      Ok(Some(atr)) => atr,
      Ok(None) => return Err(Box::from(format!("atr of {symbol:} {interval:} not exists"))),
      Err(e) => return Err(e.into()),
    };
    let atr = Decimal::from_f64(atr.unwrap()).unwrap();

    let price = match TickersRepository::price(
      ctx.clone(),
      &symbol,
    ).await {
      Ok(price) => price,
      Err(e) => return Err(e.into()),
    };
    let price = Decimal::from_f64(price).unwrap();

    let tick_size: f64;
    match SymbolsRepository::filters(ctx.clone(), symbol).await {
      Ok(result) => {
        (tick_size, _) = result;
      },
      Err(e) => return Err(e.into()),
    }
    let tick_size = Decimal::from_f64(tick_size).unwrap();

    let profit_target = price * dec!(2) - atr * dec!(1.5);
    let stop_loss_point = price - atr;
    let take_profit_price = stop_loss_point + (profit_target - stop_loss_point) / dec!(2);

    let profit_target = (profit_target / tick_size).ceil() * tick_size;
    let stop_loss_point = (stop_loss_point / tick_size).floor() * tick_size;
    let take_profit_price = (take_profit_price / tick_size).ceil() * tick_size;

    let risk_reward_ratio = ((price - stop_loss_point) / (profit_target - price)).round_dp(4).to_f32().unwrap();
    let take_profit_ratio = (price / take_profit_price).round_dp(4).to_f32().unwrap();

    rdb.hset_multiple(
      &redis_key,
      &[
        ("profit_target", profit_target.to_string()),
        ("stop_loss_point", stop_loss_point.to_string()),
        ("take_profit_price", take_profit_price.to_string()),
        ("risk_reward_ratio", risk_reward_ratio.to_string()),
        ("take_profit_ratio", take_profit_ratio.to_string()),
      ],
    ).await?;

    Ok(())
  }

  pub async fn zlema<T>(
    ctx: Ctx,
    symbol: T,
    interval: T,
  ) -> Result<(), Box<dyn std::error::Error>>
  where
    T: AsRef<str>
  {
    let _ = ctx.clone();
    let symbol = symbol.as_ref();
    let interval = interval.as_ref();

    let _ = Local::now().format("%m%d").to_string();

    println!("zlema {symbol:} {interval:}");

    Ok(())
  }

  pub async fn ha_zlema<T>(
    ctx: Ctx,
    symbol: T,
    interval: T,
  ) -> Result<(), Box<dyn std::error::Error>>
  where
    T: AsRef<str>
  {
    let _ = ctx.clone();
    let symbol = symbol.as_ref();
    let interval = interval.as_ref();

    let _ = Local::now().format("%m%d").to_string();

    println!("ha zlema {symbol:} {interval:}");

    Ok(())
  }

  pub async fn kdj<T>(
    ctx: Ctx,
    symbol: T,
    interval: T,
  ) -> Result<(), Box<dyn std::error::Error>>
  where
    T: AsRef<str>
  {
    let _ = ctx.clone();
    let symbol = symbol.as_ref();
    let interval = interval.as_ref();

    let _ = Local::now().format("%m%d").to_string();

    println!("kdj {symbol:} {interval:}");

    Ok(())
  }

  pub async fn bbands<T>(
    ctx: Ctx,
    symbol: T,
    interval: T,
  ) -> Result<(), Box<dyn std::error::Error>>
  where
    T: AsRef<str>
  {
    let _ = ctx.clone();
    let symbol = symbol.as_ref();
    let interval = interval.as_ref();

    let _ = Local::now().format("%m%d").to_string();

    println!("bbands {symbol:} {interval:}");

    Ok(())
  }

  pub async fn ichimoku_cloud<T>(
    ctx: Ctx,
    symbol: T,
    interval: T,
  ) -> Result<(), Box<dyn std::error::Error>>
  where
    T: AsRef<str>
  {
    let _ = ctx.clone();
    let symbol = symbol.as_ref();
    let interval = interval.as_ref();

    let _ = Local::now().format("%m%d").to_string();

    println!("ichimoku cloud {symbol:} {interval:}");

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
