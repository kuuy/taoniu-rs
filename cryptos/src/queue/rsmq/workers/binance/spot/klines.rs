use std::time::Duration;

use rsmq_async::RsmqConnection;
use tokio::task::JoinSet;

use crate::common::*;
use crate::config::binance::spot::config as Config;
use crate::queue::rsmq::payload::binance::spot::klines::*;
use crate::repositories::binance::spot::scalping::*;
use crate::repositories::binance::spot::klines::*;

pub struct KlinesWorker {
  ctx: Ctx,
}

impl KlinesWorker {
  pub fn new(ctx: Ctx) -> Self {
    Self {
      ctx: ctx,
    }
  }

  pub async fn sync<T>(ctx: Ctx, interval: T) -> Result<(), Box<dyn std::error::Error>> 
  where
    T: AsRef<str>
  {
    let interval = interval.as_ref();
  
    let rdb = ctx.rdb.lock().await.clone();
    let mutex_id = xid::new().to_string();
    let redis_key = format!("{}:{}", Config::LOCKS_KLINES_SYNC, interval);
    let mut mutex = Mutex::new(
      rdb,
      &redis_key,
      &mutex_id,
    );
    if !mutex.lock(Duration::from_secs(30)).await.unwrap() {
      return Err(Box::from(format!("mutex failed {}", redis_key)));
    }
 
    let symbols = ScalpingRepository::scan(ctx.clone()).await.unwrap();
    let timestamp = KlinesRepository::timestamp(interval);
    let _ = KlinesRepository::sync(
      ctx.clone(),
      symbols.iter().map(String::as_ref).collect(),
      interval,
      timestamp,
    ).await;
 
    mutex.unlock().await.unwrap();
    Ok(())
  }

  pub async fn subscribe(&self, workers: &mut JoinSet<()>) -> Result<(), Box<dyn std::error::Error>> 
  {
    println!("binance spot klines rsmq workers subscribe");
    workers.spawn(Box::pin({
      let ctx = self.ctx.clone();
      async move {
        let rmq = ctx.rmq.lock().await.clone();
        let mut client = Rsmq::new(rmq).await.unwrap();
        loop {
          println!("binance spot klines rsmq loop");
          match client.pop_message::<String>(Config::RSMQ_QUEUE_KLINES).await {
            Ok(Some(message)) => {
              let (action, content) = serde_json::from_slice::<(String, String)>(message.message.as_bytes()).unwrap();
              match action.as_str() {
                Config::RSMQ_JOBS_KLINES_SYNC => {
                  let payload = serde_json::from_slice::<KlinesSyncPayload<&str>>(content.as_bytes()).unwrap();
                  if let Err(err) = Self::sync(ctx.clone(), payload.interval).await {
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
          tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
      }
    }));
    Ok(())
  }
}