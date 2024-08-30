use clap::Parser;

pub mod api;
pub mod app;
pub mod config;
pub mod common;
pub mod commands;
pub mod models;
pub mod schema;
pub mod repositories;
pub mod cron;
pub mod queue;

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
