use std::collections::HashMap;

use crate::common::*;
use crate::queue::nats::workers::binance::futures::indicators::*;
use crate::queue::nats::workers::binance::futures::strategies::*;
use crate::queue::nats::workers::binance::futures::plans::*;
use crate::queue::nats::workers::binance::futures::tradings::*;

pub mod indicators;
pub mod strategies;
pub mod plans;
pub mod tradings;

pub struct FuturesWorker {
  ctx: Ctx,
}

impl FuturesWorker {
  pub fn new(ctx: Ctx) -> Self {
    Self {
      ctx: ctx,
    }
  }

  pub async fn subscribe(
    &self,
    callbacks: &mut HashMap<&str, Vec<EventFn>>,
  ) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance futures nats workers subscribe");
    IndicatorsWorker::new(self.ctx.clone()).subscribe(callbacks).await?;
    StrategiesWorker::new(self.ctx.clone()).subscribe(callbacks).await?;
    PlansWorker::new(self.ctx.clone()).subscribe(callbacks).await?;
    TradingsWorker::new(self.ctx.clone()).subscribe(callbacks).await?;
    Ok(())
  }
}