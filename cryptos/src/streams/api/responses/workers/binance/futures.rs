use std::collections::HashMap;

use crate::common::*;

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
    _: &mut HashMap<&str, ResponseFn>,
  ) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance futures streams api responses workers subscribe");
    let _ = self.ctx.clone();
    Ok(())
  }
}