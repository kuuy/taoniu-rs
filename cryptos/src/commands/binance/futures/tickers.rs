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
  /// tickers price
  Price(PriceArgs),
  /// tickers flush
  Flush,
}

#[derive(Args)]
struct PriceArgs {
  /// symbol
  symbol: String,
}

impl TickersCommand {
  pub fn new() -> Self {
    Self {
      ..Default::default()
    }
  }
  
  async fn price(&self, ctx: Ctx, symbol: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("tickers price {symbol:}");
    let price = match TickersRepository::price(
      ctx,
      &symbol,
    ).await {
      Ok(price) => price,
      Err(e) => return Err(e.into()),
    };
    println!("price {}", price);
    Ok(())
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
      Commands::Price(args) => self.price(ctx.clone(), args.symbol.clone()).await,
      Commands::Flush => self.flush(ctx.clone()).await,
    }
  }
}
