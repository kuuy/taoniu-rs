use clap::Parser;

#[derive(Parser)]
pub struct PositionsCommands {
}

impl PositionsCommands {
  pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance futures positions run");
    Ok(())
  }
}