use crate::common::*;
use crate::config::binance::spot::config as Config;
use crate::streams::api::requests::payload::binance::spot::klines::*;

pub struct KlinesJob {
  ctx: Ctx,
}

impl KlinesJob {
  pub fn new(ctx: Ctx) -> Self {
    Self {
      ctx: ctx,
    }
  }

  pub async fn flush<T>(&self, symbol: T, interval: T, endtime: i64, limit: i64) -> Result<(), Box<dyn std::error::Error>> 
  where
    T: AsRef<str>
  {
    let symbol = symbol.as_ref();
    let interval = interval.as_ref();
    let payload = KlinesFlushPayload::new(symbol, interval, endtime, limit);
    let message = serde_json::to_string(&payload).unwrap();
    let client = self.ctx.nats.clone();
    client.publish(Config::NATS_EVENTS_API_KLINES_FLUSH, message.into()).await?;
    client.flush().await?;
    Ok(())
  }
}