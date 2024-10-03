use std::time::Duration;
use futures_util::StreamExt;

use crate::common::*;
use crate::config::binance::spot::config as Config;
use crate::queue::nats::payload::binance::spot::indicators::*;
use crate::queue::nats::jobs::binance::spot::strategies::*;
use crate::repositories::binance::spot::strategies::*;

pub struct StrategiesWorker {
  ctx: Ctx,
}

impl StrategiesWorker {
  pub fn new(ctx: Ctx) -> Self {
    Self {
      ctx: ctx,
    }
  }

  pub async fn atr<T>(ctx: Ctx, symbol: T, interval: T) -> Result<(), Box<dyn std::error::Error>>
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();
    let interval = interval.as_ref();

    let rdb = ctx.rdb.lock().await.clone();
    let mutex_id = xid::new().to_string();
    let redis_lock_key = format!("{}:{}:atr:{}", Config::LOCKS_STRATEGIES_FLUSH, interval, symbol);
    let mut mutex = Mutex::new(
      rdb,
      &redis_lock_key,
      &mutex_id,
    );
    if !mutex.lock(Duration::from_secs(3)).await.unwrap() {
      return Err(Box::from(format!("mutex failed {}", redis_lock_key)));
    }

    if let Err(e) = StrategiesRepository::atr(ctx.clone(), symbol, interval).await {
      return Err(e.into())
    }

