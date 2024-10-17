use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use axum::{
  body::Body,
  http::{Request, StatusCode},
  response::Response,
};
use tower::{Layer, Service};

use crate::api::response::*;
use crate::repositories::auth::token::*;

#[derive(Clone)]
pub struct AuthenticatorLayer {}

impl AuthenticatorLayer {
  pub fn new() -> Self {
    Self {}
  }
}

impl<S> Layer<S> for AuthenticatorLayer {
  type Service = JwtMiddleware<S>;

  fn layer(&self, inner: S) -> Self::Service {
    JwtMiddleware {
      inner,
    }
  }
}

#[derive(Clone)]
pub struct JwtMiddleware<S> {
  inner: S,
}

impl<S> Service<Request<Body>> for JwtMiddleware<S>
where
  S: Service<Request<Body>, Response = Response>,
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
    let uid = match request.headers().get("Authorization")
      .and_then(|header| header.to_str().ok()) {
      Some(bearer) => {
        if bearer.starts_with("Taoniu") {
          match TokenRepository::uid(&bearer[7..]) {
            Ok(result) => Some(result),
            Err(_) => None
          }
        } else {
          None
        }
      }
      None => None,
    };

    let future = self.inner.call(request);

    Box::pin({
      async move {
        let response = match uid {
          Some(uid) => {
            if uid == "" {
              ErrorResponse::json(StatusCode::UNAUTHORIZED, "401", "access not authorized")
            } else {
              future.await?
            }
          },
          None => ErrorResponse::json(StatusCode::FORBIDDEN, "403", "access not allowed")
        };
        Ok(response)
      }
    })
  }
}
