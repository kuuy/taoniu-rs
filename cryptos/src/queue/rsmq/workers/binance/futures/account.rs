use futures_util::StreamExt;

use crate::common::*;
use crate::config::binance::futures::config as Config;

pub struct AccountWorker {
  ctx: Ctx,
}

impl AccountWorker {
  pub fn new(ctx: Ctx) -> Self {
    Self {
      ctx: ctx,
    }
  }

  pub async fn subscribe(&self) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance futures account rsmq workers subscribe");
    Ok(())
  }
}