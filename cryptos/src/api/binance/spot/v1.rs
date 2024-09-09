use axum::{
  routing::{get, post},
  http::StatusCode,
  Json,
  Router,
};
use clap::{Parser};

use crate::common::*;
use crate::api::jwt::*;
use crate::api::jwe::*;
use crate::api::binance::spot::v1::tickers::*;
use crate::api::binance::spot::v1::positions::*;

pub mod tickers;
pub mod positions;

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
      .nest("/positions", PositionsRouter::new(self.ctx.clone()).routes())
      .layer(EncryptionLayer::new())
      .layer(AuthenticatorLayer::new());
  }
}
