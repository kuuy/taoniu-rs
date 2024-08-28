use clap::{Parser, Subcommand};

use crate::commands::streams::binance::*;

pub mod binance;

#[derive(Parser)]
pub struct StreamsCommand {
  #[command(subcommand)]
  commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
  Binance(BinanceCommand),
}

impl StreamsCommand {
  pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
    match &self.commands {
      Commands::Binance(spot) => spot.run().await,
    }
  }
}