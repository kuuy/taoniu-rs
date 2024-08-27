use crate::common::Ctx;

pub struct StrategiesWorker {}

impl<'a> StrategiesWorker {
  pub async fn subscribe(&self, ctx: &'a mut Ctx<'_>) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
  }
}