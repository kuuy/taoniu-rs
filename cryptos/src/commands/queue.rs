use clap::{Parser, Subcommand};

use crate::commands::queue::nats::*;

pub mod nats;

#[derive(Parser)]
pub struct QueueCommand {
  #[command(subcommand)]
  commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
  Nats(NatsCommand),
}

impl QueueCommand {
  pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
    match &self.commands {
      Commands::Nats(nats) => nats.run().await,
    }
  }
}