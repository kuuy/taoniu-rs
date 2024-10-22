use serde::{Serialize, Deserialize};

use crate::common::*;

pub mod requests;
pub mod responses;

#[derive(Serialize)]
pub struct ApiRequest {
  pub id: String,
  pub method: String,
  pub params: Box<dyn erased_serde::Serialize + Send + Sync + 'static>,
}

#[derive(Deserialize)]
pub struct ApiResponse {
  pub id: String,
  pub status: i32,
}
