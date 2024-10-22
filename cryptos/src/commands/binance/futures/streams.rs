use clap::{Parser, Subcommand};

use crate::common::*;
use crate::commands::binance::futures::streams::api::*;

pub mod api;

#[derive(Parser)]
pub struct StreamsCommand {
  #[command(subcommand)]
  commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
  Api(ApiCommand),
}

impl StreamsCommand {
  pub async fn run(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    match &self.commands {
      Commands::Api(api) => api.run(ctx).await,
    }
  }
}