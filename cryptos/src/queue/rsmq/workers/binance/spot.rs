use crate::common::*;
use crate::queue::rsmq::workers::binance::spot::account::*;

pub mod account;

pub struct SpotWorkers {
  ctx: Ctx,
}

impl SpotWorkers {
  pub fn new(ctx: Ctx) -> Self {
    Self {
      ctx: ctx,
    }
  }

  pub async fn subscribe(&self) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance spot nats workers subscribe");
    let ctx = self.ctx.clone();
    AccountWorker::new(ctx.clone()).subscribe().await?;
    Ok(())
  }
}