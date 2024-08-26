use clap::{Parser, Subcommand};

use crate::commands::queue::nats::binance::*;

pub mod binance;

#[derive(Parser)]
pub struct NatsCommand {
  #[command(subcommand)]
  commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
  Binance(BinanceCommand),
}

impl NatsCommand {
  pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
    match &self.commands {
      Commands::Binance(binance) => binance.run().await,
    }
  }
}