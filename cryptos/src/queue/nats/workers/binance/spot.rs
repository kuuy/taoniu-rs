use crate::common::Ctx;

pub mod strategies;

pub struct SpotWorkers {}

impl SpotWorkers {
  pub async fn subscribe(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
  }
}