use clap::{Parser, Subcommand};

use crate::common::*;
use crate::commands::binance::spot::streams::tickers::*;
use crate::commands::binance::spot::streams::klines::*;

pub mod account;
pub mod tickers;
pub mod klines;

#[derive(Parser)]
pub struct StreamsCommand {
  #[command(subcommand)]
  commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
  Tickers(TickersCommand),
  Klines(KlinesCommand),
}

impl<'a> StreamsCommand {
  pub async fn run(&self, ctx: &'a mut Ctx<'_>) -> Result<(), Box<dyn std::error::Error>> {
    match &self.commands {
      Commands::Tickers(tickers) => tickers.run(ctx).await,
      Commands::Klines(klines) => klines.run(ctx).await,
    }
  }
}