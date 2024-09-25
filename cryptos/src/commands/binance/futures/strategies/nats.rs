use clap::{Parser, Subcommand};

use crate::common::*;

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
    println!("binance futures strategies nats publish");
    let _ = ctx.clone();
    Ok(())
  }

  async fn subscribe(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance futures strategies nats subscribe");
    let _ = ctx.clone();
    Ok(())
  }

  pub async fn run(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    match &self.commands {
      Commands::Publish => self.publish(ctx).await,
      Commands::Subscribe => self.subscribe(ctx).await,
    }
  }
}
