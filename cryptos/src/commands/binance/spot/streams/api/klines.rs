use clap::{Parser, Args, Subcommand};

use crate::common::*;
use crate::streams::api::requests::jobs::binance::spot::klines::*;

#[derive(Parser)]
pub struct KlinesCommand {
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
  /// klines flush
  Flush(FlushArgs),
}

#[derive(Args)]
struct FlushArgs {
  /// symbol
  symbol: String,
  /// symbol
  interval: String,
  /// endtime
  endtime: i64,
  /// limit
  limit: i64,
}

impl KlinesCommand {
  pub fn new() -> Self {
    Self {
      ..Default::default()
    }
  }

  async fn flush(&self, ctx: Ctx, symbol: String, interval: String, endtime: i64, limit: i64)-> Result<(), Box<dyn std::error::Error>> {
    println!("klines flush");
    let job = KlinesJob::new(ctx.clone());
    let _ = job.flush(symbol, interval, endtime, limit).await;
    Ok(())
  }

  pub async fn run(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    match &self.commands {
      Commands::Flush(args) => self.flush(ctx.clone(), args.symbol.clone(), args.interval.clone(), args.endtime, args.limit).await,
    }
  }
}
