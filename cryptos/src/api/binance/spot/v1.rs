use axum::{
  routing::{get, post},
  http::StatusCode,
  Json,
  Router,
};
use clap::{Parser};

use crate::common::*;
use crate::api::jwt::*;
use crate::api::binance::spot::v1::tickers::*;

pub mod tickers;

pub struct V1Router {
  ctx: Ctx,
}

impl V1Router {
  pub fn new(ctx: Ctx) -> Self {
    Self {
      ctx: ctx,
    }
  }

  pub fn routes(&self) -> Router {
    return Router::new()
      .route("/foo", get(|| async { "Hi! v1 router" }))
      .nest("/tickers", TickersRouter::new(self.ctx.clone()).routes())
      .layer(AuthenticatorLayer::new());
  }
}
