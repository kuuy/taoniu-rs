use std::ops::Sub;
use std::time::Duration;

use talib_sys::{TA_Integer, TA_Real, TA_ATR, TA_MA, TA_MAType_TA_MAType_EMA, TA_STOCH, TA_BBANDS, TA_RetCode};

use diesel::prelude::*;
use diesel::query_builder::QueryFragment;
use diesel::ExpressionMethods;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use chrono::{DateTime, Utc, Local, Timelike};

use redis::AsyncCommands;

use crate::common::*;
use crate::config::binance::spot::config as Config;
use crate::schema::binance::spot::symbols::*;
use crate::models::binance::spot::symbol::Filters;
use crate::schema::binance::spot::klines::*;

#[derive(Default)]
pub struct IndicatorsRepository {}

impl IndicatorsRepository {
  pub async fn pivot<T>(
    ctx: Ctx,
    symbol: T,
    interval: T,
  ) -> Result<(), Box<dyn std::error::Error>> 
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();
    let interval = interval.as_ref();

    let tick_size: f64;
    match Self::filters(ctx.clone(), symbol).await {
      Ok(result) => {
        (tick_size, _) = result;
      },
      Err(e) => return Err(e.into()),
    }

    let pool = ctx.pool.read().unwrap();
    let mut conn = pool.get().unwrap();

    let (close, high, low, timestamp): (f64, f64, f64, i64);
    match klines::table
      .select((klines::close, klines::high, klines::low, klines::timestamp))
      .filter(klines::symbol.eq(symbol))
      .filter(klines::interval.eq(interval))
      .order(klines::timestamp.desc())
      .first::<(f64, f64, f64, i64)>(&mut conn) {
      Ok(result) => {
        (close, high, low, timestamp) = result;
      },
      Err(e) => return Err(e.into()),
    };

    if timestamp < Self::timestamp(interval) - 60000 {
      return Err(Box::from(format!("[{symbol:}] waiting for {interval:} klines flush")))
    }

    let dt = DateTime::from_timestamp_millis(timestamp).unwrap();
    if dt.format("%m%d").to_string() != Utc::now().format("%m%d").to_string() {
      return Err(Box::from(format!("[{symbol:}] {interval:} timestamp is not today")))
    }
    let day = Local::now().format("%m%d").to_string();

    let close = Decimal::from_f64(close).unwrap();
    let high = Decimal::from_f64(high).unwrap();
    let low = Decimal::from_f64(low).unwrap();
    println!("kline {close:} {high:} {low:}");

    let p = (close + high + low) / dec!(3);

    let s1 = p * dec!(2) - high;
    let r1 = p * dec!(2) - low;
    let s2 = p - (r1 - s1);
    let r2 = p + (r1 - s1);
    let s3 = low - (high-p) * dec!(2);
    let r3 = high + (p-low) * dec!(2);

    let tick_size = Decimal::from_f64(tick_size).unwrap();

    let s1 = (s1 / tick_size).floor() * tick_size;
    let s2 = (s2 / tick_size).floor() * tick_size;
    let s3 = (s3 / tick_size).floor() * tick_size;
    let r1 = (r1 / tick_size).ceil() * tick_size;
    let r2 = (r2 / tick_size).ceil() * tick_size;
    let r3 = (r3 / tick_size).ceil() * tick_size;
    println!("item {s1:} {r1:} {s2:} {r2:} {s3:} {r3:}");

    let ttl = Duration::from_secs(30+86400);

    let mut rdb = ctx.rdb.lock().await.clone();
    let redis_key = format!("{}:{}:{}:{}", Config::REDIS_KEY_INDICATORS, interval, symbol, day);
    let is_exists: bool = rdb.exists(&redis_key[..]).await.unwrap();
    rdb.hset_multiple(
      &redis_key[..],
      &[
        ("r3", r3.to_string()),
        ("r2", r2.to_string()),
        ("r1", r1.to_string()),
        ("s1", s1.to_string()),
        ("s2", s2.to_string()),
        ("s3", s3.to_string()),
      ],
    ).await?;
    if !is_exists {
      rdb.expire(&redis_key[..], ttl.as_secs().try_into().unwrap()).await?;
    }

