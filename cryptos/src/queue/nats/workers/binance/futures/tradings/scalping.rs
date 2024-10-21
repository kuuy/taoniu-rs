use std::time::Duration;
use std::collections::HashMap;

use crate::common::*;
use crate::config::binance::futures::config as Config;
use crate::queue::nats::payload::binance::futures::plans::*;
use crate::repositories::binance::futures::tradings::scalping::*;

pub struct ScalpingWorker {}

impl ScalpingWorker {
  pub fn new(_: Ctx) -> Self {
    Self {}
  }

  pub async fn process<T>(ctx: Ctx, payload: T) -> Result<(), Box<dyn std::error::Error>> 
  where
    T: AsRef<str>
  {
    println!("binance futures tradings scalping nats workers process");
    let plan_id = match serde_json::from_str::<PlansUpdatePayload<&str>>(payload.as_ref()) {
      Ok(result) => {
        if result.side != 1 {
          return Ok(())
        }
        result.id
      }
      Err(err) => return Err(err.into()),
    };

    let rdb = ctx.rdb.lock().await.clone();
    let mutex_id = xid::new().to_string();
    let redis_lock_key = format!("{}:{}", Config::LOCKS_TRADINGS_SCALPING_PLACE, plan_id);
    let mut mutex = RedisMutex::new(
      rdb,
      &redis_lock_key,
      &mutex_id,
    );
    if !mutex.lock(Duration::from_secs(30)).await.unwrap() {
      return Err(Box::from(format!("mutex failed {}", redis_lock_key)));
    }

    println!("binance futures tradings scalping nats workers process {plan_id:}");
    if let Err(err) = ScalpingRepository::place(ctx.clone(), plan_id).await {
      println!("binance futures tradings scalping {plan_id:} place failed {err:?}")
    }

    mutex.unlock().await.unwrap();

    Ok(())
  }

  pub async fn subscribe(&self, callbacks: &mut HashMap<&str, Vec<EventFn>>) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance futures tradings scalping nats workers subscribe");
    match callbacks.get_mut(Config::NATS_EVENTS_PLANS_UPDATE) {
      Some(callback) => {
        callback.push(Box::new(|ctx, payload| Box::pin(Self::process(ctx, payload))))
      }
      None => {
        callbacks.insert(
          Config::NATS_EVENTS_PLANS_UPDATE,
          vec![
            Box::new(|ctx, payload| Box::pin(Self::process(ctx, payload))),
          ],
        );
      }
    };

    Ok(())
  }
}