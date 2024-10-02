use std::time::Duration;
use futures_util::StreamExt;

use crate::common::*;
use crate::config::binance::futures::config as Config;
use crate::queue::nats::payload::binance::futures::plans::*;
use crate::repositories::binance::futures::tradings::scalping::*;

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

    println!("binance futures tradings scalping nats workers process {plan_id:}");
    if let Err(e) = ScalpingRepository::place(ctx.clone(), plan_id).await {
      println!("binance futures tradings scalping {plan_id:} place failed {e:?}")
    }

    mutex.unlock().await.unwrap();

    Ok(())
  }

  pub async fn subscribe(&self) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance futures tradings scalping nats workers subscribe");

    tokio::spawn(Box::pin({
      let ctx = self.ctx.clone();
      async move {
        let client = ctx.nats.clone();
        let mut subscriber = client.subscribe(Config::NATS_EVENTS_PLANS_UPDATE).await.unwrap();
        while let Some(message) = subscriber.next().await {
          if let Ok(payload) = serde_json::from_slice::<PlansUpdatePayload<&str>>(message.payload.as_ref()) {
            if let Err(e) = Self::process(ctx.clone(), payload.id).await {
              println!("nats worders binance futures tradings scalping process failed {} {:?}", payload.id, e);
            }
          }
        }
      }
    }));

    Ok(())
  }
}