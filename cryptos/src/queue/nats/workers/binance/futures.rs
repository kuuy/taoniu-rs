use crate::common::Ctx;

pub mod indicators;
pub mod strategies;

pub struct FuturesWorkers {}

impl FuturesWorkers {
  pub async fn subscribe(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
  }
}