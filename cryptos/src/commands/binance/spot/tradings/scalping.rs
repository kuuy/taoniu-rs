use clap::{Parser, Subcommand};

use crate::common::*;

#[derive(Parser)]
pub struct ScalpingCommand {
  #[command(subcommand)]
  commands: Commands,
}

impl Default for ScalpingCommand {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(Subcommand)]
enum Commands {
  /// scalping place
  Place,
  /// scalping flush
  Flush,
}

impl ScalpingCommand {
  pub fn new() -> Self {
    Self {
      ..Default::default()
    }
  }

  async fn place(&self, _: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance spot tradings scalping place");
    Ok(())
  }

  async fn flush(&self, _: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance spot tradings scalping flush");
    Ok(())
  }

  pub async fn run(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    match &self.commands {
      Commands::Place => self.place(ctx.clone()).await,
      Commands::Flush => self.flush(ctx.clone()).await,
    }
  }
}
