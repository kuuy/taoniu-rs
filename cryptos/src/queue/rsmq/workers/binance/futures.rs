use tokio::task::JoinSet;

use crate::common::*;
use crate::queue::rsmq::workers::binance::futures::account::*;
use crate::queue::rsmq::workers::binance::futures::klines::*;

pub mod klines;
pub mod account;

pub struct FuturesWorkers {
  ctx: Ctx,
}

impl FuturesWorkers {
  pub fn new(ctx: Ctx) -> Self {
    Self {
      ctx: ctx,
    }
  }

  pub async fn subscribe(&self, workers: &mut JoinSet<()>) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance futures rsmq workers subscribe");
    let ctx = self.ctx.clone();
    AccountWorker::new(ctx.clone()).subscribe(workers).await?;
    KlinesWorker::new(ctx.clone()).subscribe(workers).await?;
    Ok(())
  }
}