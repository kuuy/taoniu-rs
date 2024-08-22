use redis::aio::MultiplexedConnection;
use clap::{Parser, Subcommand};

pub mod spot;
pub mod futures;

pub use spot::*;
pub use futures::*;

#[derive(Parser)]
pub struct BinanceCommand {
  #[command(subcommand)]
  commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
  Spot(SpotCommand),
  Futures(FuturesCommand),
}

impl<'a> BinanceCommand {
  pub async fn run(&self, rdb: &'a mut MultiplexedConnection) -> Result<(), Box<dyn std::error::Error>> {
    match &self.commands {
      Commands::Spot(spot) => spot.run(rdb).await,
      Commands::Futures(futures) => futures.run(),
    }
  }
}