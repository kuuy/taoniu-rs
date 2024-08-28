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

impl<'a> SpotCommand {
  pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
    let mut rdb = Rdb::new(1).await.expect("redis connect failed");
    let mut db = Db::new(1).expect("db connect failed");
    let mut pool = Pool::new(1).expect("pool connect failed");
    let mut nats = Nats::new().await.expect("nats connect failed");
    let mut rsmq = Rsmq::new(&mut rdb).await.expect("rsmq connect failed");
    let mut ctx = Ctx{
      rdb: &mut rdb,
      db: &mut db,
      nats: &mut nats,
      rsmq: &mut rsmq,
    };
    match &self.commands {
      Commands::Tickers(tickers) => tickers.run(&mut ctx).await,
      Commands::Klines(klines) => klines.run(&mut ctx).await,
    }
  }
}