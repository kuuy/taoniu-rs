use crate::common::*;
use crate::queue::nats::workers::binance::futures::indicators::*;
use crate::queue::nats::workers::binance::futures::strategies::*;

pub mod indicators;
pub mod strategies;

pub struct FuturesWorkers {
  ctx: Ctx,
}

impl FuturesWorkers {
  pub fn new(ctx: Ctx) -> Self {
    Self {
      ctx: ctx,
    }
  }

  pub async fn subscribe(&self) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance futures nats workers subscribe");
    let ctx = self.ctx.clone();
    IndicatorsWorker::new(ctx.clone()).subscribe().await?;
    StrategiesWorker::new(ctx.clone()).subscribe().await?;
    Ok(())
  }
}