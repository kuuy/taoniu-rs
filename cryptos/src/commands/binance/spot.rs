use redis::aio::MultiplexedConnection;
use clap::{Parser, Subcommand};

use crate::common::Rdb;

pub mod symbols;
pub mod positions;

pub use symbols::*;
pub use positions::*;

#[derive(Parser)]
pub struct SpotCommand {
  #[command(subcommand)]
  commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
  Symbols(SymbolsCommand),
  Positions(PositionsCommand),
}

impl<'a> SpotCommand {
  pub async fn run(&self, rdb: &'a mut MultiplexedConnection) -> Result<(), Box<dyn std::error::Error>> {
    match &self.commands {
      Commands::Symbols(symbols) => symbols.run(rdb).await,
      Commands::Positions(positions) => positions.run(),
    }
  }
}