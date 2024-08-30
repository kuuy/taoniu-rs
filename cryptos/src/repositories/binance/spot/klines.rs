use std::ops::Sub;
use std::time::Duration;

use diesel::prelude::*;
use diesel::ExpressionMethods;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use chrono::{prelude::Utc, Timelike};

use crate::common::*;
use crate::models::binance::spot::kline::*;
use crate::schema::binance::spot::klines::*;

#[derive(Default)]
pub struct KlinesRepository {}

impl KlinesRepository
{
  pub async fn get<T>(
    ctx: Ctx,
    symbol: T,
    interval: T,
    timestamp: i64,
  ) -> Result<(), Box<dyn std::error::Error>>
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();
    let interval = interval.as_ref();
    let pool = ctx.pool.read().unwrap();
    let mut conn = pool.get().unwrap();
    let kline = klines::table
      .select(Kline::as_select())
      .filter(klines::symbol.eq(symbol))
      .filter(klines::interval.eq(interval))
      .filter(klines::timestamp.eq(timestamp))
      .first(&mut conn)?;
    Ok(())
  }

  pub async fn gets<T>(
    ctx: Ctx,
    symbols: Vec<T>,
    fields: Vec<T>,
    interval: T,
    timestamp: i64,
  ) -> Vec<Vec<String>> 
  where
    T: AsRef<str>
  {
    let symbols = symbols.iter().map(|s| s.as_ref()).collect::<Vec<&str>>();
    let fields = fields.iter().map(|s| s.as_ref()).collect::<Vec<&str>>();
    let interval = interval.as_ref();
    let script = redis::Script::new(r"
      local hmget = function (key)
        local hash = {}
        local data = redis.call('HMGET', key, unpack(ARGV, 3))
        for i = 1, #ARGV do
          hash[i] = data[i]
        end
        return hash
      end
      local data = {}
      for i = 1, #KEYS do
        local key = 'binance:spot:klines:' .. ARGV[1] .. ':' .. KEYS[i] .. ':' .. ARGV[2]
        if redis.call('EXISTS', key) == 0 then
          data[i] = false
        else
          data[i] = hmget(key)
        end
      end
      return data
    ");
    let mut rdb = ctx.rdb.lock().await.clone();
    let mut vars = Vec::new();
    match script
      .key(symbols.as_slice())
      .arg(interval)
      .arg(timestamp)
      .arg(fields.as_slice())
      .invoke_async::<_, Vec<redis::Value>>(&mut rdb).await {
      Ok(values) => {
        values.iter().enumerate().for_each(|(i, value)| {
          if let redis::Value::Bulk(bulk) = value {
            let mut var = Vec::new();
            bulk.iter().for_each(|item| {
              if let redis::Value::Data(v) = item {
                let v = std::str::from_utf8(v).unwrap();
                var.push(v.to_string());
              }
            });
            vars.push(var.clone());
          } else {
            vars.push(Vec::new());
          }
        })
      }
      _ => {},
    }
    vars
  }

  pub async fn flush<T>(
    ctx: Ctx,
    symbols: Vec<T>,
    interval: T,
    timestamp: i64,
  ) -> Result<(), Box<dyn std::error::Error>>
  where
    T: AsRef<str>
  {
    // let fields = ["open", "close", "high", "low", "volume", "quota"].to_vec();
    // let values = Self::gets(ctx, symbols, fields, interval, timestamp).await;
    // values.iter().enumerate().for_each(|(i, value)| {
    //   if value.len() == 0 {
    //     return None;
    //   }
    //   let symbol = symbols[i];
    //   println!("klines flush {} {} {}", symbol, interval, timestamp);
    // });
    Ok(())
  }

  pub async fn updates() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
  }

  pub fn timestamp<T>(interval: T) -> i64 
  where
    T: AsRef<str>
  {
    let interval = interval.as_ref();
    let mut datetime = Utc::now();
    datetime = datetime.sub(Duration::from_secs(datetime.second() as u64));
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
}
