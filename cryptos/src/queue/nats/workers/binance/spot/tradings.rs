use std::collections::HashMap;

use crate::common::*;
use crate::queue::nats::workers::binance::spot::tradings::scalping::*;

pub mod scalping;

pub struct TradingsWorker {
  ctx: Ctx,
}

impl TradingsWorker {
  pub fn new(ctx: Ctx) -> Self {
    Self {
      ctx: ctx,
    }
  }

  pub async fn subscribe(&self, callbacks: &mut HashMap<&str, Vec<EventFn>>) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance spot nats tradings workers subscribe");
    ScalpingWorker::new(self.ctx.clone()).subscribe(callbacks).await?;
    Ok(())
  }
}