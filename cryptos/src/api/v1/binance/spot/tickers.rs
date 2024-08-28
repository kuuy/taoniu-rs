use axum::{
  routing::{get, post},
  http::StatusCode,
  Json, Router,
};
use clap::{Parser};

use crate::common::*;

pub struct TickersRouter {}

impl TickersRouter {
  pub fn new() -> Self {
    Self {}
  }

  pub async fn dispatch(&self) -> Result<(), Box<dyn std::error::Error>> {
    println!("api binance spot tickers router dispatch");
    Ok(())
  }
}
