use clap::{Parser, Subcommand};

use crate::common::*;
use crate::queue::rsmq::jobs::binance::futures::account::*;

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
    println!("binance futures account rsmq publish");
    let job = AccountJob::new(ctx.clone());
    job.flush().await?;
    Ok(())
  }

  async fn subscribe(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance futures account rsmq subscribe");
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
