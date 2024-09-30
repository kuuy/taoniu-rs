use clap::{Parser, Subcommand};

use crate::common::*;
use crate::repositories::binance::futures::scalping::plans::*;

#[derive(Parser)]
pub struct ScalpingCommand {
  #[command(subcommand)]
  commands: Commands,
}

impl Default for ScalpingCommand {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(Subcommand)]
enum Commands {
  /// scalping place
  Place,
  /// scalping flush
  Flush,
}

impl ScalpingCommand {
  pub fn new() -> Self {
    Self {
      ..Default::default()
    }
  }

  async fn place(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance futures tradings scalping place");
    let plan_ids = PlansRepository::scan(ctx.clone()).await.unwrap();
    for plan_id in plan_ids.iter() {
      println!("plan_id {plan_id:}");
    }
    Ok(())
  }

  async fn flush(&self, _: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance futures tradings scalping flush");
    Ok(())
  }

  pub async fn run(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    match &self.commands {
      Commands::Place => self.place(ctx.clone()).await,
      Commands::Flush => self.flush(ctx.clone()).await,
    }
  }
}
