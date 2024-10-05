use std::time::Duration;
use std::collections::HashMap;

use crate::common::*;
use crate::config::binance::spot::config as Config;
use crate::queue::nats::payload::binance::spot::plans::*;
use crate::repositories::binance::spot::tradings::scalping::*;

pub struct ScalpingWorker {}

impl ScalpingWorker {
  pub fn new(_: Ctx) -> Self {
    Self {}
  }

  pub async fn process<T>(ctx: Ctx, payload: T) -> Result<(), Box<dyn std::error::Error>> 
  where
    T: AsRef<str>
  {
    println!("binance spot tradings scalping nats workers process");
    let plan_id = match serde_json::from_str::<PlansUpdatePayload<&str>>(payload.as_ref()) {
      Ok(result) => {
        if result.side != 1 {
          return Ok(())
        }
        result.id
      },
      Err(e) => return Err(e.into()),
    };

    let rdb = ctx.rdb.lock().await.clone();
    let mutex_id = xid::new().to_string();
    let redis_lock_key = format!("{}:{}", Config::LOCKS_TRADINGS_SCALPING_PLACE, plan_id);
    let mut mutex = Mutex::new(
      rdb,
      &redis_lock_key,
      &mutex_id,
    );
    if !mutex.lock(Duration::from_secs(30)).await.unwrap() {
      return Err(Box::from(format!("mutex failed {}", redis_lock_key)));
    }

    println!("binance spot tradings scalping nats workers process {plan_id:}");
    if let Err(e) = ScalpingRepository::place(ctx.clone(), plan_id).await {
      println!("binance spot tradings scalping {plan_id:} place failed {e:?}")
    }

    mutex.unlock().await.unwrap();

    Ok(())
  }

  pub async fn subscribe(&self, callbacks: &mut HashMap<&str, Vec<EventFn>>) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance spot tradings scalping nats workers subscribe");
    match callbacks.get_mut(Config::NATS_EVENTS_PLANS_UPDATE) {
      Some(callback) => {
        callback.push(Box::new(|ctx, payload| Box::pin(Self::process(ctx, payload))))
      },
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