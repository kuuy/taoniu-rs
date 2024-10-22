use clap::{Parser, Subcommand};

use crate::common::*;

#[derive(Parser)]
pub struct ApiCommand {
  #[command(subcommand)]
  commands: Commands,
}

#[derive(Subcommand)]
enum Commands {}

impl ApiCommand {
  pub async fn run(&self, _: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
  }
}