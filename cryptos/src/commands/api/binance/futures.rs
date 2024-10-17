use axum::{
  routing::get,
  Router,
};
use clap::Parser;

use crate::common::*;
use crate::api::binance::futures::v1::*;

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
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", Env::int("CRYPTOS_API_BINANCE_FUTURES_PORT")))
      .await
      .unwrap();

    let rdb = Rdb::new(2).await?;
    let rmq = Rmq::new(2).await?;
    let pool = Pool::new(2)?;
    let nats = Nats::new().await?;
    let ctx = Ctx::new(rdb, rmq, pool, nats);

    let router = Router::new()
      .route("/foo", get(|| async { "Hi! futures api" }))
      .nest("/v1", V1Router::new(ctx).routes());

    axum::serve(listener, router).await.unwrap();

    Ok(())
  }
}
