use clap::{Parser, Subcommand};

use crate::common::*;
use crate::commands::binance::futures::tradings::scalping::*;
use crate::commands::binance::futures::tradings::triggers::*;

pub mod scalping;
pub mod triggers;

#[derive(Parser)]
pub struct TradingsCommand {
  #[command(subcommand)]
  commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
  Scalping(ScalpingCommand),
  Triggers(TriggersCommand),
}

impl TradingsCommand {
  pub async fn run(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    match &self.commands {
      Commands::Scalping(scalping) => scalping.run(ctx.clone()).await,
      Commands::Triggers(triggers) => triggers.run(ctx.clone()).await,
    }
  }
}