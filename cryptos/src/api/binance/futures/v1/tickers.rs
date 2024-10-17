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
use crate::repositories::binance::futures::scalping::*;
use crate::repositories::binance::futures::tickers::*;

#[derive(Deserialize)]
struct GetsRequest {
  symbols: String,
  fields: String,
}

#[derive(Deserialize)]
struct RankingRequest {
  symbols: Option<String>,
  fields: String,
  sort: String,
  current: u32,
  page_size: u32,
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
    let tickers = TickersRepository::gets(ctx.clone(), symbols, fields).await;
    let response = RankingResponse{
      success: true,
      data: tickers.into_iter().map(|x| -> Box<dyn erased_serde::Serialize> { Box::new(x) }).collect(),
    };
    Ok(Json(serde_json::json!(response)))
  }

  async fn ranking(
    State(ctx): State<Ctx>,
    request: Query<RankingRequest>,
  ) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    if request.fields == "" {
      let message = ErrorMessage::new(false, "1004", "fields is empty");
      return Err((StatusCode::FORBIDDEN, Json(serde_json::json!(message))))
    }
    if request.sort == "" {
      let message = ErrorMessage::new(false, "1004", "sort is empty");
      return Err((StatusCode::FORBIDDEN, Json(serde_json::json!(message))))
    }

    let symbols = match &request.symbols {
      Some(value) => value.split(',').map(|s|s.into()).collect(),
      None => match ScalpingRepository::scan(ctx.clone()).await {
        Ok(values) => values,
        Err(_) => {
          let message = ErrorMessage::new(false, "1004", "symbols is empty");
          return Err((StatusCode::FORBIDDEN, Json(serde_json::json!(message))))
        },
      }
    };
    let symbols = symbols.iter().map(|s|&s[..]).collect();

    let fields: Vec<&str> = request.fields.split(',').collect();

    let mut sort = request.sort.splitn(2, ',');
    let sort_field = sort.next().unwrap();
    let sort_type = sort.next().unwrap();
    if !fields.iter().any(|&i| i == sort_field) {
      let message = ErrorMessage::new(false, "1004", "sort field not valid");
      return Err((StatusCode::FORBIDDEN, Json(serde_json::json!(message))))
    }
    let sort_type = sort_type.parse::<i32>().unwrap();
    if sort_type != -1 && sort_type != 1 {
      let message = ErrorMessage::new(false, "1004", "sort type not valid");
      return Err((StatusCode::FORBIDDEN, Json(serde_json::json!(message))))
    }

    let current = request.current;
    if current < 1 {
      let message = ErrorMessage::new(false, "1004", "current not valid");
      return Err((StatusCode::FORBIDDEN, Json(serde_json::json!(message))))
    }

    let page_size = request.page_size;
    if page_size < 1 || page_size > 100 {
      let message = ErrorMessage::new(false, "1004", "page size not valid");
      return Err((StatusCode::FORBIDDEN, Json(serde_json::json!(message))))
    }

    let tickers = TickersRepository::ranking(ctx.clone(), symbols, fields, sort_field, sort_type, current, page_size).await;
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
      .route("/ranking", get(Self::ranking))
      .with_state(ctx)
  }
}
