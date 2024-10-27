use url::Url;
use std::collections::HashMap;
use std::ops::Sub;
use std::time::Duration;

use chrono::{prelude::Utc, Timelike};
use diesel::prelude::*;
use diesel::query_builder::QueryFragment;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

use crate::common::*;
use crate::models::binance::spot::kline::*;
use crate::schema::binance::spot::klines::*;
use crate::queue::nats::jobs::binance::spot::klines::*;

#[derive(Default)]
pub struct KlinesRepository {}

impl KlinesRepository
{
  pub async fn series<T> (
    ctx: Ctx,
    symbol: T,
    interval: T,
    limit: i64,
  ) -> Result<Vec<(f64, f64, f64, f64, i64)>, Box<dyn std::error::Error>>
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();
    let interval = interval.as_ref();

    let pool = ctx.pool.read().await;
    let mut conn = pool.get().unwrap();

    let series = klines::table
      .select((klines::open, klines::high, klines::low, klines::close, klines::timestamp))
      .filter(klines::symbol.eq(symbol))
      .filter(klines::interval.eq(interval))
      .order(klines::timestamp.desc())
      .limit(limit)
      .load::<(f64, f64, f64, f64, i64)>(&mut conn)?;

    Ok(series)
  }

  pub async fn timelines<T>(
    ctx: Ctx,
    symbol: T,
    interval: T,
    timestamp: i64,
  ) -> Result<Vec<i64>, Box<dyn std::error::Error>>
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();
    let interval = interval.as_ref();

    let pool = ctx.pool.read().await;
    let mut conn = pool.get().unwrap();

    let timelines = klines::table
      .select(klines::timestamp)
      .filter(klines::symbol.eq(symbol))
      .filter(klines::interval.eq(interval))
      .filter(klines::timestamp.gt(timestamp))
      .order(klines::timestamp.desc())
      .load::<i64>(&mut conn)?;

    Ok(timelines)
  }

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
        Ok(result) => Ok(Some(result)),
        Err(diesel::result::Error::NotFound) => Ok(None),
        Err(err) => Err(err.into()),
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
      _ => {}
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
      Err(err) => Err(err.into()),
    }
  }

  pub async fn update<V>(
    ctx: Ctx,
    id: String,
    values: V,
  ) -> Result<bool, Box<dyn std::error::Error>> 
  where
    V: diesel::AsChangeset<Target = klines::table>,
    <V as diesel::AsChangeset>::Changeset: QueryFragment<diesel::pg::Pg>,
  {
    let pool = ctx.pool.write().await;
    let mut conn = pool.get().unwrap();
    match diesel::update(klines::table.find(id)).set(values).execute(&mut conn) {
      Ok(effective_rows) => Ok(effective_rows > 0),
      Err(err) => Err(err.into()),
    }
  }

  pub async fn flush<T>(
    ctx: Ctx,
    symbol: T,
    interval: T,
    endtime: i64,
    limit: i64,
  ) -> Result<(), Box<dyn std::error::Error>>
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();
    let interval = interval.as_ref();
    let endtime_val = endtime.to_string();
    let limit = limit.to_string();

    let mut params = HashMap::<&str, &str>::new();
    params.insert("symbol", symbol);
    params.insert("interval", interval);
    if endtime > 0 {
      params.insert("endTime", &endtime_val);
    }
    params.insert("limit", &limit);

    let url = Url::parse_with_params(format!("{}/api/v3/klines", Env::var("BINANCE_SPOT_API_ENDPOINT")).as_str(), &params)?;

    let client = reqwest::Client::new();
    let response = client.get(url)
      .timeout(Duration::from_secs(5))
      .send()
      .await?;

    let status_code = response.status();

    if status_code.is_client_error() {
      println!("response {}", response.text().await.unwrap());
      return Err(Box::from(format!("bad request: {}", status_code)))
    }

    if !status_code.is_success() {
      return Err(Box::from(format!("request error: {}", status_code)))
    }

    let klines = response.json::<Vec<(i64, String, String, String, String, String, i64, String, i64, String, String, String)>>().await.unwrap();
    for (timestamp, open, high, low, close, volume, _, quota, ..) in klines.iter() {
      let open = open.parse::<f64>().unwrap();
      let close = close.parse::<f64>().unwrap();
      let high = high.parse::<f64>().unwrap();
      let low = low.parse::<f64>().unwrap();
      let volume = volume.parse::<f64>().unwrap();
      let quota = quota.parse::<f64>().unwrap();
      let timestamp = *timestamp;

      let kline: Option<Kline> = match Self::get(ctx.clone(), symbol, interval, timestamp).await {
        Ok(Some(result)) => Some(result),
        Ok(None) => None,
        Err(err) => {
          println!("error {:?}", err);
          continue
        },
      };

      if kline.is_none() {
        let id = xid::new().to_string();
        Self::create(
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
        ).await?;
      } else {
        Self::update(
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
        ).await?;
      }
    }

    Ok(())
  }

  pub async fn fix<T>(
    ctx: Ctx,
    symbol: T,
    interval: T,
    offset: i64,
  ) -> Result<(), Box<dyn std::error::Error>>
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();
    let interval = interval.as_ref();

    let timestamp = Self::timestamp(interval);
    let timestep = Self::timestep(interval);

    let values = Self::timelines(ctx.clone(), symbol, interval, timestamp - offset * timestep).await?;

    println!("len {}, {}", values.len(), offset);
    if values.len() == offset as usize {
      return Ok(())
    }

    let mut i = -1;
    let mut j = -1;
    for value in values.iter() {
      let k = (timestamp - *value) / timestep;
      if k == j + 1 {
        if i != -1 {
          let limit  = std::cmp::max(j - i + 1, 100);
          let endtime = timestamp - (i - limit) * timestep;
          println!("klines fix {symbol:} {interval:} {endtime:} {limit:}");
          Self::flush(ctx.clone(), symbol, interval, endtime, limit).await?;
          return Ok(())
        }
      } else {
        if i == -1 {
          i = k;
        }
      }
      j = k;
    }

    if i != -1 && j != -1 {
      let limit  = std::cmp::max(j - i + 1, 100);
      let endtime = timestamp - (i - limit) * timestep;
      println!("klines fix {symbol:} {interval:} {endtime:} {limit:}");
      Self::flush(ctx.clone(), symbol, interval, endtime, limit).await?;
      return Ok(())
    }

    if j < offset - 1 {
      let limit  = offset - j - 1;
      let endtime = timestamp - (offset - limit) * timestep;
      println!("klines fix {symbol:} {interval:} {endtime:} {limit:}");
      Self::flush(ctx.clone(), symbol, interval, endtime, limit).await?;
    }

    Ok(())
  }

  pub async fn sync<T>(
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
        continue
      }

      let symbol = symbols.clone()[i];
      let open = data[0].parse::<f64>().unwrap();
      let close = data[1].parse::<f64>().unwrap();
      let high = data[2].parse::<f64>().unwrap();
      let low = data[3].parse::<f64>().unwrap();
      let volume = data[4].parse::<f64>().unwrap();
      let quota = data[4].parse::<f64>().unwrap();

      let kline: Option<Kline> = match Self::get(ctx.clone(), symbol, interval, timestamp).await {
        Ok(Some(result)) => Some(result),
        Ok(None) => None,
        Err(err) => return Err(err.into()),
      };

      let success;
      if kline.is_none() {
        let id = xid::new().to_string();
        success = match Self::create(
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
          Ok(result) => result,
          Err(err) => return Err(err.into()),
        }
      } else {
        success = match Self::update(
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
          Ok(result) => result,
          Err(err) => return Err(err.into()),
        }
      }

      if success {
        let job = KlinesJob::new(ctx.clone());
        let _ = job.update(symbol, interval).await;
      }
    }

    Ok(())
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
