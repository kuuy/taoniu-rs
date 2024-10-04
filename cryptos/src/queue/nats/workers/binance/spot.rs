use tokio::task::JoinSet;

use crate::common::*;
use crate::queue::nats::workers::binance::spot::indicators::*;
use crate::queue::nats::workers::binance::spot::strategies::*;
use crate::queue::nats::workers::binance::spot::plans::*;
// use crate::queue::nats::workers::binance::spot::tradings::*;

pub mod indicators;
pub mod strategies;
pub mod plans;
pub mod tradings;

pub struct SpotWorker {
  ctx: Ctx,
}

impl SpotWorker {
  pub fn new(ctx: Ctx) -> Self {
    Self {
      ctx: ctx,
    }
  }

  pub async fn subscribe(&self, workers: &mut JoinSet<()>) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance spot nats workers subscribe");
    IndicatorsWorker::new(self.ctx.clone()).subscribe(workers).await?;
    StrategiesWorker::new(self.ctx.clone()).subscribe(workers).await?;
    PlansWorker::new(self.ctx.clone()).subscribe(workers).await?;
    // TradingsWorker::new(self.ctx.clone()).subscribe(workers).await?;
    Ok(())
  }
}