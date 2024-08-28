use clap::{Parser, Subcommand};

use crate::common::*;
use crate::repositories::binance::spot::scalping::*;

#[derive(Parser)]
pub struct ScalpingCommand {
  #[clap(skip)]
  repository: ScalpingRepository,
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
  /// scalping flush
  Scan,
}

impl ScalpingCommand {
  pub fn new() -> Self {
    Self {
      repository: ScalpingRepository{},
      ..Default::default()
    }
  }

  async fn scan(&self, ctx: AppContext) -> Result<(), Box<dyn std::error::Error>> {
    println!("scalping scan");
    // let symbols = self.repository.scan(ctx).expect("scalping scan failed");
    // println!("scalping scan success {:?}", symbols);
    Ok(())
  }

  pub async fn run(&self, ctx: AppContext) -> Result<(), Box<dyn std::error::Error>> {
    match &self.commands {
      Commands::Scan => self.scan(ctx).await,
    }
  }
}
