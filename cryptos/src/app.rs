use clap::{Parser, Subcommand};

use crate::commands::api::*;
use crate::commands::queue::*;
use crate::commands::cron::*;
use crate::commands::binance::*;

#[derive(Parser)]
pub struct App {
  #[command(subcommand)]
  commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
  Api(ApiCommand),
  Queue(QueueCommand),
  Cron(CronCommand),
  Binance(BinanceCommand),
}

impl App {
  pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
    match &self.commands {
      Commands::Api(api) => api.run().await,
      Commands::Queue(queue) => queue.run().await,
      Commands::Cron(cron) => cron.run().await,
      Commands::Binance(binance) => binance.run().await,
    }
  }
}