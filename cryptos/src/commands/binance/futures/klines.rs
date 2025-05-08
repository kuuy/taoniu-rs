use clap::{Parser, Args, Subcommand};

use crate::common::*;
use crate::commands::binance::futures::klines::rsmq::*;
use crate::repositories::binance::futures::klines::*;

pub mod rsmq;

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
  /// klines gets
  Gets,
  /// klines timestamp
  Timestamp(TimestampArgs),
  Rsmq(RsmqCommand),
}

#[derive(Args)]
struct TimestampArgs {
  /// interval 1m 15m 4h 1d
  interval: String,
}

impl KlinesCommand {
  pub fn new() -> Self {
    Self {
      ..Default::default()
    }
  }

  async fn gets(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    let interval = "4h";
    let timestamp = KlinesRepository::timestamp(interval);
    println!("klines gets {} {}", interval, timestamp);
    let values = KlinesRepository::gets(
      ctx.clone(),
      ["ACEUSDT", "ACHUSDT", "ADAUSDT"].to_vec(),
      ["open", "close", "high", "low", "volume", "quota", "timestamp"].to_vec(),
      interval,
      timestamp,
    ).await;
    println!("klines gets {:?}", values);
    Ok(())
  }

  async fn timestamp(&self, interval: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("klines timestamp");
    if !["1m", "15m", "4h", "1d"].iter().any(|&s| s == interval) {
      return Err(Box::from("interval not valid"))
    }
    let timestamp = KlinesRepository::timestamp(interval);
    println!("klines timestamp {}", timestamp);
    Ok(())
  }

  pub async fn run(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    match &self.commands {
      Commands::Gets => self.gets(ctx.clone()).await,
      Commands::Timestamp(args) => self.timestamp(args.interval.clone()).await,
      Commands::Rsmq(nats) => nats.run(ctx.clone()).await,
    }
  }
}
