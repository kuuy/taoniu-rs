use futures_util::StreamExt;

use crate::common::*;
use crate::config::binance::futures::config as Config;

pub struct StrategiesJob {
  ctx: Ctx,
}

impl StrategiesJob {
  pub fn new(ctx: Ctx) -> Self {
    Self {
      ctx: ctx,
    }
  }

  pub async fn publish(&self) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance futures strategies nats job publish");
    let nats = self.ctx.nats.clone();
    Ok(())
  }
}