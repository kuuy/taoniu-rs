use redis::aio::MultiplexedConnection;
use clap::{Parser, Subcommand};

use crate::common::*;
use crate::commands::binance::spot::symbols::*;
use crate::commands::binance::spot::positions::*;
use crate::commands::binance::spot::streams::*;
use crate::commands::binance::spot::scalping::*;

pub mod symbols;
pub mod positions;
pub mod streams;
pub mod scalping;

#[derive(Parser)]
pub struct SpotCommand {
  #[command(subcommand)]
  commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
  Symbols(SymbolsCommand),
  Positions(PositionsCommand),
  Scalping(ScalpingCommand),
  Streams(StreamsCommand),
}

impl SpotCommand {
  pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
    let mut rdb = Rdb::new(1).await.expect("redis connect failed");
    let mut db = Db::new(1).expect("db connect failed");
    let mut ctx = Ctx{
      rdb: &mut rdb,
      db: &mut db,
    };
    match &self.commands {
      Commands::Symbols(symbols) => symbols.run(&mut ctx).await,
      Commands::Positions(positions) => positions.run(),
      Commands::Scalping(scalping) => scalping.run(&mut ctx).await,
      Commands::Streams(streams) => streams.run(&mut ctx).await,
    }
  }
}