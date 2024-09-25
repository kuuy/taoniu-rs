use crate::common::*;
use crate::queue::nats::workers::binance::spot::indicators::*;
use crate::queue::nats::workers::binance::spot::strategies::*;

pub mod indicators;
pub mod strategies;

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
    IndicatorsWorker::new(self.ctx.clone()).subscribe().await?;
    StrategiesWorker::new(self.ctx.clone()).subscribe().await?;
    Ok(())
  }
}