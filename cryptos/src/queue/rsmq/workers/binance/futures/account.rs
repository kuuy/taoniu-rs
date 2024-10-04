use rsmq_async::RsmqConnection;
use tokio::task::JoinSet;

use crate::common::*;
use crate::config::binance::futures::config as Config;

pub struct AccountWorker {
  ctx: Ctx,
}

impl AccountWorker {
  pub fn new(ctx: Ctx) -> Self {
    Self {
      ctx: ctx,
    }
  }

  pub async fn subscribe(&self, workers: &mut JoinSet<()>) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance futures account rsmq workers subscribe");
    workers.spawn(Box::pin({
      let rmq = self.ctx.rmq.lock().await.clone();
      async move {
        let mut client = Rsmq::new(rmq.clone()).await.unwrap();
        loop {
          match client.pop_message::<String>(Config::RSMQ_QUEUE_ACCOUNT).await {
            Ok(Some(message)) => {
              println!("message received: {:?}", message);
              let _ = client.delete_message(Config::RSMQ_QUEUE_ACCOUNT, &message.id).await;
            },
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