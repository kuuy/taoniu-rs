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

impl Default for SpotCommand {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(Subcommand)]
enum Commands {
  Symbols(SymbolsCommand),
  Positions(PositionsCommand),
}

impl SpotCommand {
  pub fn new() -> Self {
    println!("current node 1");
    Self {
      ..Default::default()
    }
  }

  pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
    let mut rdb = Rdb::new(1).await.expect("redis connect failed");
    match &self.commands {
      Commands::Symbols(symbols) => symbols.run(&mut rdb).await,
      Commands::Positions(positions) => positions.run(),
    }
  }
}