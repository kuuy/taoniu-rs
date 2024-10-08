use clap::{Parser, Subcommand};

use crate::common::*;
use crate::queue::nats::jobs::binance::futures::indicators::*;

#[derive(Parser)]
pub struct NatsCommand {
  #[command(subcommand)]
  commands: Commands,
}

impl Default for NatsCommand {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(Subcommand)]
enum Commands {
  /// nats publish
  Publish,
  /// nats subscribe
  Subscribe,
}

impl NatsCommand {
  pub fn new() -> Self {
    Self {
      ..Default::default()
    }
  }

  async fn publish(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance futures indicators nats publish");
    let job = IndicatorsJob::new(ctx.clone());
    job.update("BTCUSDT", "15m").await?;
    Ok(())
  }

  async fn subscribe(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance futures indicators nats subscribe");
    let _ = ctx.clone();
    Ok(())
  }

  pub async fn run(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    match &self.commands {
      Commands::Publish => self.publish(ctx.clone()).await,
      Commands::Subscribe => self.subscribe(ctx.clone()).await,
    }
  }
}
