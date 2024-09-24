use crate::common::*;
use crate::config::binance::spot::config as Config;
use crate::queue::nats::payload::binance::spot::klines::*;

pub struct KlinesJob {
  ctx: Ctx,
}

impl KlinesJob {
  pub fn new(ctx: Ctx) -> Self {
    Self {
      ctx: ctx,
    }
  }

  pub async fn update<T>(&self, symbol: T, interval: T) -> Result<(), Box<dyn std::error::Error>> 
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();
    let interval = interval.as_ref();
    let payload = KlinesUpdatePayload::new(symbol, interval);
    let message = serde_json::to_string(&payload).unwrap();
    let client = self.ctx.nats.clone();
    client.publish(Config::NATS_EVENTS_KLINES_UPDATE, message.into()).await?;
    client.flush().await?;
    Ok(())
  }
}