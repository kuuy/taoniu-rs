use clap::{Parser, Subcommand};

use crate::commands::queue::rsmq::binance::*;

pub mod binance;

#[derive(Parser)]
pub struct RsmqCommand {
  #[command(subcommand)]
  commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
  Binance(BinanceCommand),
}

impl RsmqCommand {
  pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
    match &self.commands {
      Commands::Binance(binance) => binance.run().await,
    }
  }
}