    mutex.unlock().await.unwrap();
    Ok(())
  }

  pub async fn zlema<T>(ctx: Ctx, symbol: T, interval: T) -> Result<(), Box<dyn std::error::Error>>
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();
    let interval = interval.as_ref();

    let rdb = ctx.rdb.lock().await.clone();
    let mutex_id = xid::new().to_string();
    let redis_lock_key = format!("{}:{}:zlema:{}", Config::LOCKS_STRATEGIES_FLUSH, interval, symbol);
    let mut mutex = Mutex::new(
      rdb,
      &redis_lock_key,
      &mutex_id,
    );
    if !mutex.lock(Duration::from_secs(5)).await.unwrap() {
      return Err(Box::from(format!("mutex failed {}", redis_lock_key)));
    }

    if let Err(e) = StrategiesRepository::zlema(ctx.clone(), symbol, interval).await {
      return Err(e.into())
    }

    mutex.unlock().await.unwrap();
    Ok(())
  }

  pub async fn ha_zlema<T>(ctx: Ctx, symbol: T, interval: T) -> Result<(), Box<dyn std::error::Error>>
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();
    let interval = interval.as_ref();

    let rdb = ctx.rdb.lock().await.clone();
    let mutex_id = xid::new().to_string();
    let redis_lock_key = format!("{}:{}:ha_zlema:{}", Config::LOCKS_STRATEGIES_FLUSH, interval, symbol);
    let mut mutex = Mutex::new(
      rdb,
      &redis_lock_key,
      &mutex_id,
    );
    if !mutex.lock(Duration::from_secs(5)).await.unwrap() {
      return Err(Box::from(format!("mutex failed {}", redis_lock_key)));
    }

    if let Err(e) = StrategiesRepository::ha_zlema(ctx.clone(), symbol, interval).await {
      return Err(e.into())
    }

    mutex.unlock().await.unwrap();
    Ok(())
  }

  pub async fn kdj<T>(ctx: Ctx, symbol: T, interval: T) -> Result<(), Box<dyn std::error::Error>>
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();
    let interval = interval.as_ref();

    let rdb = ctx.rdb.lock().await.clone();
    let mutex_id = xid::new().to_string();
    let redis_lock_key = format!("{}:{}:kdj:{}", Config::LOCKS_STRATEGIES_FLUSH, interval, symbol);
    let mut mutex = Mutex::new(
      rdb,
      &redis_lock_key,
      &mutex_id,
    );
    if !mutex.lock(Duration::from_secs(5)).await.unwrap() {
      return Err(Box::from(format!("mutex failed {}", redis_lock_key)));
    }

    if let Err(e) = StrategiesRepository::kdj(ctx.clone(), symbol, interval).await {
      return Err(e.into())
    }

    let job = StrategiesJob::new(ctx.clone());
    let _ = job.update(symbol, interval).await;

    mutex.unlock().await.unwrap();
    Ok(())
  }

  pub async fn bbands<T>(ctx: Ctx, symbol: T, interval: T) -> Result<(), Box<dyn std::error::Error>>
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();
    let interval = interval.as_ref();

    let rdb = ctx.rdb.lock().await.clone();
    let mutex_id = xid::new().to_string();
    let redis_lock_key = format!("{}:{}:bbands:{}", Config::LOCKS_STRATEGIES_FLUSH, interval, symbol);
    let mut mutex = Mutex::new(
      rdb,
      &redis_lock_key,
      &mutex_id,
    );
    if !mutex.lock(Duration::from_secs(5)).await.unwrap() {
      return Err(Box::from(format!("mutex failed {}", redis_lock_key)));
    }

    if let Err(e) = StrategiesRepository::bbands(ctx.clone(), symbol, interval).await {
      return Err(e.into())
    }

    mutex.unlock().await.unwrap();
    Ok(())
  }

  pub async fn ichimoku_cloud<T>(ctx: Ctx, symbol: T, interval: T) -> Result<(), Box<dyn std::error::Error>>
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();
    let interval = interval.as_ref();

    let rdb = ctx.rdb.lock().await.clone();
    let mutex_id = xid::new().to_string();
    let redis_lock_key = format!("{}:{}:ichimoku_cloud:{}", Config::LOCKS_STRATEGIES_FLUSH, interval, symbol);
    let mut mutex = Mutex::new(
      rdb,
      &redis_lock_key,
      &mutex_id,
    );
    if !mutex.lock(Duration::from_secs(5)).await.unwrap() {
      return Err(Box::from(format!("mutex failed {}", redis_lock_key)));
    }

    if let Err(e) = StrategiesRepository::ichimoku_cloud(ctx.clone(), symbol, interval).await {
      return Err(e.into())
    }

    let job = StrategiesJob::new(ctx.clone());
    let _ = job.update(symbol, interval).await;

    mutex.unlock().await.unwrap();
    Ok(())
  }

  pub async fn process<T>(ctx: Ctx, symbol: T, interval: T) -> Result<(), Box<dyn std::error::Error>> 
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();
    let interval = interval.as_ref();

    println!("binance spot strategies nats workers process {symbol:} {interval:}");
    Self::atr(ctx.clone(), symbol, interval).await?;
    Self::zlema(ctx.clone(), symbol, interval).await?;
    Self::ha_zlema(ctx.clone(), symbol, interval).await?;
    Self::kdj(ctx.clone(), symbol, interval).await?;
    Self::bbands(ctx.clone(), symbol, interval).await?;
    Self::ichimoku_cloud(ctx.clone(), symbol, interval).await?;

    Ok(())
  }

  pub async fn subscribe(&self) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance spot strategies nats workers subscribe");

    tokio::spawn(Box::pin({
      let ctx = self.ctx.clone();
      async move {
        let client = ctx.nats.clone();
        let mut subscriber = client.subscribe(Config::NATS_EVENTS_INDICATORS_UPDATE).await.unwrap();
        while let Some(message) = subscriber.next().await {
          if let Ok(payload) = serde_json::from_slice::<IndicatorsUpdatePayload<&str>>(message.payload.as_ref()) {
            if let Err(e) = Self::process(ctx.clone(), payload.symbol, payload.interval).await {
              println!("nats worders binance spot strategies process failed {} {} {:?}", payload.symbol, payload.interval, e);
            }
          }
        }
      }
    }));

    Ok(())
  }
}