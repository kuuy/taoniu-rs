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
    let ctx = self.ctx.clone();
    IndicatorsWorker::new(ctx.clone()).subscribe().await?;
    StrategiesWorker::new(ctx.clone()).subscribe().await?;
    Ok(())
  }
}