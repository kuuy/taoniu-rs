use std::time::Duration;

use rsmq_async::RsmqConnection;
use tokio::task::JoinSet;

use crate::common::*;
use crate::config::binance::futures::config as Config;
use crate::queue::rsmq::payload::binance::futures::plans::*;
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

  pub async fn flush<T>(ctx: Ctx, symbol: T, interval: T) -> Result<(), Box<dyn std::error::Error>> 
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

    println!("binance futures plans rsmq workers process {symbol:} {interval:}");
    if let Err(err) = PlansRepository::flush(ctx.clone(), symbol, interval).await {
      println!("binance futures plans {symbol:} {interval:} flush failed {err:?}")
    }

    mutex.unlock().await.unwrap();

    Ok(())
  }

  pub async fn subscribe(&self, workers: &mut JoinSet<()>) -> Result<(), Box<dyn std::error::Error>> 
  {
    println!("binance futures plans rsmq workers subscribe");
    workers.spawn(Box::pin({
      let ctx = self.ctx.clone();
      async move {
        let rmq = ctx.rmq.lock().await.clone();
        let mut client = Rsmq::new(rmq).await.unwrap();
        loop {
          println!("binance futures plans rsmq loop");
          match client.pop_message::<String>(Config::RSMQ_QUEUE_PLANS).await {
            Ok(Some(message)) => {
              let (action, content) = serde_json::from_slice::<(String, String)>(message.message.as_bytes()).unwrap();
              match action.as_str() {
                Config::RSMQ_JOBS_PLANS_FLUSH => {
                  let payload = serde_json::from_slice::<PlansFlushPayload<&str>>(content.as_bytes()).unwrap();
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