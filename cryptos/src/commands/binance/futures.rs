use clap::{Parser, Subcommand};

pub mod symbols;
pub mod positions;

pub use symbols::*;
pub use positions::*;

#[derive(Parser)]
pub struct FuturesCommand {
  #[command(subcommand)]
  commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
  Symbols(SymbolsCommand),
  Positions(PositionsCommand),
}

impl FuturesCommand {
  pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
    match &self.commands {
      Commands::Symbols(symbols) => symbols.run(),
      Commands::Positions(positions) => positions.run(),
    }
  }
}