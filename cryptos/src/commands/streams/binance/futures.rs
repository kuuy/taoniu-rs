use clap::{Parser, Subcommand};

use crate::common::*;
use crate::commands::streams::binance::futures::tickers::*;

mod account;
mod tickers;
mod klines;

#[derive(Parser)]
pub struct FuturesCommand {
  #[command(subcommand)]
  commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
  Tickers(TickersCommand),
}

impl FuturesCommand {
  pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
    let rdb = Rdb::new(2).await.unwrap();
    let rmq = Rmq::new(2).await.unwrap();
    let pool = Pool::new(2).unwrap();
    let nats = Nats::new().await.unwrap();
    let ctx = Ctx::new(rdb, rmq, pool, nats);
    match &self.commands {
      Commands::Tickers(tickers) => tickers.run(ctx).await,
    }
  }
}