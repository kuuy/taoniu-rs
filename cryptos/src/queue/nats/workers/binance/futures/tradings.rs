use tokio::task::JoinSet;

use crate::common::*;
use crate::queue::nats::workers::binance::futures::tradings::scalping::*;

pub mod scalping;

pub struct TradingsWorker {
  ctx: Ctx,
}

impl TradingsWorker {
  pub fn new(ctx: Ctx) -> Self {
    Self {
      ctx: ctx,
    }
  }

  pub async fn subscribe(&self, workers: &mut JoinSet<()>) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance futures nats tradings workers subscribe");
    ScalpingWorker::new(self.ctx.clone()).subscribe(workers).await?;
    Ok(())
  }
}