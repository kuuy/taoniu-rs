use clap::{Parser, Subcommand};

use crate::commands::queue::nats::*;
use crate::commands::queue::rsmq::*;

pub mod nats;
pub mod rsmq;

#[derive(Parser)]
pub struct QueueCommand {
  #[command(subcommand)]
  commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
  Nats(NatsCommand),
  Rsmq(RsmqCommand),
}

impl QueueCommand {
  pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
    match &self.commands {
      Commands::Nats(nats) => nats.run().await,
      Commands::Rsmq(rsmq) => rsmq.run().await,
    }
  }
}