use axum::Router;

use crate::common::*;
use crate::api::jwt::*;
use crate::api::jwe::*;
use crate::api::binance::futures::v1::tickers::*;
use crate::api::binance::futures::v1::positions::*;

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
      .nest("/tickers", TickersRouter::new(self.ctx.clone()).routes())
      .nest("/positions", PositionsRouter::new(self.ctx.clone()).routes())
      .layer(AuthenticatorLayer::new())
      .layer(EncryptionLayer::new());
  }
}
