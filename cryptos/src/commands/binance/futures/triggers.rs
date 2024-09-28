use clap::{Parser, Subcommand};

use crate::common::*;
use crate::repositories::binance::futures::triggers::*;

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
  /// triggers flush
  Scan,
}

impl TriggersCommand {
  pub fn new() -> Self {
    Self {
      ..Default::default()
    }
  }

  async fn scan(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    println!("triggers scan");
    let symbols = TriggersRepository::scan(ctx.clone()).await?;
    println!("triggers scan symbols {:?}", symbols);
    Ok(())
  }

  pub async fn run(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    match &self.commands {
      Commands::Scan => self.scan(ctx.clone()).await,
    }
  }
}
