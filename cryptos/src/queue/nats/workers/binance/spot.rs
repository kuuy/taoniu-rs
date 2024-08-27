use crate::common::Ctx;

pub mod strategies;

pub struct SpotWorkers {}

impl<'a> SpotWorkers {
  pub async fn subscribe(&self, ctx: &'a mut Ctx<'_>) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
  }
}