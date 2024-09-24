use crate::common::*;

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
    let _ = self.ctx.nats.clone();
    Ok(())
  }
}