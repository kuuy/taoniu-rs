use std::time::Duration;
use std::collections::HashMap;

use crate::common::*;
use crate::config::binance::spot::config as Config;
use crate::queue::nats::payload::binance::spot::strategies::*;
use crate::repositories::binance::spot::plans::*;

pub struct PlansWorker {}

impl PlansWorker {
  pub fn new(_: Ctx) -> Self {
    Self {}
  }

  pub async fn process<T>(ctx: Ctx, payload: T) -> Result<(), Box<dyn std::error::Error>> 
  where
    T: AsRef<str>
  {
    println!("binance spot plans nats workers process");
    let (symbol, interval) = match serde_json::from_str::<StrategiesUpdatePayload<&str>>(payload.as_ref()) {
      Ok(result) => {
        (result.symbol, result.interval)
      }
      Err(err) => return Err(err.into()),
    };

    let rdb = ctx.rdb.lock().await.clone();
    let mutex_id = xid::new().to_string();
    let redis_lock_key = format!("{}:{}:{}", Config::LOCKS_PLANS_FLUSH, interval, symbol);
    let mut mutex = RedisMutex::new(
      rdb,
      &redis_lock_key,
      &mutex_id,
    );
    if !mutex.lock(Duration::from_secs(30)).await.unwrap() {
      return Err(Box::from(format!("mutex failed {}", redis_lock_key)));
    }

    println!("binance spot plans nats workers process {symbol:} {interval:}");
    if let Err(err) = PlansRepository::flush(ctx.clone(), symbol, interval).await {
      println!("binance spot plans {symbol:} {interval:} flush failed {err:?}")
    }

    mutex.unlock().await.unwrap();

    Ok(())
  }

  pub async fn subscribe(&self, callbacks: &mut HashMap<&str, Vec<EventFn>>) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance spot plans nats workers subscribe");
    match callbacks.get_mut(Config::NATS_EVENTS_STRATEGIES_UPDATE) {
      Some(callback) => {
        callback.push(Box::new(|ctx, payload| Box::pin(Self::process(ctx, payload))))
      }
      None => {
        callbacks.insert(
          Config::NATS_EVENTS_STRATEGIES_UPDATE,
          vec![
            Box::new(|ctx, payload| Box::pin(Self::process(ctx, payload))),
          ],
        );
      }
    };

    Ok(())
  }
}