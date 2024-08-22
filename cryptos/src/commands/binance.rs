use clap::{Parser, Subcommand};

pub mod spot;
pub mod futures;

pub use spot::*;
pub use futures::*;

#[derive(Parser)]
pub struct BinanceCommands {
  #[command(subcommand)]
  commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
  Spot(SpotCommands),
  Futures(FuturesCommands),
}

impl BinanceCommands {
  pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
    match &self.commands {
      Commands::Spot(spot) => spot.run(),
      Commands::Futures(futures) => futures.run(),
    }
  }
}