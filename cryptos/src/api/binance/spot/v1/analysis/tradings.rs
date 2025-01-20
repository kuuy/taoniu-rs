use axum::Router;

use crate::common::*;
use crate::api::binance::spot::v1::analysis::tradings::scalping::*;

mod scalping;

pub struct TradingsRouter {
  ctx: Ctx,
}

impl TradingsRouter {
  pub fn new(ctx: Ctx) -> Self {
    Self {
      ctx: ctx,
    }
  }

  pub fn routes(&self) -> Router {
    return Router::new()
      .nest("/scalping", ScalpingRouter::new(self.ctx.clone()).routes());
  }
}