use axum::Router;

use crate::common::*;
use crate::api::jwt::*;
use crate::api::jwe::*;
use crate::api::binance::spot::v1::analysis::*;
use crate::api::binance::spot::v1::tickers::*;
use crate::api::binance::spot::v1::indicators::*;
use crate::api::binance::spot::v1::strategies::*;
use crate::api::binance::spot::v1::plans::*;
use crate::api::binance::spot::v1::positions::*;
use crate::api::binance::spot::v1::scalping::*;

mod analysis;
mod tickers;
mod indicators;
mod strategies;
mod plans;
mod positions;
mod orders;
mod scalping;
mod tradings;

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
      .nest("/analysis", AnalysisRouter::new(self.ctx.clone()).routes())
      .nest("/tickers", TickersRouter::new(self.ctx.clone()).routes())
      .nest("/indicators", IndicatorsRouter::new(self.ctx.clone()).routes())
      .nest("/strategies", StrategiesRouter::new(self.ctx.clone()).routes())
      .nest("/plans", PlansRouter::new(self.ctx.clone()).routes())
      .nest("/positions", PositionsRouter::new(self.ctx.clone()).routes())
      .nest("/scalping", ScalpingRouter::new(self.ctx.clone()).routes())
      .layer(AuthenticatorLayer::new())
      .layer(EncryptionLayer::new());
  }
}
