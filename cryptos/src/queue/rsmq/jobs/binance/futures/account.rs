use futures_util::StreamExt;

use crate::common::*;
use crate::config::binance::spot::config as Config;

pub struct AccountJob {
  ctx: Ctx,
}

impl AccountJob {
  pub fn new(ctx: Ctx) -> Self {
    Self {
      ctx: ctx,
    }
  }

  pub async fn flush(&self) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance futures account rsmq job flush");
    Ok(())
  }
}