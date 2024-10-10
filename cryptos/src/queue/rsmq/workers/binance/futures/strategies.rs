use std::time::Duration;

use rsmq_async::RsmqConnection;
use tokio::task::JoinSet;

use crate::common::*;
use crate::config::binance::futures::config as Config;
use crate::queue::rsmq::payload::binance::futures::strategies::*;
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

    if let Err(err) = StrategiesRepository::atr(ctx.clone(), symbol, interval).await {
      return Err(err.into())
    }

    Ok(())
  }

  pub async fn zlema<T>(ctx: Ctx, symbol: T, interval: T) -> Result<(), Box<dyn std::error::Error>>
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();
    let interval = interval.as_ref();

    if let Err(err) = StrategiesRepository::zlema(ctx.clone(), symbol, interval).await {
      return Err(err.into())
    }

    Ok(())
  }

  pub async fn ha_zlema<T>(ctx: Ctx, symbol: T, interval: T) -> Result<(), Box<dyn std::error::Error>>
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();
    let interval = interval.as_ref();

    if let Err(err) = StrategiesRepository::ha_zlema(ctx.clone(), symbol, interval).await {
      return Err(err.into())
    }

    Ok(())
  }

  pub async fn kdj<T>(ctx: Ctx, symbol: T, interval: T) -> Result<(), Box<dyn std::error::Error>>
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();
    let interval = interval.as_ref();

    if let Err(err) = StrategiesRepository::kdj(ctx.clone(), symbol, interval).await {
      return Err(err.into())
    }

    Ok(())
  }

  pub async fn bbands<T>(ctx: Ctx, symbol: T, interval: T) -> Result<(), Box<dyn std::error::Error>>
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();
    let interval = interval.as_ref();

    if let Err(err) = StrategiesRepository::bbands(ctx.clone(), symbol, interval).await {
      return Err(err.into())
    }

    Ok(())
  }

  pub async fn ichimoku_cloud<T>(ctx: Ctx, symbol: T, interval: T) -> Result<(), Box<dyn std::error::Error>>
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();
    let interval = interval.as_ref();

    if let Err(err) = StrategiesRepository::ichimoku_cloud(ctx.clone(), symbol, interval).await {
      return Err(err.into())
    }

    Ok(())
  }

  pub async fn flush<T>(ctx: Ctx, symbol: T, interval: T) -> Result<(), Box<dyn std::error::Error>> 
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

    println!("binance futures strategies rsmq workers process {symbol:} {interval:}");
    Self::atr(ctx.clone(), symbol, interval).await?;
    Self::zlema(ctx.clone(), symbol, interval).await?;
    Self::ha_zlema(ctx.clone(), symbol, interval).await?;
    Self::kdj(ctx.clone(), symbol, interval).await?;
    Self::bbands(ctx.clone(), symbol, interval).await?;
    Self::ichimoku_cloud(ctx.clone(), symbol, interval).await?;

    mutex.unlock().await.unwrap();
    Ok(())
  }

  pub async fn subscribe(&self, workers: &mut JoinSet<()>) -> Result<(), Box<dyn std::error::Error>> 
  {
    println!("binance futures strategies rsmq workers subscribe");
    workers.spawn(Box::pin({
      let ctx = self.ctx.clone();
      async move {
        let rmq = ctx.rmq.lock().await.clone();
        let mut client = Rsmq::new(rmq).await.unwrap();
        loop {
          println!("binance futures strategies rsmq loop");
          match client.pop_message::<String>(Config::RSMQ_QUEUE_STRATEGIES).await {
            Ok(Some(message)) => {
              let (action, content) = serde_json::from_slice::<(String, String)>(message.message.as_bytes()).unwrap();
              match action.as_str() {
                Config::RSMQ_JOBS_STRATEGIES_FLUSH => {
                  let payload = serde_json::from_slice::<StrategiesFlushPayload<&str>>(content.as_bytes()).unwrap();
                  if let Err(err) = Self::flush(ctx.clone(), payload.symbol, payload.interval).await {
                    println!("{err:?}");
                  }
                }
                _ => (),
              };
            }
            Ok(None) => {
              tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            }
            Err(_) => ()
          }
        }
      }
    }));
    Ok(())
  }
}