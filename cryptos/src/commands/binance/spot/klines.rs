use clap::{Parser, Args, Subcommand};

use crate::common::*;
use crate::repositories::binance::spot::klines::*;

#[derive(Parser)]
pub struct KlinesCommand {
  #[clap(skip)]
  repository: KlinesRepository,
  #[command(subcommand)]
  commands: Commands,
}

impl Default for KlinesCommand {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(Subcommand)]
enum Commands {
  /// klines timestamp
  Timestamp(TimestampArgs),
}

#[derive(Args)]
struct TimestampArgs {
  /// interval 1m 15m 4h 1d
  interval: String,
}

impl<'a> KlinesCommand {
  pub fn new() -> Self {
    Self {
      repository: KlinesRepository{},
      ..Default::default()
    }
  }

  async fn timestamp(&self, interval: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("klines timestamp");
    if !["1m", "15m", "4h", "1d"].iter().any(|&s| s == interval) {
      return Err(Box::from("interval not valid"))
    }
    let timestamp = self.repository.timestamp(interval);
    println!("klines timestamp {}", timestamp);
    Ok(())
  }

  pub async fn run(&self, _: AppContext) -> Result<(), Box<dyn std::error::Error>> {
    match &self.commands {
      Commands::Timestamp(args) => self.timestamp(args.interval.clone()).await,
    }
  }
}
