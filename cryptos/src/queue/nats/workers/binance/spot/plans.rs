use std::time::Duration;
use futures_util::StreamExt;
use tokio::task::JoinSet;

use crate::common::*;
use crate::config::binance::spot::config as Config;
use crate::queue::nats::payload::binance::spot::strategies::*;
use crate::repositories::binance::spot::plans::*;

pub struct PlansWorker {
  ctx: Ctx,
}

impl PlansWorker {
  pub fn new(ctx: Ctx) -> Self {
    Self {
      ctx: ctx,
    }
  }

  pub async fn process<T>(ctx: Ctx, symbol: T, interval: T) -> Result<(), Box<dyn std::error::Error>> 
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();
    let interval = interval.as_ref();

    let rdb = ctx.rdb.lock().await.clone();
    let mutex_id = xid::new().to_string();
    let redis_lock_key = format!("{}:{}:{}", Config::LOCKS_PLANS_FLUSH, interval, symbol);
    let mut mutex = Mutex::new(
      rdb,
      &redis_lock_key,
      &mutex_id,
    );
    if !mutex.lock(Duration::from_secs(30)).await.unwrap() {
      return Err(Box::from(format!("mutex failed {}", redis_lock_key)));
    }

    println!("binance spot plans nats workers process {symbol:} {interval:}");
    if let Err(e) = PlansRepository::flush(ctx.clone(), symbol, interval).await {
      println!("binance spot plans {symbol:} {interval:} flush failed {e:?}")
    }

    mutex.unlock().await.unwrap();

    Ok(())
  }

  pub async fn subscribe(&self, workers: &mut JoinSet<()>) -> Result<(), Box<dyn std::error::Error>> {
    workers.spawn(Box::pin({
      let ctx = self.ctx.clone();
      let client = ctx.nats.clone();
      async move {
        println!("binance spot plans nats workers subscribe");
        let mut subscriber = client.subscribe(Config::NATS_EVENTS_STRATEGIES_UPDATE).await.unwrap();
        loop {
          if let Ok(Some(message)) = tokio::time::timeout(Duration::from_millis(100), subscriber.next()).await {
            if let Ok(payload) = serde_json::from_slice::<StrategiesUpdatePayload<&str>>(message.payload.as_ref()) {
              if let Err(e) = Self::process(ctx.clone(), payload.symbol, payload.interval).await {
                println!("nats worders binance spot plans process failed {} {} {:?}", payload.symbol, payload.interval, e);
              }
            }
          } else {
            println!("binance spot plans nats workers sleep");
            tokio::time::sleep(Duration::from_millis(500)).await;
          }
        }
      }
    }));

    Ok(())
  }
}