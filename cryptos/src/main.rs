use clap::{Parser, Subcommand};

mod app;
mod common;
mod commands;

use app::App;
use common::Env;

#[tokio::main]
async fn main() {
  Env::load();
  match App::parse().run() {
    Ok(_) => (),
    Err(e) => panic!("error {:?}", e),
  }
}
