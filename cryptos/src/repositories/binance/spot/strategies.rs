use chrono::{prelude::Utc, Local};
use diesel::prelude::*;
use diesel::query_builder::QueryFragment;
use redis::AsyncCommands;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

use crate::common::*;
use crate::config::binance::spot::config as Config;
use crate::repositories::binance::spot::tickers::*;
use crate::repositories::binance::spot::symbols::*;
use crate::models::binance::spot::symbol::Filters;
use crate::schema::binance::spot::symbols::*;
use crate::models::binance::spot::strategy::*;
use crate::schema::binance::spot::strategies::*;

#[derive(Default)]
pub struct StrategiesRepository {}

impl StrategiesRepository {
  pub async fn get<T>(
    ctx: Ctx,
    symbol: T,
    indicator: T,
    interval: T,
  ) -> Result<Option<Strategy>, Box<dyn std::error::Error>>
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();
    let indicator = indicator.as_ref();
    let interval = interval.as_ref();

    let pool = ctx.pool.read().await;
    let mut conn = pool.get().unwrap();

    match strategies::table
      .select(Strategy::as_select())
      .filter(strategies::symbol.eq(symbol))
      .filter(strategies::indicator.eq(indicator))
      .filter(strategies::interval.eq(interval))
      .order(strategies::timestamp.desc())
      .first(&mut conn) {
        Ok(result) => Ok(Some(result)),
        Err(diesel::result::Error::NotFound) => Ok(None),
        Err(err) => Err(err.into()),
      }
  }

  pub async fn last<T>(
    ctx: Ctx,
    symbol: T,
    indicators: Vec<T>,
    interval: T,
    timestamp: i64,
  ) -> Result<Option<Strategy>, Box<dyn std::error::Error>>
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();
    let indicators = indicators.iter().map(|s| s.as_ref()).collect::<Vec<&str>>();
    let interval = interval.as_ref();

    let pool = ctx.pool.read().await;
    let mut conn = pool.get().unwrap();

    match strategies::table
      .select(Strategy::as_select())
      .filter(strategies::symbol.eq(symbol))
      .filter(strategies::indicator.eq_any(indicators.as_slice()))
      .filter(strategies::interval.eq(interval))
      .filter(strategies::timestamp.ge(timestamp))
      .order(strategies::timestamp.desc())
      .first(&mut conn) {
        Ok(result) => Ok(Some(result)),
        Err(diesel::result::Error::NotFound) => Ok(None),
        Err(err) => Err(err.into()),
      }
  }

  pub async fn create(
    ctx: Ctx,
    id: String,
    symbol: String,
    indicator: String,
    interval: String,
    price: f64,
    signal: i32,
    timestamp: i64,
    remark: String,
  ) -> Result<bool, Box<dyn std::error::Error>> {
    let pool = ctx.pool.write().await;
    let mut conn = pool.get().unwrap();

    let now = Utc::now();
    let strategy = Strategy::new(
      id,
      symbol,
      indicator,
      interval,
      price,
      signal,
      timestamp,
      remark,
      now,
      now,
    );
    match diesel::insert_into(strategies::table)
      .values(&strategy)
      .execute(&mut conn) {
      Ok(effective_rows) => Ok(effective_rows > 0),
      Err(err) => Err(err.into()),
    }
  }

  pub async fn update<V>(
    ctx: Ctx,
    id: String,
    value: V,
  ) -> Result<bool, Box<dyn std::error::Error>> 
  where
    V: diesel::AsChangeset<Target = strategies::table>,
    <V as diesel::AsChangeset>::Changeset: QueryFragment<diesel::pg::Pg>,
  {
    let pool = ctx.pool.write().await;
    let mut conn = pool.get().unwrap();
    match diesel::update(strategies::table.find(id)).set(value).execute(&mut conn) {
      Ok(effective_rows) => Ok(effective_rows > 0),
      Err(err) => Err(err.into()),
    }
  }

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
    let indicator = "atr";
    let interval = interval.as_ref();

    let day = Local::now().format("%m%d").to_string();
    let redis_key = format!("{}:{}:{}:{}", Config::REDIS_KEY_INDICATORS, interval, symbol, day);
    let atr: Option<f64> = match rdb.hget(&redis_key, indicator).await {
      Ok(Some(result)) => result,
      Ok(None) => return Err(Box::from(format!("atr of {symbol:} {interval:} not exists"))),
      Err(err) => return Err(err.into()),
    };
    let atr = Decimal::from_f64(atr.unwrap()).unwrap();

    let price = match TickersRepository::price(
      ctx.clone(),
      &symbol,
    ).await {
      Ok(price) => price,
      Err(err) => return Err(err.into()),
    };
    let price = Decimal::from_f64(price).unwrap();

    let (tick_size, _) = match SymbolsRepository::filters(ctx.clone(), symbol).await {
      Ok(result) => result,
      Err(err) => return Err(err.into()),
    };
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
    let mut rdb = ctx.rdb.lock().await.clone();
    let symbol = symbol.as_ref();
    let indicator = "zlema";
    let interval = interval.as_ref();

    let day = Local::now().format("%m%d").to_string();
    let redis_key = format!("{}:{}:{}:{}", Config::REDIS_KEY_INDICATORS, interval, symbol, day);
    let data: Option<String> = match rdb.hget(&redis_key, indicator).await {
      Ok(Some(result)) => result,
      Ok(None) => return Err(Box::from(format!("{indicator:} of {symbol:} {interval:} not exists"))),
      Err(err) => return Err(err.into()),
    };
    let data = data.unwrap();
    let values: Vec<&str> = data.split(",").collect();

    let zlema1 = values[0].parse::<f64>().unwrap();
    let zlema2 = values[1].parse::<f64>().unwrap();
    let price = values[2].parse::<f64>().unwrap();
    let timestamp = values[3].parse::<i64>().unwrap();

    if zlema1 * zlema2 >= 0.0 {
      return Ok(())
    }

    let signal: i32;
    if zlema2 > 0.0 {
      signal = 1;
    } else {
      signal = 2;
    }

    let strategy: Option<Strategy> = match Self::get(ctx.clone(), symbol, indicator, interval).await {
      Ok(Some(result)) => Some(result),
      Ok(None) => None,
      Err(err) => return Err(err.into()),
    };

    if !strategy.is_none() {
      let strategy = strategy.unwrap();
      if strategy.signal == signal {
        return Err(Box::from(format!("{indicator:} of {symbol:} {interval:} waiting for change")))
      }
      if strategy.timestamp >= timestamp {
        return Err(Box::from(format!("{indicator:} of {symbol:} {interval:} exists")))
      }
    }

    let id = xid::new().to_string();
    match Self::create(
      ctx.clone(),
      id,
      symbol.to_string(),
      indicator.to_string(),
      interval.to_string(),
      price,
      signal,
      timestamp,
      "".to_string(),
    ).await {
      Ok(_) => (),
      Err(err) => return Err(err.into()),
    }

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
    let mut rdb = ctx.rdb.lock().await.clone();
    let symbol = symbol.as_ref();
    let indicator = "ha_zlema";
    let interval = interval.as_ref();

    let day = Local::now().format("%m%d").to_string();
    let redis_key = format!("{}:{}:{}:{}", Config::REDIS_KEY_INDICATORS, interval, symbol, day);
    let data: Option<String> = match rdb.hget(&redis_key, indicator).await {
      Ok(Some(result)) => result,
      Ok(None) => return Err(Box::from(format!("{indicator:} of {symbol:} {interval:} not exists"))),
      Err(err) => return Err(err.into()),
    };
    let data = data.unwrap();
    let values: Vec<&str> = data.split(",").collect();

    let zlema1 = values[0].parse::<f64>().unwrap();
    let zlema2 = values[1].parse::<f64>().unwrap();
    let price = values[2].parse::<f64>().unwrap();
    let timestamp = values[3].parse::<i64>().unwrap();

    if zlema1 * zlema2 >= 0.0 {
      return Ok(())
    }

    let signal: i32;
    if zlema2 > 0.0 {
      signal = 1;
    } else {
      signal = 2;
    }

    let strategy: Option<Strategy> = match Self::get(ctx.clone(), symbol, indicator, interval).await {
      Ok(Some(result)) => Some(result),
      Ok(None) => None,
      Err(err) => return Err(err.into()),
    };

    if !strategy.is_none() {
      let strategy = strategy.unwrap();
      if strategy.signal == signal {
        return Err(Box::from(format!("{indicator:} of {symbol:} {interval:} waiting for change")))
      }
      if strategy.timestamp >= timestamp {
        return Err(Box::from(format!("{indicator:} of {symbol:} {interval:} exists")))
      }
    }

    let id = xid::new().to_string();
    let _ = match Self::create(
      ctx.clone(),
      id,
      symbol.to_string(),
      indicator.to_string(),
      interval.to_string(),
      price,
      signal,
      timestamp,
      "".to_string(),
    ).await {
      Ok(result) => result,
      Err(err) => return Err(err.into()),
    };

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
    let mut rdb = ctx.rdb.lock().await.clone();
    let symbol = symbol.as_ref();
    let indicator = "kdj";
    let interval = interval.as_ref();

    let day = Local::now().format("%m%d").to_string();
    let redis_key = format!("{}:{}:{}:{}", Config::REDIS_KEY_INDICATORS, interval, symbol, day);
    let data: Option<String> = match rdb.hget(&redis_key, indicator).await {
      Ok(Some(result)) => result,
      Ok(None) => return Err(Box::from(format!("{indicator:} of {symbol:} {interval:} not exists"))),
      Err(err) => return Err(err.into()),
    };
    let data = data.unwrap();
    let values: Vec<&str> = data.split(",").collect();

    let k = values[0].parse::<f64>().unwrap();
    let d = values[1].parse::<f64>().unwrap();
    let j = values[2].parse::<f64>().unwrap();
    let price = values[3].parse::<f64>().unwrap();
    let timestamp = values[4].parse::<i64>().unwrap();

    let signal: i32;
    if k < 20.0 && d > 60.0 && j < 60.0 {
      signal = 1;
    } else if k > 80.0 && d > 70.0 && j > 90.0 {
      signal = 2;
    } else {
      return Ok(())
    }

    let strategy: Option<Strategy> = match Self::get(ctx.clone(), symbol, indicator, interval).await {
      Ok(Some(result)) => Some(result),
      Ok(None) => None,
      Err(err) => return Err(err.into()),
    };

    if !strategy.is_none() {
      let strategy = strategy.unwrap();
      if strategy.signal == signal {
        return Err(Box::from(format!("{indicator:} of {symbol:} {interval:} waiting for change")))
      }
      if strategy.timestamp >= timestamp {
        return Err(Box::from(format!("{indicator:} of {symbol:} {interval:} exists")))
      }
    }

    let id = xid::new().to_string();
    let _ = match Self::create(
      ctx.clone(),
      id,
      symbol.to_string(),
      indicator.to_string(),
      interval.to_string(),
      price,
      signal,
      timestamp,
      "".to_string(),
    ).await {
      Ok(result) => result,
      Err(err) => return Err(err.into()),
    };

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
    let mut rdb = ctx.rdb.lock().await.clone();
    let symbol = symbol.as_ref();
    let indicator = "bbands";
    let interval = interval.as_ref();

    let day = Local::now().format("%m%d").to_string();
    let redis_key = format!("{}:{}:{}:{}", Config::REDIS_KEY_INDICATORS, interval, symbol, day);
    let data: Option<String> = match rdb.hget(&redis_key, indicator).await {
      Ok(Some(result)) => result,
      Ok(None) => return Err(Box::from(format!("{indicator:} of {symbol:} {interval:} not exists"))),
      Err(err) => return Err(err.into()),
    };
    let data = data.unwrap();
    let values: Vec<&str> = data.split(",").collect();

    let b1 = values[0].parse::<f64>().unwrap();
    let b2 = values[1].parse::<f64>().unwrap();
    let b3 = values[2].parse::<f64>().unwrap();
    let w1 = values[3].parse::<f64>().unwrap();
    let w2 = values[4].parse::<f64>().unwrap();
    let w3 = values[5].parse::<f64>().unwrap();
    let price = values[6].parse::<f64>().unwrap();
    let timestamp = values[7].parse::<i64>().unwrap();

    let signal: i32;
    if b1 < 0.5 && b2 < 0.5 && b3 > 0.5 {
      signal = 1;
    } else if b1 > 0.5 && b2 < 0.5 && b3 < 0.5 {
      signal = 2;
    } else if b1 > 0.8 && b2 > 0.8 && b3 > 0.8 {
      signal = 1;
    } else if b1 > 0.8 && b2 > 0.8 && b3 < 0.8 {
      signal = 2;
    } else {
      return Err(Box::from(format!("{indicator:} of {symbol:} {interval:} invalid")))
    }

    if w1 < 0.1 && w2 < 0.1 && w3 < 0.1 {
      if w1 < 0.03 || w2 < 0.03 || w3 > 0.03 {
        return Ok(())
      } 
    }

    let strategy: Option<Strategy> = match Self::get(ctx.clone(), symbol, indicator, interval).await {
      Ok(Some(result)) => Some(result),
      Ok(None) => None,
      Err(err) => return Err(err.into()),
    };

    if !strategy.is_none() {
      let strategy = strategy.unwrap();
      if strategy.signal == signal {
        return Err(Box::from(format!("{indicator:} of {symbol:} {interval:} waiting for change")))
      }
      if strategy.timestamp >= timestamp {
        return Err(Box::from(format!("{indicator:} of {symbol:} {interval:} exists")))
      }
    }

    let id = xid::new().to_string();
    let _ = match Self::create(
      ctx.clone(),
      id,
      symbol.to_string(),
      indicator.to_string(),
      interval.to_string(),
      price,
      signal,
      timestamp,
      "".to_string(),
    ).await {
      Ok(result) => result,
      Err(err) => return Err(err.into()),
    };

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
    let mut rdb = ctx.rdb.lock().await.clone();
    let symbol = symbol.as_ref();
    let indicator = "ichimoku_cloud";
    let interval = interval.as_ref();

    let day = Local::now().format("%m%d").to_string();
    let redis_key = format!("{}:{}:{}:{}", Config::REDIS_KEY_INDICATORS, interval, symbol, day);
    let data: Option<String> = match rdb.hget(&redis_key, indicator).await {
      Ok(Some(result)) => result,
      Ok(None) => return Err(Box::from(format!("{indicator:} of {symbol:} {interval:} not exists"))),
      Err(err) => return Err(err.into()),
    };
    let data = data.unwrap();
    let values: Vec<&str> = data.split(",").collect();

    let signal = values[0].parse::<i32>().unwrap();
    let price = values[6].parse::<f64>().unwrap();
    let timestamp = values[7].parse::<i64>().unwrap();

    if signal == 0 {
      return Err(Box::from(format!("{indicator:} of {symbol:} {interval:} invalid")))
    }

    let strategy: Option<Strategy> = match Self::get(ctx.clone(), symbol, indicator, interval).await {
      Ok(Some(result)) => Some(result),
      Ok(None) => None,
      Err(err) => return Err(err.into()),
    };

    if !strategy.is_none() {
      let strategy = strategy.unwrap();
      if strategy.signal == signal {
        return Err(Box::from(format!("{indicator:} of {symbol:} {interval:} waiting for change")))
      }
      if strategy.timestamp >= timestamp {
        return Err(Box::from(format!("{indicator:} of {symbol:} {interval:} exists")))
      }
    }

    let id = xid::new().to_string();
    let _ = match Self::create(
      ctx.clone(),
      id,
      symbol.to_string(),
      indicator.to_string(),
      interval.to_string(),
      price,
      signal,
      timestamp,
      "".to_string(),
    ).await {
      Ok(result) => result,
      Err(err) => return Err(err.into()),
    };

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
      }
      Err(err) => return Err(err.into()),
    };

    Ok((tick_size, step_size))
  }
}
