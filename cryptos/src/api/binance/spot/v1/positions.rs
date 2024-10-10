use axum::{
  extract::{State, Query},
  routing::get,
  http::StatusCode,
  Json,
  Router,
};
use serde::{Serialize, Deserialize};

use crate::common::*;
use crate::api::response::*;

#[derive(Deserialize)]
struct ListingsRequest {
  current: u32,
  page_size: u32,
}

pub struct PositionsRouter {
  ctx: Ctx,
}

#[derive(Serialize)]
pub struct PositionInfo {}

impl PositionsRouter {
  pub fn new(ctx: Ctx) -> Self {
    Self {
      ctx: ctx,
    }
  }

  async fn listings(
    State(ctx): State<Ctx>,
    request: Query<ListingsRequest>,
  ) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let _ = ctx.clone();
    let current = request.current;
    let page_size = request.page_size;

    let position_info = PositionInfo{};
    let response = PagenateResponse{
      success: true,
      data: vec![Box::new(position_info)],
      total: 10,
      current: current,
      page_size: page_size,
    };
    Ok(Json(serde_json::json!(response)))
  }

  pub fn routes(&self) -> Router {
    let ctx = self.ctx.clone();
    return Router::new()
      .route("/", get(Self::listings))
      .with_state(ctx);
  }
}
