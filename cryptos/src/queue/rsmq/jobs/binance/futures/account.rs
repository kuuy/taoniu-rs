use futures_util::StreamExt;
use rsmq_async::{RsmqError, RsmqConnection};

use crate::common::*;
use crate::config::binance::futures::config as Config;

pub struct AccountJob {
  ctx: Ctx,
}

impl AccountJob {
  pub fn new(ctx: Ctx) -> Self {
    Self {
      ctx: ctx,
    }
  }

  pub async fn flush(&self) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance futures account rsmq job flush");
    let content = "";
    let message = serde_json::to_string(&[
      Config::RSMQ_JOBS_ACCOUNT_FLUSH,
      &content,
    ]).unwrap();
    let rmq = self.ctx.rmq.lock().await.clone();
    let mut client = Rsmq::new(rmq.clone()).await?;
    match client.send_message(Config::RSMQ_QUEUE_ACCOUNT, message.clone(), None).await {
      Err(RsmqError::QueueNotFound) => {
        client.create_queue(Config::RSMQ_QUEUE_ACCOUNT, None, None, None).await?;
        client.send_message(Config::RSMQ_QUEUE_ACCOUNT, message.clone(), None).await?;
      },
      _ => {},
    };
    Ok(())
  }
}