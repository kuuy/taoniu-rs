use redis::aio::MultiplexedConnection;
use clap::{Parser, Subcommand};

use crate::common::Rdb;
use crate::commands::binance::*;

#[derive(Parser)]
pub struct App {
  #[command(subcommand)]
  commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
  Binance(BinanceCommand),
}

impl App {
  pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
    match &self.commands {
      Commands::Binance(binance) => binance.run().await,
    }
  }
}