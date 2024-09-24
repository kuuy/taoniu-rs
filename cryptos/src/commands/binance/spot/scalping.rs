use clap::{Parser, Subcommand};

use crate::common::*;
use crate::repositories::binance::spot::scalping::*;

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
  /// scalping scan
  Scan,
}

impl ScalpingCommand {
  pub fn new() -> Self {
    Self {
      ..Default::default()
    }
  }

  async fn scan(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    println!("scalping scan");
    let symbols = ScalpingRepository::scan(ctx).await?;
    println!("scalping scan symbols {:?}", symbols);
    Ok(())
  }

  pub async fn run(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    match &self.commands {
      Commands::Scan => self.scan(ctx).await,
    }
  }
}
