use futures_util::StreamExt;

use crate::common::*;
use crate::config::binance::spot::config as Config;

pub struct IndicatorsWorker {
  ctx: Ctx,
}

impl IndicatorsWorker {
  pub fn new(ctx: Ctx) -> Self {
    Self {
      ctx: ctx,
    }
  }

  pub async fn subscribe(&self) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance spot indicators nats workers subscribe");
    let client = self.ctx.nats.clone();
    tokio::spawn(Box::pin({
      let mut subscriber = client.subscribe(Config::NATS_EVENTS_KLINES_UPDATE).await?;
      async move {
        while let Some(message) = subscriber.next().await {
          println!("message received: {:?}", message);
        }
      }
    }));
    Ok(())
  }
}