use clap::{Parser, Subcommand};

use crate::common::*;
use crate::commands::tasks::binance::spot::klines::*;

mod klines;

#[derive(Parser)]
pub struct SpotCommand {
  #[command(subcommand)]
  commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
  Klines(KlinesCommand),
}

impl SpotCommand {
  pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
    let rdb = Rdb::new(1).await.expect("bad rdb connection");
    let rmq = Rmq::new(1).await.expect("bad rmq connection");
    let pool = Pool::new(1).expect("bad pool connection");
    let nats = Nats::new().await.expect("bad nats connection");
    let ctx = Ctx::new(rdb, rmq, pool, nats);
    match &self.commands {
      Commands::Klines(klines) => klines.run(ctx).await,
    }
  }
}