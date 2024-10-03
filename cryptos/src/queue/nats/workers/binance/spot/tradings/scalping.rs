use std::time::Duration;
use futures_util::StreamExt;
use tokio::task::JoinSet;

use crate::common::*;
use crate::config::binance::spot::config as Config;
use crate::queue::nats::payload::binance::spot::plans::*;
use crate::repositories::binance::spot::tradings::scalping::*;

pub struct ScalpingWorker {
  ctx: Ctx,
}

impl ScalpingWorker {
  pub fn new(ctx: Ctx) -> Self {
    Self {
      ctx: ctx,
    }
  }

  pub async fn process<T>(ctx: Ctx, plan_id: T) -> Result<(), Box<dyn std::error::Error>> 
  where
    T: AsRef<str>
  {
    let plan_id = plan_id.as_ref();

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

  pub async fn subscribe(&self, workers: &mut JoinSet<()>) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance spot tradings scalping nats workers subscribe");

    workers.spawn(Box::pin({
      let ctx = self.ctx.clone();
      let client = self.ctx.nats.clone();
      async move {
        let mut subscriber = client.subscribe(Config::NATS_EVENTS_PLANS_UPDATE).await.unwrap();
        while let Some(message) = subscriber.next().await {
          if let Ok(payload) = serde_json::from_slice::<PlansUpdatePayload<&str>>(message.payload.as_ref()) {
            if payload.side != 1 {
              continue
            }
            if let Err(e) = Self::process(ctx.clone(), payload.id).await {
              println!("nats worders binance spot tradings scalping process failed {} {:?}", payload.id, e);
            }
          }
        }
      }
    }));

    Ok(())
  }
}