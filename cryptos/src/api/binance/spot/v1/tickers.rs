use axum::{
  routing::{get, post},
  http::StatusCode,
  Json, 
  Router,
};
use clap::{Parser};

use crate::common::*;

pub struct TickersRouter {
  ctx: Ctx,
}

impl TickersRouter {
  pub fn new(ctx: Ctx) -> Self {
    Self {
      ctx: ctx,
    }
  }

  pub fn routes(&self) -> Router {
    return Router::new()
      .route("/foo", get(|| async { "Hi! tickers" }));
  }
}
