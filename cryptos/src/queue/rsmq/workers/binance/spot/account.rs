use futures_util::StreamExt;
use rsmq_async::{RsmqMessage, RsmqConnection};

use crate::common::*;
use crate::config::binance::spot::config as Config;

pub struct AccountWorker {
  ctx: Ctx,
}

impl AccountWorker {
  pub fn new(ctx: Ctx) -> Self {
    Self {
      ctx: ctx,
    }
  }

  pub async fn subscribe(&self) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance spot account rsmq workers subscribe");
    tokio::spawn(Box::pin({
      let rmq = self.ctx.rmq.lock().await.clone();
      async move {
        let mut client = Rsmq::new(rmq.clone()).await.unwrap();
        loop {
          let _ = match client.receive_message::<String>(Config::RSMQ_QUEUE_ACCOUNT, None).await {
            Ok(Some(message)) => {
              println!("message received: {:?}", message);
              let _ = client.delete_message(Config::RSMQ_QUEUE_ACCOUNT, &message.id).await;
            },
            Ok(None) => {
              tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            }
            Err(_) => {}
          };
        }
      }
    }));
    Ok(())
  }
}