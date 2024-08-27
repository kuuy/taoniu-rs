use crate::common::Ctx;

pub mod strategies;

pub struct SpotWorker {}

impl<'a> SpotWorker {
  pub async fn subscribe(&self, ctx: &'a mut Ctx<'_>) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
  }
}