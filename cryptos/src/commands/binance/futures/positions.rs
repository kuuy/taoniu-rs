use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct PositionsCommand {
  #[command(subcommand)]
  commands: Commands,
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

impl PositionsCommand {
  pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance futures positions run");
    Ok(())
  }
}