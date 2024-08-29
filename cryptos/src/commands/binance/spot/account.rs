use clap::{Parser, Subcommand};

use crate::common::*;
use crate::commands::binance::spot::account::rsmq::*;

pub mod rsmq;

#[derive(Parser)]
pub struct AccountCommand {
  #[command(subcommand)]
  commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
  Rsmq(RsmqCommand),
}

impl AccountCommand {
  pub async fn run(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    match &self.commands {
      Commands::Rsmq(nats) => nats.run(ctx).await,
    }
  }
}