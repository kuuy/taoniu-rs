use clap::{Parser, Subcommand};

pub mod spot;
pub mod futures;

pub use spot::*;

#[derive(Parser)]
pub struct BinanceCommands {
  #[command(subcommand)]
  subcommands: Commands,
}

#[derive(Subcommand)]
enum Commands {
  Spot(SpotCommands),
}
