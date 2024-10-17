use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use axum::{
  body::Body,
  http::{Request, StatusCode},
  response::Response,
};
use http_body_util::BodyExt;
use tower::{Layer, Service};

use crate::api::response::*;
use crate::repositories::jwe::JweRepository;

#[derive(Clone)]
pub struct EncryptionLayer {}

impl EncryptionLayer {
  pub fn new() -> Self {
    Self {}
  }
}

impl<S> Layer<S> for EncryptionLayer {
  type Service = JweMiddleware<S>;

  fn layer(&self, inner: S) -> Self::Service {
    JweMiddleware {
      inner,
    }
  }
}

#[derive(Clone)]
pub struct JweMiddleware<S> {
  inner: S,
}

impl<S> Service<Request<Body>> for JweMiddleware<S>
where
  S: Service<Request<Body>, Response = Response> + Clone + Send + 'static,
  S::Future: Send + 'static,
{
  type Response = S::Response;
  type Error = S::Error;
  type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

  #[inline]
  fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
    self.inner.poll_ready(cx)
  }

  #[inline]
  fn call(&mut self, request: Request<Body>) -> Self::Future {
    Box::pin({
      let mut inner = self.inner.clone();
      async move {
        let (parts, body) = request.into_parts();

        let request = match body.collect().await {
          Ok(payload) => {
            let payload = payload.to_bytes();
            if payload.is_empty() {
              Request::from_parts(parts, Body::empty())
            } else {
              let payload = std::str::from_utf8(&payload[..]).unwrap();
              match JweRepository::decrypt(payload) {
                Ok(result) => {
                  Request::from_parts(parts, Body::from(result))
                },
                Err(err) => {
                  let response = ErrorResponse::json(StatusCode::INTERNAL_SERVER_ERROR, "500", &err.to_string());
                  return Ok(response)
                },
              }
            }
          },
          Err(err) => {
            let response = ErrorResponse::json(StatusCode::INTERNAL_SERVER_ERROR, "500", &err.to_string());
            return Ok(response)
          },
        };

        let future = inner.call(request);
        let response = future.await?;
        let (mut parts, body) = response.into_parts();
        let response = match body.collect().await {
          Ok(payload) => {
            match JweRepository::encrypt(&payload.to_bytes().to_vec()) {
              Ok(result) => {
                parts.headers.insert("content-length", result.as_bytes().len().into());
                Response::from_parts(parts, Body::from(result))
              },
              Err(err) => ErrorResponse::json(StatusCode::INTERNAL_SERVER_ERROR, "500", &err.to_string()),
            }
          },
          Err(err) => ErrorResponse::json(StatusCode::INTERNAL_SERVER_ERROR, "500", &err.to_string()),
        };
        Ok(response)
      }
    })
  }
}
