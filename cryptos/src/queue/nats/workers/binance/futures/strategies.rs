use std::time::Duration;
use futures_util::StreamExt;
use tokio::task::JoinSet;

use crate::common::*;
use crate::config::binance::futures::config as Config;
use crate::queue::nats::payload::binance::futures::indicators::*;
use crate::queue::nats::jobs::binance::futures::strategies::*;
use crate::repositories::binance::futures::strategies::*;

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

    if let Err(e) = StrategiesRepository::atr(ctx.clone(), symbol, interval).await {
      return Err(e.into())
    }

    Ok(())
  }

  pub async fn zlema<T>(ctx: Ctx, symbol: T, interval: T) -> Result<(), Box<dyn std::error::Error>>
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();
    let interval = interval.as_ref();

    if let Err(e) = StrategiesRepository::zlema(ctx.clone(), symbol, interval).await {
      return Err(e.into())
    }

    Ok(())
  }

  pub async fn ha_zlema<T>(ctx: Ctx, symbol: T, interval: T) -> Result<(), Box<dyn std::error::Error>>
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();
    let interval = interval.as_ref();

    if let Err(e) = StrategiesRepository::ha_zlema(ctx.clone(), symbol, interval).await {
      return Err(e.into())
    }

    Ok(())
  }

  pub async fn kdj<T>(ctx: Ctx, symbol: T, interval: T) -> Result<(), Box<dyn std::error::Error>>
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();
    let interval = interval.as_ref();

    if let Err(e) = StrategiesRepository::kdj(ctx.clone(), symbol, interval).await {
      return Err(e.into())
    }

    let job = StrategiesJob::new(ctx.clone());
    let _ = job.update(symbol, interval).await;

    Ok(())
  }

  pub async fn bbands<T>(ctx: Ctx, symbol: T, interval: T) -> Result<(), Box<dyn std::error::Error>>
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();
    let interval = interval.as_ref();

    if let Err(e) = StrategiesRepository::bbands(ctx.clone(), symbol, interval).await {
      return Err(e.into())
    }

    Ok(())
  }

  pub async fn ichimoku_cloud<T>(ctx: Ctx, symbol: T, interval: T) -> Result<(), Box<dyn std::error::Error>>
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();
    let interval = interval.as_ref();

    if let Err(e) = StrategiesRepository::ichimoku_cloud(ctx.clone(), symbol, interval).await {
      return Err(e.into())
    }

    let job = StrategiesJob::new(ctx.clone());
    let _ = job.update(symbol, interval).await;

    Ok(())
  }

  pub async fn process<T>(ctx: Ctx, symbol: T, interval: T) -> Result<(), Box<dyn std::error::Error>> 
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();
    let interval = interval.as_ref();

    let rdb = ctx.rdb.lock().await.clone();
    let mutex_id = xid::new().to_string();
    let redis_lock_key = format!("{}:{}:{}", Config::LOCKS_STRATEGIES_FLUSH, interval, symbol);
    let mut mutex = Mutex::new(
      rdb,
      &redis_lock_key,
      &mutex_id,
    );
    if !mutex.lock(Duration::from_secs(30)).await.unwrap() {
      return Err(Box::from(format!("mutex failed {}", redis_lock_key)));
    }

    println!("binance futures strategies nats workers process {symbol:} {interval:}");
    Self::atr(ctx.clone(), symbol, interval).await?;
    Self::zlema(ctx.clone(), symbol, interval).await?;
    Self::ha_zlema(ctx.clone(), symbol, interval).await?;
    Self::kdj(ctx.clone(), symbol, interval).await?;
    Self::bbands(ctx.clone(), symbol, interval).await?;
    Self::ichimoku_cloud(ctx.clone(), symbol, interval).await?;

    mutex.unlock().await.unwrap();
    Ok(())
  }

  pub async fn subscribe(&self, workers: &mut JoinSet<()>) -> Result<(), Box<dyn std::error::Error>> {
    workers.spawn(Box::pin({
      let ctx = self.ctx.clone();
      let client = ctx.nats.clone();
      async move {
        println!("binance futures strategies nats workers subscribe");
        let mut subscriber = client.subscribe(Config::NATS_EVENTS_INDICATORS_UPDATE).await.unwrap();
        loop {
          if let Ok(Some(message)) = tokio::time::timeout(Duration::from_millis(100), subscriber.next()).await {
            if let Ok(payload) = serde_json::from_slice::<IndicatorsUpdatePayload<&str>>(message.payload.as_ref()) {
              if let Err(e) = Self::process(ctx.clone(), payload.symbol, payload.interval).await {
                println!("nats worders binance futures strategies process failed {} {} {:?}", payload.symbol, payload.interval, e);
              }
            }
          } else {
            println!("binance futures strategies nats workers sleep");
            tokio::time::sleep(Duration::from_millis(500)).await;
          }
        }
      }
    }));

    Ok(())
  }
}