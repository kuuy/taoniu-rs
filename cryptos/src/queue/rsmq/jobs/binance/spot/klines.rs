use rsmq_async::{RsmqError, RsmqConnection};

use crate::common::*;
use crate::config::binance::spot::config as Config;
use crate::queue::rsmq::payload::binance::spot::klines::*;

pub struct KlinesJob {
  ctx: Ctx,
}

impl KlinesJob {
  pub fn new(ctx: Ctx) -> Self {
    Self {
      ctx: ctx,
    }
  }

  pub async fn sync<T>(&self, interval: T) -> Result<(), Box<dyn std::error::Error>> 
  where
    T: AsRef<str>
  {
    let interval = interval.as_ref();

    let payload = KlinesSyncPayload::new(interval);
    let content = serde_json::to_string(&payload).unwrap();
    let message = serde_json::to_string(&[
      Config::RSMQ_JOBS_KLINES_SYNC,
      &content,
    ]).unwrap();

    let rmq = self.ctx.rmq.lock().await.clone();
    let mut client = Rsmq::new(rmq.clone()).await?;
    match client.send_message(Config::RSMQ_QUEUE_KLINES, message.clone(), None).await {
      Err(RsmqError::QueueNotFound) => {
        client.create_queue(Config::RSMQ_QUEUE_KLINES, None, None, None).await?;
        client.send_message(Config::RSMQ_QUEUE_KLINES, message.clone(), None).await?;
      }
      _ => (),
    };

    Ok(())
  }
}