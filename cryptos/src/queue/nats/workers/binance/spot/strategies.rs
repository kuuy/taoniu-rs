use futures_util::StreamExt;

use crate::common::*;
use crate::config::binance::spot::config as Config;

pub struct StrategiesWorker {
  ctx: Ctx,
}

impl StrategiesWorker {
  pub fn new(ctx: Ctx) -> Self {
    Self {
      ctx: ctx,
    }
  }

  pub async fn subscribe(&self) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance spot strategies nats workers subscribe");
    tokio::spawn(Box::pin({
      let ctx = self.ctx.clone();
      async move {
        let client = ctx.nats.clone();
        let mut subscriber = client.subscribe(Config::NATS_EVENTS_INDICATORS_UPDATE).await.unwrap();
        while let Some(message) = subscriber.next().await {
          println!("message received: {:?}", message);
        }
      }
    }));
    Ok(())
  }
}