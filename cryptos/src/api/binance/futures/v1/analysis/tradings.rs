use axum::Router;

use crate::common::*;
use crate::api::binance::futures::v1::analysis::tradings::scalping::*;
use crate::api::binance::futures::v1::analysis::tradings::triggers::*;

mod scalping;
mod triggers;

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
      .nest("/scalping", ScalpingRouter::new(self.ctx.clone()).routes())
      .nest("/triggers", TriggersRouter::new(self.ctx.clone()).routes());
  }
}