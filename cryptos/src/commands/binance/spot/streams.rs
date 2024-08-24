use redis::aio::MultiplexedConnection;
use clap::{Parser, Subcommand};

use crate::common::*;

pub mod account;
pub mod tickers;
pub mod klines;

pub use account::*;
pub use tickers::*;
pub use klines::*;

#[derive(Parser)]
pub struct StreamsCommand {
  #[command(subcommand)]
  commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
  Tickers(TickersCommand),
}

impl StreamsCommand {
  pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
    let mut rdb = Rdb::new(1).await.expect("redis connect failed");
    let mut db = Db::new(1).expect("db connect failed");
    let mut ctx = Ctx{
      rdb: &mut rdb,
      db: &mut db,
    };
    match &self.commands {
      Commands::Tickers(tickers) => tickers.run(&mut ctx).await,
    }
  }
}