use axum::{
  routing::get,
  http::StatusCode,
  Json, 
  Router,
};
use serde::Serialize;

use crate::common::*;
use crate::api::response::*;

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

  pub async fn gets() -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let position_info = PositionInfo{};
    let response = PagenateResponse{
      success: true,
      data: vec![Box::new(position_info)],
      total: 10,
      current: 1,
      page_size: 1,
    };
    Ok(Json(serde_json::json!(response)))
  }

  pub fn routes(&self) -> Router {
    return Router::new()
      .route("/", get(Self::gets));
  }
}
