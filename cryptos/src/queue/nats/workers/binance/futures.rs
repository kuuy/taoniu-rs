use crate::common::Ctx;

pub mod strategies;

pub struct FuturesWorker {}

impl<'a> FuturesWorker {
  pub async fn subscribe(&self, ctx: &'a mut Ctx<'_>) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
  }
}