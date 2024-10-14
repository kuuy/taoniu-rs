use clap::{Parser, Subcommand};

use crate::common::*;
use crate::queue::rsmq::jobs::binance::spot::klines::*;

#[derive(Parser)]
pub struct RsmqCommand {
  #[command(subcommand)]
  commands: Commands,
}

impl Default for RsmqCommand {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(Subcommand)]
enum Commands {
  /// rsmq publish
  Publish,
  /// rsmq subscribe
  Subscribe,
}

impl RsmqCommand {
  pub fn new() -> Self {
    Self {
      ..Default::default()
    }
  }

  async fn publish(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance spot klines rsmq publish");
    let job = KlinesJob::new(ctx.clone());
    job.sync("15m").await?;
    Ok(())
  }

  async fn subscribe(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance spot klines rsmq subscribe");
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
