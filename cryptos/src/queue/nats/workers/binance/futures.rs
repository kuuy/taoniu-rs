use crate::common::Ctx;

pub mod strategies;

pub struct FuturesWorkers {}

impl<'a> FuturesWorkers {
  pub async fn subscribe(&self, ctx: &'a mut Ctx<'_>) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
  }
}