use std::time::Duration;
use std::collections::HashMap;

use crate::common::*;
use crate::config::binance::futures::config as Config;
use crate::queue::nats::payload::binance::futures::indicators::*;
use crate::queue::nats::jobs::binance::futures::strategies::*;
use crate::repositories::binance::futures::strategies::*;

pub struct StrategiesWorker {}

impl StrategiesWorker {
  pub fn new(_: Ctx) -> Self {
    Self {}
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

  pub async fn process<T>(ctx: Ctx, payload: T) -> Result<(), Box<dyn std::error::Error>> 
  where
    T: AsRef<str>
  {
    println!("binance futures strategies nats workers process");
    let (symbol, interval) = match serde_json::from_str::<IndicatorsUpdatePayload<&str>>(payload.as_ref()) {
      Ok(result) => {
        (result.symbol, result.interval)
      },
      Err(e) => return Err(e.into()),
    };

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

  pub async fn subscribe(&self, callbacks: &mut HashMap<&str, Vec<EventFn>>) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance futures strategies nats workers subscribe");
    match callbacks.get_mut(Config::NATS_EVENTS_INDICATORS_UPDATE) {
      Some(callback) => {
        callback.push(Box::new(|ctx, payload| Box::pin(Self::process(ctx, payload))))
      },
      None => {
        callbacks.insert(
          Config::NATS_EVENTS_INDICATORS_UPDATE,
          vec![
            Box::new(|ctx, payload| Box::pin(Self::process(ctx, payload))),
          ],
        );
      }
    };

    Ok(())
  }
}