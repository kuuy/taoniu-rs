use clap::{Parser, Subcommand};

use crate::commands::binance::*;
use crate::commands::queue::*;
use crate::commands::cron::*;

#[derive(Parser)]
pub struct App {
  #[command(subcommand)]
  commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
  Binance(BinanceCommand),
  Queue(QueueCommand),
  Cron(CronCommand),
}

impl App {
  pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
    match &self.commands {
      Commands::Binance(binance) => binance.run().await,
      Commands::Queue(queue) => queue.run().await,
      Commands::Cron(cron) => cron.run().await,
    }
  }
}