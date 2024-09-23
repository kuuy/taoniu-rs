use std::time::Duration;

use futures_util::StreamExt;
use rsmq_async::{RsmqMessage, RsmqConnection};

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

  pub async fn flush<T>(ctx: Ctx, interval: T) -> Result<(), Box<dyn std::error::Error>> 
  where
    T: AsRef<str>
  {
    let interval = interval.as_ref();
  
    let rdb = ctx.rdb.lock().await.clone();
    let mutex_id = xid::new().to_string();
    let redis_key = format!("{}:{}", Config::LOCKS_KLINES_FLUSH, interval);
    let mut mutex = Mutex::new(
      rdb,
      &redis_key[..],
      &mutex_id[..],
    );
    if !mutex.lock(Duration::from_secs(30)).await.unwrap() {
      return Err(Box::from(format!("mutex failed {}", redis_key)));
    }

    let symbols = ScalpingRepository::scan(ctx.clone()).await.unwrap();
    let timestamp = KlinesRepository::timestamp(interval);
    let _ = KlinesRepository::flush(ctx.clone(), symbols.iter().map(String::as_ref).collect(), interval, timestamp).await;

    mutex.unlock().await.unwrap();
    Ok(())
  }

  pub async fn subscribe(&self) -> Result<(), Box<dyn std::error::Error>> 
  {
    println!("binance spot klines rsmq workers subscribe");
    tokio::spawn(Box::pin({
      let ctx = self.ctx.clone();
      async move {
        let rmq = ctx.rmq.lock().await.clone();
        let mut client = Rsmq::new(rmq).await.unwrap();
        loop {
          println!("binance spot klines rsmq loop");
          let _ = match client.pop_message::<String>(Config::RSMQ_QUEUE_KLINES).await {
            Ok(Some(message)) => {
              let (action, content) = serde_json::from_slice::<(String, String)>(message.message.as_bytes()).unwrap();
              match action.as_str() {
                Config::RSMQ_JOBS_KLINES_FLUSH => {
                  let payload = serde_json::from_slice::<KlinesFlushPayload<&str>>(content.as_bytes()).unwrap();
                  if let Err(e) = Self::flush(ctx.clone(), payload.interval).await {
                    println!("{e:?}");
                  }
                }
                _ => {},
              };
            },
            Ok(None) => {
              tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            }
            Err(_) => {}
          };
          tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
      }
    }));
    Ok(())
  }
}