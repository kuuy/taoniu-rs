use crate::common::Ctx;

pub struct StrategiesWorker {}

impl StrategiesWorker {
  pub async fn subscribe(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
  }
}