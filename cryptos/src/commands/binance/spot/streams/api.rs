use std::time::Duration;

use clap::{Parser, Subcommand};

use crate::common::*;
use crate::commands::binance::spot::streams::api::klines::*;

pub mod klines;

#[derive(Parser)]
pub struct ApiCommand {
  #[command(subcommand)]
  commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
  Klines(KlinesCommand),
}

impl ApiCommand {
  pub async fn run(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    match &self.commands {
      Commands::Klines(klines) => klines.run(ctx).await,
    }
  }
}