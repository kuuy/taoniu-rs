use rsmq_async::{RsmqError, RsmqConnection};

use crate::common::*;
use crate::config::binance::spot::config as Config;
use crate::queue::rsmq::payload::binance::spot::indicators::*;

pub struct IndicatorsJob {
  ctx: Ctx,
}

impl IndicatorsJob {
  pub fn new(ctx: Ctx) -> Self {
    Self {
      ctx: ctx,
    }
  }

  pub async fn flush<T>(&self, symbol: T, interval: T) -> Result<(), Box<dyn std::error::Error>> 
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();
    let interval = interval.as_ref();

    let payload = IndicatorsFlushPayload::new(symbol, interval);
    let content = serde_json::to_string(&payload).unwrap();
    let message = serde_json::to_string(&[
      Config::RSMQ_JOBS_INDICATORS_FLUSH,
      &content,
    ]).unwrap();

    let rmq = self.ctx.rmq.lock().await.clone();
    let mut client = Rsmq::new(rmq.clone()).await?;
    match client.send_message(Config::RSMQ_QUEUE_INDICATORS, message.clone(), None).await {
      Err(RsmqError::QueueNotFound) => {
        client.create_queue(Config::RSMQ_QUEUE_INDICATORS, None, None, None).await?;
        client.send_message(Config::RSMQ_QUEUE_INDICATORS, message.clone(), None).await?;
      }
      _ => ()
    };

    Ok(())
  }
}