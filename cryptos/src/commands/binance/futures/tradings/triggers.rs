use clap::{Parser, Subcommand};

use crate::common::*;

#[derive(Parser)]
pub struct TriggersCommand {
  #[command(subcommand)]
  commands: Commands,
}

impl Default for TriggersCommand {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(Subcommand)]
enum Commands {
  /// triggers place
  Place,
  /// triggers flush
  Flush,
}

impl TriggersCommand {
  pub fn new() -> Self {
    Self {
      ..Default::default()
    }
  }

  async fn place(&self, _: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance futures tradings triggers place");
    Ok(())
  }

  async fn flush(&self, _: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance futures tradings triggers flush");
    Ok(())
  }

  pub async fn run(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    match &self.commands {
      Commands::Place => self.place(ctx.clone()).await,
      Commands::Flush => self.flush(ctx.clone()).await,
    }
  }
}
