use clap::{Parser, Subcommand};

use crate::commands::api::binance::*;

pub mod binance;

#[derive(Parser)]
pub struct ApiCommand {
  #[command(subcommand)]
  commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
  Binance(BinanceCommand),
}

impl ApiCommand {
  pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
    match &self.commands {
      Commands::Binance(spot) => spot.run().await,
    }
  }
}