use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct SymbolsCommands {
  #[command(subcommand)]
  subcommands: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
  /// does testing things
  Test {
    /// lists test values
    #[arg(short, long)]
    list: bool,
  },
}

impl SymbolsCommands {
  pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance futures symbols run");
    Ok(())
  }
}