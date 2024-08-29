use futures_util::StreamExt;

use crate::common::*;
use crate::config::binance::spot::config as Config;

pub struct IndicatorsJob {
  ctx: Ctx,
}

impl IndicatorsJob {
  pub fn new(ctx: Ctx) -> Self {
    Self {
      ctx: ctx,
    }
  }

  pub async fn update(&self) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance spot indicators nats job update");
    let client = self.ctx.nats.clone();
    client.publish(Config::NATS_EVENTS_INDICATORS_UPDATE, "binance spot incidateors update".into()).await?;
    client.flush().await?;
    Ok(())
  }
}