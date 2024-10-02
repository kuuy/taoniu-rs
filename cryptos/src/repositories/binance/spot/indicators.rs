use std::ops::Sub;
use std::time::Duration;
use std::collections::HashMap;

use talib_sys::{TA_Integer, TA_Real, TA_ATR, TA_MA, TA_MAType_TA_MAType_EMA, TA_STOCH, TA_BBANDS, TA_RetCode};

use chrono::{prelude::Utc, DateTime, Local, Timelike};
use diesel::prelude::*;

use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use rust_decimal::MathematicalOps;

use redis::AsyncCommands;

use crate::common::*;
use crate::config::binance::spot::config as Config;
use crate::schema::binance::spot::symbols::*;
use crate::models::binance::spot::symbol::Filters;
use crate::schema::binance::spot::klines::*;

#[derive(Default)]
pub struct IndicatorsRepository {}

#[derive(Debug, Clone)]
struct VolumeSegment {
  prices: Vec<f64>,
  offsets: Vec<usize>,
  volume: f64,
}

impl VolumeSegment {
  pub fn new(
    prices: Vec<f64>,
    offsets: Vec<usize>,
    volume: f64,
  ) -> Self {
    Self {
      prices: prices,
      offsets: offsets,
      volume: volume,
    }
  }
}

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

    let pool = ctx.pool.read().await;
    let mut conn = pool.get().unwrap();

    let tick_size: f64;
    match Self::filters(ctx.clone(), symbol).await {
      Ok(result) => {
        (tick_size, _) = result;
      },
      Err(e) => return Err(e.into()),
    }

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
    let is_exists: bool = rdb.exists(&redis_key).await.unwrap();
    rdb.hset_multiple(
      &redis_key,
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
      rdb.expire(&redis_key, ttl.as_secs().try_into().unwrap()).await?;
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

    let pool = ctx.pool.read().await;
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
    let is_exists: bool = rdb.exists(&redis_key).await.unwrap();
    rdb.hset(
      &redis_key,
      "atr",
      result.to_string(),
    ).await?;
    if !is_exists {
      rdb.expire(&redis_key, ttl.as_secs().try_into().unwrap()).await?;
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

    let pool = ctx.pool.read().await;
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
    let is_exists: bool = rdb.exists(&redis_key).await.unwrap();
    rdb.hset(
      &redis_key,
      "zlema",
      result.clone(),
    ).await?;
    if !is_exists {
      rdb.expire(&redis_key, ttl.as_secs().try_into().unwrap()).await?;
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

    let pool = ctx.pool.read().await;
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
    let is_exists: bool = rdb.exists(&redis_key).await.unwrap();
    rdb.hset(
      &redis_key,
      "ha_zlema",
      result.clone(),
    ).await?;
    if !is_exists {
      rdb.expire(&redis_key, ttl.as_secs().try_into().unwrap()).await?;
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

    let pool = ctx.pool.read().await;
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
    let is_exists: bool = rdb.exists(&redis_key).await.unwrap();
    rdb.hset(
      &redis_key,
      "kdj",
      result.to_string(),
    ).await?;
    if !is_exists {
      rdb.expire(&redis_key, ttl.as_secs().try_into().unwrap()).await?;
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
    let symbol = symbol.as_ref();
    let interval = interval.as_ref();

    let pool = ctx.pool.read().await;
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
    let mut closes: Vec<TA_Real> = Vec::new();
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
      closes.splice(0..0, vec![high]);
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
      let mut out_ubands: Vec<TA_Real> = Vec::with_capacity(size);
      let mut out_mbands: Vec<TA_Real> = Vec::with_capacity(size);
      let mut out_lbands: Vec<TA_Real> = Vec::with_capacity(size);
      let mut out_begin: TA_Integer = 0;
      let mut out_size: TA_Integer = 0;

      let ret_code = TA_BBANDS(
        0,
        size as i32 - 1,
        avg_prices.as_ptr(),
        period,
        2.0,
        2.0,
        0,
        &mut out_begin,
        &mut out_size,
        out_ubands.as_mut_ptr(),
        out_mbands.as_mut_ptr(),
        out_lbands.as_mut_ptr(),
      );
      let period = period as usize;
      let out_size = out_size as usize;
      match ret_code {
        TA_RetCode::TA_SUCCESS => {
          out_ubands.set_len(out_size);
          out_mbands.set_len(out_size);
          out_lbands.set_len(out_size);

          if out_ubands[out_size-3] == out_lbands[out_size-3] 
            || out_ubands[out_size-2] == out_lbands[out_size-2] 
            || out_ubands[out_size-1] == out_lbands[out_size-1] {
            return Err(Box::from(format!("[{symbol:}] {interval:} klined invalid")))
          }

          let p1 = Decimal::from_f64(closes[out_size-period-2] + highs[out_size-period-2] + lows[out_size-period-2]).unwrap() / dec!(3);
          let p2 = Decimal::from_f64(closes[out_size-period-1] + highs[out_size-period-1] + lows[out_size-period-1]).unwrap() / dec!(3);
          let p3 = Decimal::from_f64(closes[out_size-period] + highs[out_size-period] + lows[out_size-period]).unwrap() / dec!(3);
          let b1 = (p1 - Decimal::from_f64(out_lbands[out_size-3]).unwrap()) / Decimal::from_f64(out_ubands[out_size-3] - out_lbands[out_size-3]).unwrap();
          let b2 = (p2 - Decimal::from_f64(out_lbands[out_size-2]).unwrap()) / Decimal::from_f64(out_ubands[out_size-2] - out_lbands[out_size-2]).unwrap();
          let b3 = (p3 - Decimal::from_f64(out_lbands[out_size-1]).unwrap()) / Decimal::from_f64(out_ubands[out_size-1] - out_lbands[out_size-1]).unwrap();
          let w1 = Decimal::from_f64(out_ubands[out_size-3] - out_lbands[out_size-3]).unwrap() / Decimal::from_f64(out_mbands[out_size-3]).unwrap();
          let w2 = Decimal::from_f64(out_ubands[out_size-2] - out_lbands[out_size-2]).unwrap() / Decimal::from_f64(out_mbands[out_size-3]).unwrap();
          let w3 = Decimal::from_f64(out_ubands[out_size-1] - out_lbands[out_size-1]).unwrap() / Decimal::from_f64(out_mbands[out_size-3]).unwrap();

          result = format!(
            "{},{},{},{},{},{},{},{}",
            b1,
            b2,
            b3,
            w1,
            w2,
            w3,
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
    let is_exists: bool = rdb.exists(&redis_key).await.unwrap();
    rdb.hset(
      &redis_key,
      "bbands",
      result,
    ).await?;
    if !is_exists {
      rdb.expire(&redis_key, ttl.as_secs().try_into().unwrap()).await?;
    }

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
    let symbol = symbol.as_ref();
    let interval = interval.as_ref();

    let pool = ctx.pool.read().await;
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

      last_timestamp = timestamp;
      avg_prices.push(avg_price.to_f64().unwrap());
    }

    let dt = DateTime::from_timestamp_millis(first_timestamp).unwrap();
    if dt.format("%m%d").to_string() != Utc::now().format("%m%d").to_string() {
      return Err(Box::from(format!("[{symbol:}] {interval:} timestamp is not today")))
    }
    let day = Local::now().format("%m%d").to_string();

    let tenkan_period = tenkan_period as usize;
    let kijun_period = kijun_period as usize;
    let senkou_period = senkou_period as usize;

    let mut last_conversion_prices = avg_prices[2..tenkan_period].to_vec();
    last_conversion_prices.splice(0..0, vec![avg_prices[1]]);

    let mut last_base_prices = avg_prices[2..kijun_period].to_vec();
    last_base_prices.splice(0..0, vec![avg_prices[1]]);

    let mut conversion_prices = avg_prices[1..tenkan_period].to_vec();
    conversion_prices.splice(0..0, vec![avg_prices[0]]);

    let mut base_prices = avg_prices[1..kijun_period].to_vec();
    base_prices.splice(0..0, vec![avg_prices[0]]);

    let last_conversion_line = Decimal::from_f64(last_conversion_prices.iter().sum::<f64>()).unwrap() / Decimal::from_usize(last_conversion_prices.len()).unwrap();
    let last_base_line = Decimal::from_f64(last_base_prices.iter().sum::<f64>()).unwrap() / Decimal::from_usize(last_base_prices.len()).unwrap();
    let conversion_line = Decimal::from_f64(conversion_prices.iter().sum::<f64>()).unwrap() / Decimal::from_usize(conversion_prices.len()).unwrap();
    let base_line = Decimal::from_f64(base_prices.iter().sum::<f64>()).unwrap() / Decimal::from_usize(base_prices.len()).unwrap();

    let mut senkou_prices = avg_prices[1..senkou_period].to_vec();
    senkou_prices.splice(0..0, vec![avg_prices[0]]);

    let mut chikou_prices = avg_prices[1..kijun_period].to_vec();
    chikou_prices.splice(0..0, vec![avg_prices[0]]);
    let chikou_span_min = chikou_prices.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let chikou_span_max = chikou_prices.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();

    let senkou_span_a = (conversion_line + base_line) / dec!(2);
    let senkou_span_b = Decimal::from_f64(senkou_prices.iter().sum::<f64>()).unwrap() / Decimal::from_usize(senkou_prices.len()).unwrap();
    let chikou_span = Decimal::from_f64(chikou_span_min + chikou_span_max).unwrap() / dec!(2);

    let mut signal: i32 = 0;
    if conversion_line > base_line && last_conversion_line < last_base_line {
      signal = 1
    }
    if conversion_line < base_line && last_conversion_line > last_base_line {
      signal = 2
    }

    let ttl = Duration::from_secs(30+86400);

    let mut rdb = ctx.rdb.lock().await.clone();
    let redis_key = format!("{}:{}:{}:{}", Config::REDIS_KEY_INDICATORS, interval, symbol, day);

    if signal == 0 {
      let value: Option<String> = rdb.hget(&redis_key, "ichimoku_cloud").await?;
      match value {
        Some(value) => {
          let data: Vec<&str> = value.split(",").collect();
          let last_conversion_line = data[1].parse::<f64>().unwrap();
          let last_base_line = data[2].parse::<f64>().unwrap();
          if conversion_line > base_line && last_conversion_line < last_base_line {
            signal = 1
          }
          if conversion_line < base_line && last_conversion_line > last_base_line {
            signal = 2
          }
        },
        None => {},
      };
    }

    let tick_size: f64;
    match Self::filters(ctx.clone(), symbol).await {
      Ok(data) => {
        (tick_size, _) = data;
      },
      Err(e) => return Err(e.into()),
    }

    let tick_size = Decimal::from_f64(tick_size).unwrap();
    let first_avg_price = Decimal::from_f64(first_avg_price).unwrap();
    let conversion_line = (conversion_line / tick_size).floor() * tick_size;
    let base_line = (base_line / tick_size).floor() * tick_size;
    let senkou_span_a = (senkou_span_a / tick_size).floor() * tick_size;
    let senkou_span_b = (senkou_span_b / tick_size).ceil() * tick_size;
    let chikou_span = (chikou_span / tick_size).floor() * tick_size;
    let first_avg_price = (first_avg_price / tick_size).floor() * tick_size;
    let result = format!(
      "{},{},{},{},{},{},{},{}",
      signal,
      conversion_line,
      base_line,
      senkou_span_a,
      senkou_span_b,
      chikou_span,
      first_avg_price,
      first_timestamp,
    );

    let is_exists: bool = rdb.exists(&redis_key).await.unwrap();
    rdb.hset(
      &redis_key,
      "ichimoku_cloud",
      result.to_string(),
    ).await?;
    if !is_exists {
      rdb.expire(&redis_key, ttl.as_secs().try_into().unwrap()).await?;
    }
    println!("result {result:}");

    Ok(())
  }

  pub async fn volume_profile<T>(
    ctx: Ctx,
    symbol: T,
    interval: T,
    limit: i64,
  ) -> Result<(), Box<dyn std::error::Error>> 
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();
    let interval = interval.as_ref();

    let pool = ctx.pool.read().await;
    let mut conn = pool.get().unwrap();

    let items = klines::table
      .select((klines::close, klines::high, klines::low, klines::volume, klines::timestamp))
      .filter(klines::symbol.eq(symbol))
      .filter(klines::interval.eq(interval))
      .order(klines::timestamp.desc())
      .limit(limit)
      .load::<(f64, f64, f64, f64, i64)>(&mut conn)?;

    if items.len() < limit as usize {
      return Err(Box::from(format!("[{symbol:}] {interval:} klines not enough")))
    }

    let mut avg_prices: Vec<TA_Real> = Vec::new();
    let mut volumes: Vec<TA_Real> = Vec::new();
    let mut offsets: Vec<usize> = Vec::new();
    let mut first_timestamp: i64 = 0;
    let mut last_timestamp: i64 = 0;
    let current_timestamp = Self::timestamp(interval);

    for (close, high, low, volume, timestamp) in items {
      if first_timestamp == 0 {
        if timestamp < current_timestamp - 60000 {
          return Err(Box::from(format!("[{symbol:}] waiting for {interval:} klines flush")))
        }
        first_timestamp = timestamp;
      }

      if last_timestamp > 0 && last_timestamp != timestamp + Self::timestep(interval) {
        return Err(Box::from(format!("[{symbol:}] {interval:} klines lost")))
      }

      let close = Decimal::from_f64(close).unwrap();
      let high = Decimal::from_f64(high).unwrap();
      let low = Decimal::from_f64(low).unwrap();
      let avg_price = (close + high + low) / dec!(3);
      let avg_price = avg_price.to_f64().unwrap();

      let dt = DateTime::from_timestamp_millis(timestamp).unwrap();
      let mut offset = dt.hour() * 2 + 1;
      if dt.minute() > 30 {
        offset += 1;
      }

      avg_prices.splice(0..0, vec![avg_price]);
      volumes.splice(0..0, vec![volume]);
      offsets.splice(0..0, vec![offset as usize]);

      last_timestamp = timestamp;
    }

    let dt = DateTime::from_timestamp_millis(first_timestamp).unwrap();
    if dt.format("%m%d").to_string() != Utc::now().format("%m%d").to_string() {
      return Err(Box::from(format!("[{symbol:}] {interval:} timestamp is not today")))
    }
    let day = Local::now().format("%m%d").to_string();

    let avg_prices_min = avg_prices.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let avg_prices_max = avg_prices.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();

    if avg_prices_min == avg_prices_max {
      return Err(Box::from(format!("[{symbol:}] {interval:} klines avg prices invalid")))
    }

    println!("avg prices {} {}", avg_prices_min, avg_prices_max);

    let total_volumes = Decimal::from_f64(volumes.iter().sum::<f64>()).unwrap();

    let base_volume = total_volumes * dec!(0.7);
    let base_volume = base_volume.to_f64().unwrap();
    let segment_price = Decimal::from_f64(avg_prices_max - avg_prices_min).unwrap() / dec!(100);

    let mut poc_index: usize = 0;
    let mut poc_volume: f64 = 0.0;

    let mut items = HashMap::<usize, VolumeSegment>::new();
    for (i, avg_price) in avg_prices.iter().enumerate() {
      let index = (Decimal::from_f64(avg_prices_max - *avg_price).unwrap() / segment_price).floor();
      let mut index = index.to_usize().unwrap();
      if index > 99 {
        index = 99
      }
      match items.get_mut(&index) {
        Some(item) => {
          if item.prices[0] > *avg_price {
            item.prices[0] = *avg_price;
          }
          if item.prices[1] < *avg_price {
            item.prices[1] = *avg_price;
          }

          if let Some(_) = item.offsets.iter().position(|&r| r == offsets[i]) {
            item.volume += volumes[i];
          } else {
            item.offsets.push(offsets[i]);
            item.volume += volumes[i];
          }

          if poc_volume < item.volume {
            poc_index = index;
            poc_volume = item.volume;
          }
        },
        None => {
          items.insert(index, VolumeSegment::new(
            vec![*avg_price, *avg_price],
            vec![offsets[i]],
            volumes[i],
          ));
          poc_index = index;
          poc_volume = volumes[i];
        }
      };
    }
    println!("poc {poc_index:} {poc_volume:}");

    let mut start_index: usize = 0;
    let mut end_index: usize = 0;
    let mut best_volume: f64 = 0.0;

    for i in 0..100 {
      if items.get(&i).is_none() {
        continue;
      }
      let mut area_volume: f64 = 0.0;
      for j in i..100 {
        if let Some(item) = items.get(&j) {
          area_volume += item.volume;
          if area_volume > base_volume {
            if best_volume < area_volume {
              start_index = i;
              end_index = j;
              best_volume = area_volume;
            }
            break
          }
        }
      }
    }

    if best_volume == 0.0 {
      return Err(Box::from(format!("[{symbol:}] {interval:} best volume not exists")))
    }

    let tick_size: f64;
    match Self::filters(ctx.clone(), symbol).await {
      Ok(data) => {
        (tick_size, _) = data;
      },
      Err(e) => return Err(e.into()),
    }

    let tick_size = Decimal::from_f64(tick_size).unwrap();

    let item = items.get(&poc_index).unwrap();
    let poc = Decimal::from_f64(item.prices.iter().sum::<f64>()).unwrap() / Decimal::from_usize(item.prices.len()).unwrap();

    let start_item = items.get(&start_index).unwrap();
    let start_avg_price = Decimal::from_f64(start_item.prices.iter().sum::<f64>()).unwrap() / Decimal::from_usize(start_item.prices.len()).unwrap();

    let end_item = items.get(&end_index).unwrap();
    let end_avg_price = Decimal::from_f64(end_item.prices.iter().sum::<f64>()).unwrap() / Decimal::from_usize(end_item.prices.len()).unwrap();

    let vah = start_avg_price.max(end_avg_price);
    let val = start_avg_price.min(end_avg_price);

    let poc_ratio = ((vah - val) / poc).round_dp(4);

    let poc = (poc / tick_size).floor() * tick_size;
    let vah = (vah / tick_size).ceil() * tick_size;
    let val = (val / tick_size).floor() * tick_size;

    println!("poc {} {} {} {}", poc, vah, val, poc_ratio);

    let ttl = Duration::from_secs(30+86400);

    let mut rdb = ctx.rdb.lock().await.clone();
    let redis_key = format!("{}:{}:{}:{}", Config::REDIS_KEY_INDICATORS, interval, symbol, day);
    let is_exists: bool = rdb.exists(&redis_key).await.unwrap();
    rdb.hset_multiple(
      &redis_key,
      &[
        ("vah", vah.to_string()),
        ("val", val.to_string()),
        ("poc", poc.to_string()),
        ("poc_ratio", poc_ratio.to_string()),
      ],
    ).await?;
    if !is_exists {
      rdb.expire(&redis_key, ttl.as_secs().try_into().unwrap()).await?;
    }

    Ok(())
  }

  pub async fn andean_oscillator<T>(
    ctx: Ctx,
    symbol: T,
    interval: T,
    period: i32,
    length: i32,
    limit: i64,
  ) -> Result<(), Box<dyn std::error::Error>> 
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();
    let interval = interval.as_ref();

    let pool = ctx.pool.read().await;
    let mut conn = pool.get().unwrap();

    let items = klines::table
      .select((klines::open, klines::close, klines::timestamp))
      .filter(klines::symbol.eq(symbol))
      .filter(klines::interval.eq(interval))
      .order(klines::timestamp.desc())
      .limit(limit)
      .load::<(f64, f64, i64)>(&mut conn)?;

    if items.len() < limit as usize {
      return Err(Box::from(format!("[{symbol:}] {interval:} klines not enough")))
    }

    let mut opens: Vec<TA_Real> = Vec::new();
    let mut closes: Vec<TA_Real> = Vec::new();
    let mut first_timestamp: i64 = 0;
    let mut last_timestamp: i64 = 0;
    let current_timestamp = Self::timestamp(interval);

    for (open, close, timestamp) in items {
      if first_timestamp == 0 {
        if timestamp < current_timestamp - 60000 {
          return Err(Box::from(format!("[{symbol:}] waiting for {interval:} klines flush")))
        }
        first_timestamp = timestamp;
      }

      if last_timestamp > 0 && last_timestamp != timestamp + Self::timestep(interval) {
        return Err(Box::from(format!("[{symbol:}] {interval:} klines lost")))
      }

      opens.splice(0..0, vec![open]);
      closes.splice(0..0, vec![close]);

      last_timestamp = timestamp;
    }

    let dt = DateTime::from_timestamp_millis(first_timestamp).unwrap();
    if dt.format("%m%d").to_string() != Utc::now().format("%m%d").to_string() {
      return Err(Box::from(format!("[{symbol:}] {interval:} timestamp is not today")))
    }
    let day = Local::now().format("%m%d").to_string();

    let size = closes.len();

    let mut up1: Vec<f64> = Vec::with_capacity(size);
    let mut up2: Vec<f64> = Vec::with_capacity(size);
    let mut dn1: Vec<f64> = Vec::with_capacity(size);
    let mut dn2: Vec<f64> = Vec::with_capacity(size);
    let mut bulls: Vec<f64> = Vec::with_capacity(size);
    let mut bears: Vec<f64> = Vec::with_capacity(size);
    let mut signals: Vec<f64> = Vec::with_capacity(size);

    let close = Decimal::from_f64(closes[0]).unwrap();
    let pow_close = close.powd(dec!(2));

    let close = close.to_f64().unwrap();
    let pow_close = pow_close.to_f64().unwrap();

    up1.push(close);
    up2.push(pow_close);
    dn1.push(close);
    dn2.push(pow_close);
    bulls.push(0.0);
    bears.push(0.0);
    signals.push(close);

    let alpha = dec!(2) / Decimal::from_i32(period + 1).unwrap();
    let appha_signal = dec!(2) / Decimal::from_i32(length + 1).unwrap();

    for i in 1..size {
      let close = Decimal::from_f64(closes[i]).unwrap();
      let pow_close = close.powd(dec!(2));
      let open = Decimal::from_f64(opens[i]).unwrap();
      let pow_open = open.powd(dec!(2));
      let last_up1 = Decimal::from_f64(up1[i-1]).unwrap();
      let last_up2 = Decimal::from_f64(up2[i-1]).unwrap();
      let last_dn1 = Decimal::from_f64(dn1[i-1]).unwrap();
      let last_dn2 = Decimal::from_f64(dn2[i-1]).unwrap();
      let last_signal = Decimal::from_f64(signals[i-1]).unwrap();

      let value = last_up1 - alpha * (last_up1 - close);
      let value = close.max(open).max(value).to_f64().unwrap();
      up1.push(value);

      let value = last_up2 - alpha * (last_up2 - pow_close);
      let value = pow_close.max(pow_open).max(value).to_f64().unwrap();
      up2.push(value);

      let value = last_dn1 + alpha * (close - last_dn1);
      let value = close.min(open).min(value).to_f64().unwrap();
      dn1.push(value);

      let value = last_dn2 + alpha * (pow_close - last_dn2);
      let value = pow_close.min(pow_open).min(value).to_f64().unwrap();
      dn2.push(value);

      let value = Decimal::from_f64(dn2[i]).unwrap() - Decimal::from_f64(dn1[i]).unwrap().powd(dec!(2));
      if value < dec!(0) {
        return Err(Box::from(format!("[{symbol:}] {interval:} bears not valid {value:}")))
      }
      let value = value.sqrt().unwrap().to_f64().unwrap();
      bulls.push(value);

      let value = Decimal::from_f64(up2[i]).unwrap() - Decimal::from_f64(up1[i]).unwrap().powd(dec!(2));
      if value < dec!(0) {
        return Err(Box::from(format!("[{symbol:}] {interval:} bears not valid {value:}")))
      }
      let value = value.sqrt().unwrap().to_f64().unwrap();
      bears.push(value);

      let value = last_signal + appha_signal * (Decimal::from_f64(bulls[i].max(bears[i])).unwrap() - last_signal);
      let value = value.to_f64().unwrap();
      signals.push(value);
    }

    let ttl = Duration::from_secs(30+86400);

    let mut rdb = ctx.rdb.lock().await.clone();
    let redis_key = format!("{}:{}:{}:{}", Config::REDIS_KEY_INDICATORS, interval, symbol, day);
    let is_exists: bool = rdb.exists(&redis_key).await.unwrap();
    rdb.hset_multiple(
      &redis_key,
      &[
        ("ao_bull", bulls.last().unwrap().to_string()),
        ("ao_bear", bears.last().unwrap().to_string()),
        ("ao_signal", signals.last().unwrap().to_string()),
      ],
    ).await?;
    if !is_exists {
      rdb.expire(&redis_key, ttl.as_secs().try_into().unwrap()).await?;
    }

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
