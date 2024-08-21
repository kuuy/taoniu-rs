use clap::{Parser, Subcommand};

pub mod symbols;
pub mod positions;

pub use symbols::*;
pub use positions::*;

#[derive(Parser)]
pub struct SpotCommands {
  #[command(subcommand)]
  commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
  Symbols(SymbolsCommands),
  Positions(PositionsCommands),
}