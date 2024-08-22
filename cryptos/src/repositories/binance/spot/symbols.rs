use redis::aio::MultiplexedConnection;

use crate::common::Rdb;

#[derive(Default)]
pub struct SymbolsRepository {}

impl SymbolsRepository {
  pub fn flush(&mut self) -> Result<(), Box<dyn std::error::Error>> {
    println!("symbols flush");
    if 1 > 0 {
      return Err(Box::from("symbols repository flush failed"))
    }
    Ok(())
  }
}
