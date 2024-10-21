use serde::Serialize;

pub mod requests;
pub mod responses;

#[derive(Serialize)]
pub struct ApiRequest {
  pub id: String,
  pub method: String,
  pub params: Box<dyn erased_serde::Serialize + Send + Sync + 'static>,
}
