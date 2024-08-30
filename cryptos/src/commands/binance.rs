use clap::{Parser, Subcommand};

use crate::commands::binance::spot::*;
use crate::commands::binance::futures::*;

pub mod spot;
pub mod futures;
pub mod margin;

#[derive(Parser)]
pub struct BinanceCommand {
  #[command(subcommand)]
  commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
  Spot(SpotCommand),
  Futures(FuturesCommand),
}

impl BinanceCommand {
  pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
    match &self.commands {
      Commands::Spot(spot) => spot.run().await,
      Commands::Futures(futures) => futures.run().await,
    }
  }
}