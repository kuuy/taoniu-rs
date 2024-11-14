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
use crate::repositories::binance::futures::indicators::*;

#[derive(Deserialize)]
struct GetsRequest {
  symbols: String,
  interval: String,
  fields: String,
}

#[derive(Deserialize)]
struct RankingRequest {
  symbols: Option<String>,
  interval: String,
  fields: String,
  sort: String,
  current: u32,
  page_size: u32,
}

pub struct IndicatorsRouter {
  ctx: Ctx,
}

impl IndicatorsRouter {
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
    let interval = request.interval.as_ref();
    let fields = request.fields.split(',').collect();
    let indicators = IndicatorsRepository::gets(ctx.clone(), symbols, interval, fields).await;
    let response = RankingResponse{
      success: true,
      data: indicators.into_iter().map(|x| -> Box<dyn erased_serde::Serialize> { Box::new(x) }).collect(),
    };
    Ok(Json(serde_json::json!(response)))
  }

  async fn ranking(
    State(ctx): State<Ctx>,
    request: Query<RankingRequest>,
  ) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    if request.interval == "" {
      let message = ErrorMessage::new(false, "1004", "interval is empty");
      return Err((StatusCode::FORBIDDEN, Json(serde_json::json!(message))))
    }
    if request.fields == "" {
      let message = ErrorMessage::new(false, "1004", "fields is empty");
      return Err((StatusCode::FORBIDDEN, Json(serde_json::json!(message))))
    }
    if request.sort == "" {
      let message = ErrorMessage::new(false, "1004", "sort is empty");
      return Err((StatusCode::FORBIDDEN, Json(serde_json::json!(message))))
    }

    let mut symbols = match &request.symbols {
      Some(value) => {
        if value != "" {
          value.split(',').map(|s|s.into()).collect()
        } else {
          vec![]
        }
      }
      None => vec![],
    };
    if symbols.is_empty() {
      symbols = match ScalpingRepository::scan(ctx.clone(), 2).await {
        Ok(values) => values,
        Err(_) => {
          let message = ErrorMessage::new(false, "1004", "symbols is empty");
          return Err((StatusCode::FORBIDDEN, Json(serde_json::json!(message))))
        },
      };
    }
    let symbols = symbols.iter().map(|s|&s[..]).collect();
    let interval = request.interval.as_ref();

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

    let indicators = IndicatorsRepository::ranking(
      ctx.clone(),
      symbols,
      interval,
      fields,
      sort_field,
      sort_type,
      current,
      page_size,
    ).await;
    let response = RankingResponse{
      success: true,
      data: indicators.into_iter().map(|x| -> Box<dyn erased_serde::Serialize> { Box::new(x) }).collect(),
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
