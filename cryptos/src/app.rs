use redis::aio::MultiplexedConnection;
use clap::{Parser, Subcommand};

use crate::common::Rdb;
use crate::commands::*;

#[derive(Parser)]
pub struct App {
  #[command(subcommand)]
  commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
  Binance(BinanceCommand),
}

impl<'a> App {
  pub async fn run(&self, rdb: &'a mut MultiplexedConnection) -> Result<(), Box<dyn std::error::Error>> {
    match &self.commands {
      Commands::Binance(binance) => binance.run(rdb).await,
    }
  }
}