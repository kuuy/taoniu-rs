use clap::{Parser, Subcommand};

use crate::commands::tasks::binance::*;

pub mod binance;

#[derive(Parser)]
pub struct TasksCommand {
  #[command(subcommand)]
  commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
  Binance(BinanceCommand),
}

impl TasksCommand {
  pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
    match &self.commands {
      Commands::Binance(binance) => binance.run().await,
    }
  }
}