use clap::{Parser, Subcommand};

mod app;
mod config;
mod common;
mod commands;
mod repositories;

use app::App;
use common::*;

#[tokio::main]
async fn main() {
  Env::load();
  match App::parse().run().await {
    Ok(_) => (),
    Err(e) => panic!("error {:?}", e),
  }
}
