use axum::Router;

use crate::common::*;
use crate::api::binance::spot::v1::analysis::tradings::*;

mod tradings;

pub struct AnalysisRouter {
  ctx: Ctx,
}

impl AnalysisRouter {
  pub fn new(ctx: Ctx) -> Self {
    Self {
      ctx: ctx,
    }
  }

  pub fn routes(&self) -> Router {
    return Router::new()
      .nest("/tradings", TradingsRouter::new(self.ctx.clone()).routes());
  }
}