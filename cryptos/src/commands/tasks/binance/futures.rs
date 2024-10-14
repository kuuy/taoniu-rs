use clap::{Parser, Subcommand};

use crate::common::*;
use crate::commands::tasks::binance::futures::klines::*;

mod klines;

#[derive(Parser)]
pub struct FuturesCommand {
  #[command(subcommand)]
  commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
  Klines(KlinesCommand),
}

impl FuturesCommand {
  pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
    let rdb = Rdb::new(2).await.expect("bad rdb connection");
    let rmq = Rmq::new(2).await.expect("bad rmq connection");
    let pool = Pool::new(2).expect("bad pool connection");
    let nats = Nats::new().await.expect("bad nats connection");
    let ctx = Ctx::new(rdb, rmq, pool, nats);
    match &self.commands {
      Commands::Klines(klines) => klines.run(ctx).await,
    }
  }
}