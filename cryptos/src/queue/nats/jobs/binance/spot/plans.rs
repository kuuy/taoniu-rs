use crate::common::*;
use crate::config::binance::spot::config as Config;
use crate::queue::nats::payload::binance::spot::plans::*;

pub struct PlansJob {
  ctx: Ctx,
}

impl PlansJob {
  pub fn new(ctx: Ctx) -> Self {
    Self {
      ctx: ctx,
    }
  }

  pub async fn update<T>(&self, id: T, side: i32, amount: f64) -> Result<(), Box<dyn std::error::Error>> 
  where
    T: AsRef<str>
  {
    let id = id.as_ref();
    let payload = PlansUpdatePayload::new(id, side, amount);
    let message = serde_json::to_string(&payload).unwrap();
    let client = self.ctx.nats.clone();
    client.publish(Config::NATS_EVENTS_PLANS_UPDATE, message.into()).await?;
    client.flush().await?;
    Ok(())
  }
}