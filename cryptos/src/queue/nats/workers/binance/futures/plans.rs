use std::time::Duration;
use futures_util::StreamExt;

use crate::common::*;
use crate::config::binance::futures::config as Config;
use crate::queue::nats::payload::binance::futures::indicators::*;
use crate::repositories::binance::futures::plans::*;

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

    println!("binance futures plans nats workers process {symbol:} {interval:}");
    if let Err(e) = PlansRepository::flush(ctx.clone(), symbol, interval).await {
      println!("binance futures plans {symbol:} {interval:} flush failed {e:?}")
    }

    mutex.unlock().await.unwrap();

    Ok(())
  }

  pub async fn subscribe(&self) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance futures plans nats workers subscribe");

    tokio::spawn(Box::pin({
      let ctx = self.ctx.clone();
      async move {
        let client = ctx.nats.clone();
        let mut subscriber = client.subscribe(Config::NATS_EVENTS_STRATEGIES_UPDATE).await.unwrap();
        while let Some(message) = subscriber.next().await {
          if let Ok(payload) = serde_json::from_slice::<IndicatorsUpdatePayload<&str>>(message.payload.as_ref()) {
            if let Err(e) = Self::process(ctx.clone(), payload.symbol, payload.interval).await {
              println!("nats worders binance futures plans process failed {} {} {:?}", payload.symbol, payload.interval, e);
            }
          }
        }
      }
    }));

    Ok(())
  }
}