use clap::{Parser, Subcommand};

use crate::common::*;
use crate::commands::binance::futures::symbols::*;
use crate::commands::binance::futures::positions::*;
use crate::commands::binance::futures::scalping::*;

pub mod symbols;
pub mod positions;
pub mod scalping;

#[derive(Parser)]
pub struct FuturesCommand {
  #[command(subcommand)]
  commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
  Symbols(SymbolsCommand),
  Positions(PositionsCommand),
  Scalping(ScalpingCommand),
}

impl FuturesCommand {
  pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
    let mut rdb = Rdb::new(2).await.expect("redis connect failed");
    let mut db = Db::new(2).expect("db connect failed");
    let mut nats = Nats::new().await.expect("nats connect failed");
    let mut rsmq = Rsmq::new(&mut rdb).await.expect("rsmq connect failed");
    let mut ctx = Ctx{
      rdb: &mut rdb,
      db: &mut db,
      nats: &mut nats,
      rsmq: &mut rsmq,
    };
    match &self.commands {
      Commands::Symbols(symbols) => symbols.run(&mut ctx).await,
      Commands::Positions(positions) => positions.run(),
      Commands::Scalping(scalping) => scalping.run(&mut ctx).await,
    }
  }
}