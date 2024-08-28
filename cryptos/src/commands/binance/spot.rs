use clap::{Parser, Subcommand};

use crate::common::*;
use crate::commands::binance::spot::symbols::*;
use crate::commands::binance::spot::klines::*;
use crate::commands::binance::spot::positions::*;
use crate::commands::binance::spot::scalping::*;

pub mod symbols;
pub mod klines;
pub mod positions;
pub mod scalping;

#[derive(Parser)]
pub struct SpotCommand {
  #[command(subcommand)]
  commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
  Symbols(SymbolsCommand),
  Klines(KlinesCommand),
  Positions(PositionsCommand),
  Scalping(ScalpingCommand),
}

impl SpotCommand {
  pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
    let rdb = Rdb::new(1).await.expect("redis connect failed");
    let pool = Pool::new(1).expect("pool connect failed");
    let ctx = AppContext::new(rdb, pool);
    match &self.commands {
      Commands::Symbols(symbols) => symbols.run(ctx).await,
      Commands::Klines(klines) => klines.run(ctx).await,
      Commands::Positions(positions) => positions.run(),
      Commands::Scalping(scalping) => scalping.run(ctx).await,
    }
  }
}