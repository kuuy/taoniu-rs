use clap::{Parser, Subcommand};

use crate::common::*;
use crate::commands::binance::futures::account::*;
use crate::commands::binance::futures::symbols::*;
use crate::commands::binance::futures::klines::*;
use crate::commands::binance::futures::positions::*;
use crate::commands::binance::futures::scalping::*;

pub mod account;
pub mod symbols;
pub mod klines;
pub mod positions;
pub mod scalping;

#[derive(Parser)]
pub struct FuturesCommand {
  #[command(subcommand)]
  commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
  Account(AccountCommand),
  Symbols(SymbolsCommand),
  Klines(KlinesCommand),
  Positions(PositionsCommand),
  Scalping(ScalpingCommand),
}

impl FuturesCommand {
  pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
    let rdb = Rdb::new(2).await.unwrap();
    let rmq = Rmq::new(2).await.unwrap();
    let pool = Pool::new(2).unwrap();
    let nats = Nats::new().await.unwrap();
    let ctx = Ctx::new(rdb, rmq, pool, nats);
    match &self.commands {
      Commands::Account(account) => account.run(ctx).await,
      Commands::Symbols(symbols) => symbols.run(ctx).await,
      Commands::Klines(klines) => klines.run(ctx).await,
      Commands::Positions(positions) => positions.run(),
      Commands::Scalping(scalping) => scalping.run(ctx).await,
    }
  }
}