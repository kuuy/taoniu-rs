use clap::Parser;

mod api;
mod app;
mod config;
mod common;
mod commands;
mod models;
mod repositories;
mod tasks;
mod queue;

use app::App;
use common::Env;

#[tokio::main]
async fn main() {
  Env::load();
  match App::parse().run().await {
    Ok(_) => (),
    Err(e) => panic!("error {:?}", e),
  }
}
