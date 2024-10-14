use clap::{Parser, Subcommand};

use crate::commands::cron::binance::*;

pub mod binance;

#[derive(Parser)]
pub struct CronCommand {
  #[command(subcommand)]
  commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
  Binance(BinanceCommand),
}

impl CronCommand {
  pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
    match &self.commands {
      Commands::Binance(binance) => binance.run().await,
    }
  }
}