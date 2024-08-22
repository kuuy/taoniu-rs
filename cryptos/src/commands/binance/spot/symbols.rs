use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct SymbolsCommands {
  #[command(subcommand)]
  commands: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
  /// symbols flush
  Flush,
}

impl SymbolsCommands {
  fn flush(&self) -> Result<(), Box<dyn std::error::Error>> {
    println!("symbols flush");
    if 1 > 0 {
      return Err(Box::from("symbols flush failed"))
    }
    Ok(())
  }
}

impl SymbolsCommands {
  pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
    match &self.commands {
      Commands::Flush => self.flush(),
    }
  }
}