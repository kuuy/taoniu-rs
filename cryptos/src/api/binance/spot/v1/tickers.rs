use axum::{
  extract::{State, Query},
  routing::get,
  http::StatusCode,
  Json, 
  Router,
};
use serde::Deserialize;

use crate::common::*;
use crate::api::response::*;
use crate::repositories::binance::spot::tickers::*;

#[derive(Deserialize)]
struct GetsRequest {
  symbols: String,
  fields: String,
}

pub struct TickersRouter {
  ctx: Ctx,
}

impl TickersRouter {
  pub fn new(ctx: Ctx) -> Self {
    Self {
      ctx: ctx,
    }
  }

  async fn gets(
    State(ctx): State<Ctx>,
    request: Query<GetsRequest>,
  ) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let symbols = request.symbols.split(',').collect();
    let fields = request.fields.split(',').collect();
    let tickers = TickersRepository::ranking(ctx.clone(), symbols, fields).await;
    let response = RankingResponse{
      success: true,
      data: tickers.into_iter().map(|x| -> Box<dyn erased_serde::Serialize> { Box::new(x) }).collect(),
    };
    Ok(Json(serde_json::json!(response)))
  }

  pub fn routes(&self) -> Router {
    let ctx = self.ctx.clone();
    return Router::new()
      .route("/", get(Self::gets))
      .with_state(ctx)
  }
}
