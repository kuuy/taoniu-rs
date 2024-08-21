use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct PositionsCommands {
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