use clap::{Parser, Subcommand};

use crate::common::*;
use crate::commands::binance::spot::indicators::nats::*;

pub mod nats;

#[derive(Parser)]
pub struct IndicatorsCommand {
  #[command(subcommand)]
  commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
  Nats(NatsCommand),
}

impl IndicatorsCommand {
  pub async fn run(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    match &self.commands {
      Commands::Nats(nats) => nats.run(ctx).await,
    }
  }
}