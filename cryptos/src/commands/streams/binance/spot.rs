use clap::{Parser, Subcommand};

use crate::common::*;
use crate::commands::streams::binance::spot::tickers::*;
use crate::commands::streams::binance::spot::klines::*;

pub mod account;
pub mod tickers;
pub mod klines;

#[derive(Parser)]
pub struct SpotCommand {
  #[command(subcommand)]
  commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
  Tickers(TickersCommand),
  Klines(KlinesCommand),
}

impl SpotCommand {
  pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
    let rdb = Rdb::new(1).await.unwrap();
    let pool = Pool::new(1).unwrap();
    let ctx = Ctx::new(rdb, pool);
    match &self.commands {
      Commands::Tickers(tickers) => tickers.run(ctx).await,
      Commands::Klines(klines) => klines.run(ctx).await,
    }
  }
}