use std::ops::Sub;
use std::time::Duration;

use talib_sys::{TA_Integer, TA_Real, TA_ATR,  TA_RetCode};

use diesel::prelude::*;
use diesel::query_builder::QueryFragment;
use diesel::ExpressionMethods;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use chrono::{prelude::Utc, Timelike};

use crate::common::*;
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

    let mut closes: Vec<TA_Real> = Vec::new();
    let mut highs: Vec<TA_Real> = Vec::new();
    let mut lows: Vec<TA_Real> = Vec::new();
    let mut last_timestamp: i64 = 0;

    let size = items.len();

    for (close, high, low, timestamp) in items {
      if last_timestamp == 0 && timestamp < Self::timestamp(interval) - 60000 {
        return Err(Box::from(format!("[{symbol:}] waiting fo {interval:} klines flush")))
      }
      if last_timestamp > 0 && last_timestamp != timestamp + Self::timestep(interval) {
        return Err(Box::from(format!("[{symbol:}] {interval:} klines lost")))
      }
      closes.splice(0..0, vec![close]);
      highs.splice(0..0, vec![high]);
      lows.splice(0..0, vec![low]);
      last_timestamp = timestamp;
    }

    if size < limit as usize {
      return Err(Box::from(format!("[{symbol:}] {interval:} klines not enough")))
    }

    let mut out: Vec<TA_Real> = Vec::with_capacity(size);
    let mut out_begin: TA_Integer = 0;
    let mut out_size: TA_Integer = 0;

    unsafe {
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
      match ret_code {
        TA_RetCode::TA_SUCCESS => out.set_len(out_size as usize),
        _ => panic!("Could not compute indicator, err: {:?}", ret_code)  
      }
    }

    println!("result {:?}", out);

    Ok(())
  }

  pub async fn zlema<T>(
    ctx: Ctx,
    symbol: T,
    interval: T,
    period: i32,
    limit: i32,
  ) -> Result<(), Box<dyn std::error::Error>> 
  where
    T: AsRef<str>
  {
    Ok(())
  }

  pub async fn ha_zlema<T>(
    ctx: Ctx,
    symbol: T,
    interval: T,
    period: i32,
    limit: i32,
  ) -> Result<(), Box<dyn std::error::Error>> 
  where
    T: AsRef<str>
  {
    Ok(())
  }

  pub async fn kdj<T>(
    ctx: Ctx,
    symbol: T,
    interval: T,
    long_period: i32,
    short_period: i32,
    limit: i32,
  ) -> Result<(), Box<dyn std::error::Error>> 
  where
    T: AsRef<str>
  {
    Ok(())
  }

  pub async fn bbands<T>(
    ctx: Ctx,
    symbol: T,
    interval: T,
    period: i32,
    limit: i32,
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
    limit: i32,
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
}
