use clap::Parser;

#[derive(Parser)]
pub struct PositionsCommand {
}

impl PositionsCommand {
  pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
    println!("binance futures positions run");
    Ok(())
  }
}