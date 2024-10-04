use tokio::task::JoinSet;

use crate::common::*;
use crate::queue::rsmq::workers::binance::spot::account::*;
use crate::queue::rsmq::workers::binance::spot::klines::*;

pub mod klines;
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

  pub async fn subscribe(&self, workers: &mut JoinSet<()>) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance spot rsmq workers subscribe");
    let ctx = self.ctx.clone();
    AccountWorker::new(ctx.clone()).subscribe(workers).await?;
    KlinesWorker::new(ctx.clone()).subscribe(workers).await?;
    Ok(())
  }
}