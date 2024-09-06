use axum::{
  routing::{get, post},
  Router,
};
use clap::{Parser};

use crate::common::*;
use crate::api::binance::spot::v1::*;

#[derive(Parser)]
pub struct SpotCommand {}

impl Default for SpotCommand {
  fn default() -> Self {
    Self::new()
  }
}

impl SpotCommand {
  pub fn new() -> Self {
    Self {
      ..Default::default()
    }
  }

  pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
    println!("api binance spot");
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", Env::int("CRYPTOS_API_BINANCE_SPOT_PORT")))
      .await
      .unwrap();

    let rdb = Rdb::new(1).await.unwrap();
    let rmq = Rmq::new(1).await.unwrap();
    let pool = Pool::new(1).unwrap();
    let nats = Nats::new().await.unwrap();
    let ctx = Ctx::new(rdb, rmq, pool, nats);

    let router = Router::new()
      .route("/foo", get(|| async { "Hi! spot api" }))
      .nest("/v1", V1Router::new(ctx).routes());

    axum::serve(listener, router).await.unwrap();

    Ok(())
  }
}
