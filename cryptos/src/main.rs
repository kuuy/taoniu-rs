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
  let mut rdb = Rdb::new(1).await.expect("redis connect failed");
  match App::parse().run(&mut rdb).await {
    Ok(_) => (),
    Err(e) => panic!("error {:?}", e),
  }
}
