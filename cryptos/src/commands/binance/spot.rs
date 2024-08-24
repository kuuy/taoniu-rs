use redis::aio::MultiplexedConnection;
use clap::{Parser, Subcommand};

use crate::common::*;

pub mod symbols;
pub mod positions;
pub mod streams;

pub use symbols::*;
pub use positions::*;
pub use streams::*;

#[derive(Parser)]
pub struct SpotCommand {
  #[command(subcommand)]
  commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
  Symbols(SymbolsCommand),
  Positions(PositionsCommand),
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
      Commands::Streams(streams) => streams.run().await,
    }
  }
}