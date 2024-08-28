use axum::{
  routing::{get, post},
  http::StatusCode,
  Json, Router,
};
use clap::{Parser};

use crate::common::*;

pub mod spot;
pub mod futures;

pub struct BinanceRouter {}

impl BinanceRouter {
  pub fn new() -> Self {
    Self {}
  }

  pub async fn dispatch(&self) -> Result<(), Box<dyn std::error::Error>> {
    println!("api binance router dispatch");
    Ok(())
  }
}
