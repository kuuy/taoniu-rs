use std::collections::HashMap;

use crate::common::*;
use crate::streams::api::requests::workers::binance::spot::klines::*;

pub mod klines;

pub struct SpotWorker {
  ctx: Ctx,
}

impl SpotWorker {
  pub fn new(ctx: Ctx) -> Self {
    Self {
      ctx: ctx,
    }
  }

  pub async fn subscribe(
    &self,
    callbacks: &mut HashMap<&str, RequestFn>,
  ) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance spot streams api requests workers subscribe");
    KlinesWorker::new(self.ctx.clone()).subscribe(callbacks).await?;
    Ok(())
  }
}