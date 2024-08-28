use axum::{
  routing::{get, post},
  http::StatusCode,
  Json, Router,
};
use clap::{Parser};

use crate::common::*;

pub mod binance;

pub struct V1Router {}

impl V1Router {
  pub fn new() -> Self {
    Self {}
  }

  pub async fn dispatch(&self) -> Result<(), Box<dyn std::error::Error>> {
    println!("api binance spot router dispatch");
    Ok(())
  }
}
