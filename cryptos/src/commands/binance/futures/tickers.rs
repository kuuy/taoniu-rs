use clap::{Parser, Args, Subcommand};

use crate::common::*;
use crate::repositories::binance::futures::tickers::*;

#[derive(Parser)]
pub struct TickersCommand {
  #[command(subcommand)]
  commands: Commands,
}

impl Default for TickersCommand {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(Subcommand)]
enum Commands {
  /// tickers flush
  Flush,
}

impl TickersCommand {
  pub fn new() -> Self {
    Self {
      ..Default::default()
    }
  }

  async fn flush(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    println!("tickers flush");
    let values = TickersRepository::flush(
      ctx,
      ["BTCUSDT", "ETHUSDT", "BNBUSDT", "ADAUSDT", "AUD"].to_vec(),
    ).await;
    println!("tickers flush {:?}", values);
    Ok(())
  }

  pub async fn run(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    match &self.commands {
      Commands::Flush => self.flush(ctx.clone()).await,
    }
  }
}
