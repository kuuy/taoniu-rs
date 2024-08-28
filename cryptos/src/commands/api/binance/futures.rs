use axum::Router;
use clap::{Parser};

use crate::common::*;

#[derive(Parser)]
pub struct FuturesCommand {}

impl Default for FuturesCommand {
  fn default() -> Self {
    Self::new()
  }
}

impl FuturesCommand {
  pub fn new() -> Self {
    Self {
      ..Default::default()
    }
  }

  pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
    println!("api binance futures");
    let router = Router::new();
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", Env::int("CRYPTOS_API_BINANCE_FUTURES_PORT")))
      .await
      .unwrap();
    axum::serve(listener, router).await.unwrap();
    Ok(())
  }
}