    Ok(())
  }

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
    let symbol = symbol.as_ref();
    let interval = interval.as_ref();

    let pool = ctx.pool.read().unwrap();
    let mut conn = pool.get().unwrap();

    let items = klines::table
      .select((klines::close, klines::high, klines::low, klines::timestamp))
      .filter(klines::symbol.eq(symbol))
      .filter(klines::interval.eq(interval))
      .order(klines::timestamp.desc())
      .limit(limit)
      .load::<(f64, f64, f64, i64)>(&mut conn)?;

    if items.len() < limit as usize {
      return Err(Box::from(format!("[{symbol:}] {interval:} klines not enough")))
    }

    let mut closes: Vec<TA_Real> = Vec::new();
    let mut highs: Vec<TA_Real> = Vec::new();
    let mut lows: Vec<TA_Real> = Vec::new();
    let mut first_timestamp: i64 = 0;
    let mut last_timestamp: i64 = 0;
    let current_timestamp = Self::timestamp(interval);

    for (close, high, low, timestamp) in items {
      if first_timestamp == 0 {
        if timestamp < current_timestamp - 60000 {
          return Err(Box::from(format!("[{symbol:}] waiting for {interval:} klines flush")))
        }
        first_timestamp = timestamp;
      }
      if last_timestamp > 0 && last_timestamp != timestamp + Self::timestep(interval) {
        return Err(Box::from(format!("[{symbol:}] {interval:} klines lost")))
      }
      closes.splice(0..0, vec![close]);
      highs.splice(0..0, vec![high]);
      lows.splice(0..0, vec![low]);
      last_timestamp = timestamp;
    }

    let dt = DateTime::from_timestamp_millis(first_timestamp).unwrap();
    if dt.format("%m%d").to_string() != Utc::now().format("%m%d").to_string() {
      return Err(Box::from(format!("[{symbol:}] {interval:} timestamp is not today")))
    }
    let day = Local::now().format("%m%d").to_string();

    let result: f64;

    unsafe {
      let size = closes.len();
      let mut out: Vec<TA_Real> = Vec::with_capacity(size);
      let mut out_begin: TA_Integer = 0;
      let mut out_size: TA_Integer = 0;

      let ret_code = TA_ATR(
        0,
        size as i32 - 1,
        highs.as_ptr(),
        lows.as_ptr(),
        closes.as_ptr(),
        period,
        &mut out_begin,
        &mut out_size,
        out.as_mut_ptr()
      );
      let out_size = out_size as usize;
      match ret_code {
        TA_RetCode::TA_SUCCESS => {
          out.set_len(out_size);
          result = out[out_size-1];
        },
        _ => return Err(Box::from(format!("[{symbol:}] {interval:} calc failed {ret_code:?}")))
      }
    }

    let ttl = Duration::from_secs(30+86400);

    let mut rdb = ctx.rdb.lock().await.clone();
    let redis_key = format!("{}:{}:{}:{}", Config::REDIS_KEY_INDICATORS, interval, symbol, day);
    let is_exists: bool = rdb.exists(&redis_key[..]).await.unwrap();
    rdb.hset(
      &redis_key[..],
      "atr",
      result.to_string(),
    ).await?;
    if !is_exists {
      rdb.expire(&redis_key[..], ttl.as_secs().try_into().unwrap()).await?;
    }
    println!("result {result:}");

    Ok(())
  }

  pub async fn zlema<T>(
    ctx: Ctx,
    symbol: T,
    interval: T,
    period: i32,
    limit: i64,
  ) -> Result<(), Box<dyn std::error::Error>> 
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();
    let interval = interval.as_ref();

    let pool = ctx.pool.read().unwrap();
    let mut conn = pool.get().unwrap();

    let items = klines::table
      .select((klines::close, klines::timestamp))
      .filter(klines::symbol.eq(symbol))
      .filter(klines::interval.eq(interval))
      .order(klines::timestamp.desc())
      .limit(limit)
      .load::<(f64, i64)>(&mut conn)?;

    if items.len() < limit as usize {
      return Err(Box::from(format!("[{symbol:}] {interval:} klines not enough")))
    }

    let lag = ((period - 1) / 2) as usize;
    println!("lag {lag:}");

    let mut data: Vec<TA_Real> = Vec::new();
    let mut temp: Vec<TA_Real> = Vec::new();
    let mut first_close:f64 = 0.0;
    let mut first_timestamp: i64 = 0;
    let mut last_timestamp: i64 = 0;
    let current_timestamp = Self::timestamp(interval);

    for (close, timestamp) in items {
      if first_timestamp == 0 {
        if timestamp < current_timestamp - 60000 {
          return Err(Box::from(format!("[{symbol:}] waiting for {interval:} klines flush")))
        }
        first_close = close;
        first_timestamp = timestamp;
      }
      if last_timestamp > 0 && last_timestamp != timestamp + Self::timestep(interval) {
        return Err(Box::from(format!("[{symbol:}] {interval:} klines lost")))
      }
      if temp.len() < lag  {
        temp.splice(0..0, vec![close]);
      } else {
        let value = temp.pop().unwrap();
        data.splice(0..0, vec![close - value]);
        temp.splice(0..0, vec![close]);
      }
      last_timestamp = timestamp;
    }

    let dt = DateTime::from_timestamp_millis(first_timestamp).unwrap();
    if dt.format("%m%d").to_string() != Utc::now().format("%m%d").to_string() {
      return Err(Box::from(format!("[{symbol:}] {interval:} timestamp is not today")))
    }
    let day = Local::now().format("%m%d").to_string();

    let result: String;

    unsafe {
      let size = data.len();
      let mut out: Vec<TA_Real> = Vec::with_capacity(size);
      let mut out_begin: TA_Integer = 0;
      let mut out_size: TA_Integer = 0;

      let ret_code = TA_MA(
        0,
        size as i32 - 1,
        data.as_ptr(),
        period,
        TA_MAType_TA_MAType_EMA,
        &mut out_begin,
        &mut out_size,
        out.as_mut_ptr()
      );
      let out_size = out_size as usize;
      match ret_code {
        TA_RetCode::TA_SUCCESS => {
          out.set_len(out_size as usize);
          result = format!(
            "{},{},{},{}",
            out[out_size-2],
            out[out_size-1],
            first_close,
            current_timestamp,
          );
        },
        _ => return Err(Box::from(format!("[{symbol:}] {interval:} calc failed {ret_code:?}")))
      }
    }

    let ttl = Duration::from_secs(30+86400);

    let mut rdb = ctx.rdb.lock().await.clone();
    let redis_key = format!("{}:{}:{}:{}", Config::REDIS_KEY_INDICATORS, interval, symbol, day);
    let is_exists: bool = rdb.exists(&redis_key[..]).await.unwrap();
    rdb.hset(
      &redis_key[..],
      "zlema",
      result.clone(),
    ).await?;
    if !is_exists {
      rdb.expire(&redis_key[..], ttl.as_secs().try_into().unwrap()).await?;
    }
    println!("result {result:}");

    Ok(())
  }

  pub async fn ha_zlema<T>(
    ctx: Ctx,
    symbol: T,
    interval: T,
    period: i32,
    limit: i64,
  ) -> Result<(), Box<dyn std::error::Error>> 
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();
    let interval = interval.as_ref();

    let pool = ctx.pool.read().unwrap();
    let mut conn = pool.get().unwrap();

    let items = klines::table
      .select((klines::open, klines::close, klines::high, klines::low, klines::timestamp))
      .filter(klines::symbol.eq(symbol))
      .filter(klines::interval.eq(interval))
      .order(klines::timestamp.desc())
      .limit(limit)
      .load::<(f64, f64, f64, f64, i64)>(&mut conn)?;

    if items.len() < limit as usize {
      return Err(Box::from(format!("[{symbol:}] {interval:} klines not enough")))
    }

    let lag = ((period - 1) / 2) as usize;
    println!("lag {lag:}");

    let mut data: Vec<TA_Real> = Vec::new();
    let mut temp: Vec<TA_Real> = Vec::new();
    let mut first_avg_price:f64 = 0.0;
    let mut first_timestamp: i64 = 0;
    let mut last_timestamp: i64 = 0;
    let current_timestamp = Self::timestamp(interval);

    for (open, close, high, low, timestamp) in items {
      let open = Decimal::from_f64(open).unwrap();
      let close = Decimal::from_f64(close).unwrap();
      let high = Decimal::from_f64(high).unwrap();
      let low = Decimal::from_f64(low).unwrap();
      let avg_price = (open + close + high + low) / dec!(4);
      let avg_price = avg_price.to_f64().unwrap();
      if first_timestamp == 0 {
        if timestamp < current_timestamp - 60000 {
          return Err(Box::from(format!("[{symbol:}] waiting for {interval:} klines flush")))
        }
        first_avg_price = avg_price;
        first_timestamp = timestamp;
      }
      if last_timestamp > 0 && last_timestamp != timestamp + Self::timestep(interval) {
        return Err(Box::from(format!("[{symbol:}] {interval:} klines lost")))
      }
      if temp.len() < lag  {
        temp.splice(0..0, vec![avg_price]);
      } else {
        let value = temp.pop().unwrap();
        data.splice(0..0, vec![avg_price - value]);
        temp.splice(0..0, vec![avg_price]);
      }
      last_timestamp = timestamp;
    }

    let dt = DateTime::from_timestamp_millis(first_timestamp).unwrap();
    if dt.format("%m%d").to_string() != Utc::now().format("%m%d").to_string() {
      return Err(Box::from(format!("[{symbol:}] {interval:} timestamp is not today")))
    }
    let day = Local::now().format("%m%d").to_string();

    let result: String;

    unsafe {
      let size = data.len();
      let mut out: Vec<TA_Real> = Vec::with_capacity(size);
      let mut out_begin: TA_Integer = 0;
      let mut out_size: TA_Integer = 0;

      let ret_code = TA_MA(
        0,
        size as i32 - 1,
        data.as_ptr(),
        period,
        TA_MAType_TA_MAType_EMA,
        &mut out_begin,
        &mut out_size,
        out.as_mut_ptr()
      );
      let out_size = out_size as usize;
      match ret_code {
        TA_RetCode::TA_SUCCESS => {
          out.set_len(out_size as usize);
          result = format!(
            "{},{},{},{}",
            out[out_size-2],
            out[out_size-1],
            first_avg_price,
            current_timestamp,
          );
        },
        _ => return Err(Box::from(format!("[{symbol:}] {interval:} calc failed {ret_code:?}")))
      }
    }

    let ttl = Duration::from_secs(30+86400);

    let mut rdb = ctx.rdb.lock().await.clone();
    let redis_key = format!("{}:{}:{}:{}", Config::REDIS_KEY_INDICATORS, interval, symbol, day);
    let is_exists: bool = rdb.exists(&redis_key[..]).await.unwrap();
    rdb.hset(
      &redis_key[..],
      "ha_zlema",
      result.clone(),
    ).await?;
    if !is_exists {
      rdb.expire(&redis_key[..], ttl.as_secs().try_into().unwrap()).await?;
    }
    println!("result {result:}");

    Ok(())
  }

  pub async fn kdj<T>(
    ctx: Ctx,
    symbol: T,
    interval: T,
    long_period: i32,
    short_period: i32,
    limit: i64,
  ) -> Result<(), Box<dyn std::error::Error>> 
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();
    let interval = interval.as_ref();

    let pool = ctx.pool.read().unwrap();
    let mut conn = pool.get().unwrap();

    let items = klines::table
      .select((klines::close, klines::high, klines::low, klines::timestamp))
      .filter(klines::symbol.eq(symbol))
      .filter(klines::interval.eq(interval))
      .order(klines::timestamp.desc())
      .limit(limit)
      .load::<(f64, f64, f64, i64)>(&mut conn)?;

    if items.len() < limit as usize {
      return Err(Box::from(format!("[{symbol:}] {interval:} klines not enough")))
    }

    let mut avg_prices: Vec<TA_Real> = Vec::new();
    let mut highs: Vec<TA_Real> = Vec::new();
    let mut lows: Vec<TA_Real> = Vec::new();
    let mut first_avg_price:f64 = 0.0;
    let mut first_timestamp: i64 = 0;
    let mut last_timestamp: i64 = 0;
    let current_timestamp = Self::timestamp(interval);

    for (close, high, low, timestamp) in items {
      let close = Decimal::from_f64(close).unwrap();
      let high = Decimal::from_f64(high).unwrap();
      let low = Decimal::from_f64(low).unwrap();
      let avg_price = (close + high + low) / dec!(3);
      let avg_price = avg_price.to_f64().unwrap();

      if first_timestamp == 0 {
        if timestamp < current_timestamp - 60000 {
          return Err(Box::from(format!("[{symbol:}] waiting for {interval:} klines flush")))
        }
        first_avg_price = avg_price;
        first_timestamp = timestamp;
      }

      if last_timestamp > 0 && last_timestamp != timestamp + Self::timestep(interval) {
        return Err(Box::from(format!("[{symbol:}] {interval:} klines lost")))
      }

      let avg_price = avg_price.to_f64().unwrap();
      let high = high.to_f64().unwrap();
      let low = low.to_f64().unwrap();

      avg_prices.splice(0..0, vec![avg_price]);
      highs.splice(0..0, vec![high]);
      lows.splice(0..0, vec![low]);

      last_timestamp = timestamp;
    }

    let dt = DateTime::from_timestamp_millis(first_timestamp).unwrap();
    if dt.format("%m%d").to_string() != Utc::now().format("%m%d").to_string() {
      return Err(Box::from(format!("[{symbol:}] {interval:} timestamp is not today")))
    }
    let day = Local::now().format("%m%d").to_string();

    let result: String;

    unsafe {
      let size = avg_prices.len();
      let mut out_slowk: Vec<TA_Real> = Vec::with_capacity(size);
      let mut out_slowd: Vec<TA_Real> = Vec::with_capacity(size);
      let mut out_begin: TA_Integer = 0;
      let mut out_size: TA_Integer = 0;

      let ret_code = TA_STOCH(
        0,
        size as i32 - 1,
        highs.as_ptr(),
        lows.as_ptr(),
        avg_prices.as_ptr(),
        long_period,
        short_period,
        0,
        short_period,
        0,
        &mut out_begin,
        &mut out_size,
        out_slowk.as_mut_ptr(),
        out_slowd.as_mut_ptr(),
      );
      let out_size = out_size as usize;
      match ret_code {
        TA_RetCode::TA_SUCCESS => {
          out_slowk.set_len(out_size as usize);
          out_slowd.set_len(out_size as usize);
          let slowk = Decimal::from_f64(out_slowk[out_size-1]).unwrap();
          let slowd = Decimal::from_f64(out_slowd[out_size-1]).unwrap();
          let slowj = slowk * dec!(3) - slowd * dec!(2);
          result = format!(
            "{},{},{},{},{}",
            slowk,
            slowd,
            slowj,
            first_avg_price,
            current_timestamp,
          );
        },
        _ => return Err(Box::from(format!("[{symbol:}] {interval:} calc failed {ret_code:?}")))
      }
    }

    let ttl = Duration::from_secs(30+86400);

    let mut rdb = ctx.rdb.lock().await.clone();
    let redis_key = format!("{}:{}:{}:{}", Config::REDIS_KEY_INDICATORS, interval, symbol, day);
    let is_exists: bool = rdb.exists(&redis_key[..]).await.unwrap();
    rdb.hset(
      &redis_key[..],
      "kdj",
      result.to_string(),
    ).await?;
    if !is_exists {
      rdb.expire(&redis_key[..], ttl.as_secs().try_into().unwrap()).await?;
    }
    println!("result {result:}");

    Ok(())
  }

  pub async fn bbands<T>(
    ctx: Ctx,
    symbol: T,
    interval: T,
    period: i32,
    limit: i64,
  ) -> Result<(), Box<dyn std::error::Error>> 
  where
    T: AsRef<str>
  {
    Ok(())
  }

  pub async fn ichimoku_cloud<T>(
    ctx: Ctx,
    symbol: T,
    interval: T,
    tenkan_period: i32,
    kijun_period: i32,
    senkou_period: i32,
    limit: i64,
  ) -> Result<(), Box<dyn std::error::Error>> 
  where
    T: AsRef<str>
  {
    Ok(())
  }

  pub async fn volume_profile<T>(
    ctx: Ctx,
    symbol: T,
    interval: T,
    period: i32,
    limit: i64,
  ) -> Result<(), Box<dyn std::error::Error>> 
  where
    T: AsRef<str>
  {
    Ok(())
  }

  pub fn timestamp<T>(interval: T) -> i64 
  where
    T: AsRef<str>
  {
    let interval = interval.as_ref();
    let mut datetime = Utc::now();
    datetime = datetime.sub(Duration::from_secs(datetime.second() as u64));
    datetime = datetime.sub(Duration::from_nanos(datetime.nanosecond() as u64));
    if interval == "15m" {
      let minutes = datetime.minute() as u64 - ((Decimal::from_u64(datetime.minute() as u64).unwrap() / dec!(15)).floor() * dec!(15)).to_u64().unwrap();
      datetime = datetime.sub(Duration::from_secs(minutes * 60));
    } else if interval == "4h" {
      let hours = datetime.hour() as u64 - ((Decimal::from_u64(datetime.hour() as u64).unwrap() / dec!(4)).floor() * dec!(4)).to_u64().unwrap();
      let minutes = datetime.minute() as u64;
      datetime = datetime.sub(Duration::from_secs(hours * 3600 + minutes * 60));
    } else if interval == "1d" {
      let hours = datetime.hour() as u64;
      let minutes = datetime.minute() as u64;
      datetime = datetime.sub(Duration::from_secs(hours * 3600 + minutes * 60));
    }
    datetime.timestamp_millis()
  }

  pub fn timestep<T>(interval: T) -> i64
  where
    T: AsRef<str>
  {
    let interval = interval.as_ref();
    if interval == "1m" {
      return 60000
    } else if interval == "15m" {
      return 900000
    } else if interval == "4h" {
      return 14400000
    }
    return 86400000
  }

  pub async fn filters<T>(ctx: Ctx, symbol: T) -> Result<(f64, f64), Box<dyn std::error::Error>> 
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();

    let pool = ctx.pool.read().unwrap();
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
