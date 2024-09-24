use std::ops::Sub;
use std::time::Duration;

use diesel::prelude::*;
use diesel::query_builder::QueryFragment;
use diesel::ExpressionMethods;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use chrono::{prelude::Utc, Timelike};

use crate::common::*;
use crate::models::binance::spot::kline::*;
use crate::schema::binance::spot::klines::*;
use crate::queue::nats::jobs::binance::spot::klines::*;

#[derive(Default)]
pub struct KlinesRepository {}

impl KlinesRepository
{
  pub async fn get<T>(
    ctx: Ctx,
    symbol: T,
    interval: T,
    timestamp: i64,
  ) -> Result<Option<Kline>, Box<dyn std::error::Error>>
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();
    let interval = interval.as_ref();

    let pool = ctx.pool.read().await;
    let mut conn = pool.get().unwrap();

    match klines::table
      .select(Kline::as_select())
      .filter(klines::symbol.eq(symbol))
      .filter(klines::interval.eq(interval))
      .filter(klines::timestamp.eq(timestamp))
      .first(&mut conn) {
      Ok(kline) => Ok(Some(kline)),
      Err(diesel::result::Error::NotFound) => Ok(None),
      Err(e) => Err(e.into()),
    }
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
        values.iter().enumerate().for_each(|(_, value)| {
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

  pub async fn create(
    ctx: Ctx,
    id: String,
    symbol: String,
    interval: String,
    open: f64,
    close: f64,
    high: f64,
    low: f64,
    volume: f64,
    quota: f64,
    timestamp: i64,
  ) -> Result<bool, Box<dyn std::error::Error>> {
    let pool = ctx.pool.write().await;
    let mut conn = pool.get().unwrap();

    let now = Utc::now();
    let kline = Kline::new(
      id,
      symbol,
      interval,
      open,
      close,
      high,
      low,
      volume,
      quota,
      timestamp,
      now,
      now,
    );
    match diesel::insert_into(klines::table)
      .values(&kline)
      .execute(&mut conn) {
      Ok(effective_rows) => Ok(effective_rows > 0),
      Err(e) => Err(e.into()),
    }
  }

  pub async fn update<V>(
    ctx: Ctx,
    id: String,
    value: V,
  ) -> Result<bool, Box<dyn std::error::Error>> 
  where
    V: diesel::AsChangeset<Target = klines::table>,
    <V as diesel::AsChangeset>::Changeset: QueryFragment<diesel::pg::Pg>,
  {
    let pool = ctx.pool.write().await;
    let mut conn = pool.get().unwrap();
    match diesel::update(klines::table.find(id)).set(value).execute(&mut conn) {
      Ok(effective_rows) => Ok(effective_rows > 0),
      Err(e) => Err(e.into()),
    }
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
    let symbols = symbols.iter().map(|s| s.as_ref()).collect::<Vec<&str>>();
    let fields = ["open", "close", "high", "low", "volume", "quota"].to_vec();
    let interval = interval.as_ref();
    let values = Self::gets(ctx.clone(), symbols.clone(), fields, interval, timestamp).await;
    for (i, data) in values.iter().enumerate() {
      if data.len() == 0 {
        continue;
      }

      let symbol = symbols.clone()[i];
      let open = data[0].parse::<f64>().unwrap();
      let close = data[1].parse::<f64>().unwrap();
      let high = data[2].parse::<f64>().unwrap();
      let low = data[3].parse::<f64>().unwrap();
      let volume = data[4].parse::<f64>().unwrap();
      let quota = data[4].parse::<f64>().unwrap();

      let mut kline: Option<Kline> = None;
      match Self::get(ctx.clone(), symbol, interval, timestamp).await {
        Ok(Some(result)) => {
          kline = Some(result);
        },
        Ok(None) => {},
        Err(e) => return Err(e.into()),
      }

      let mut success = false;
      if kline.is_none() {
        let id = xid::new().to_string();
        match Self::create(
          ctx.clone(), 
          id,
          symbol.to_string(),
          interval.to_string(),
          open,
          close,
          high,
          low,
          volume,
          quota,
          timestamp,
        ).await {
          Ok(result) => {
            if result {
              success = result;
            }
            println!("binance spot kline {symbol:} {interval:} {timestamp:} create success {result:}");
          }
          Err(e) => {
            println!("binance spot kline {symbol:} {interval:} {timestamp:} create failed {e:?}")
          },
        }
      } else {
        match Self::update(
          ctx.clone(),
          kline.unwrap().id,
          (
            klines::open.eq(open),
            klines::close.eq(close),
            klines::high.eq(high),
            klines::low.eq(low),
            klines::volume.eq(volume),
            klines::quota.eq(quota),
          ),
        ).await {
          Ok(result) => {
            success = result;
            println!("binance spot kline {symbol:} {interval:} {timestamp:} update success {result:}");
          }
          Err(e) => {
            println!("binance spot kline {symbol:} {interval:} {timestamp:} update failed {e:?}")
          },
        }
      }

      if success {
        let job = KlinesJob::new(ctx.clone());
        let _ = job.update(symbol, interval).await;
      }
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
}
