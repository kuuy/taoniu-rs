use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use axum::{
  http::Request,
};
use tower::{Layer, Service};

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

impl<S, Body, Response> Service<Request<Body>> for JwtMiddleware<S>
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
    let _ = match request.headers().get("Authorization")
      .and_then(|header| header.to_str().ok()) {
      Some(bearer) => {
        if bearer.starts_with("Taoniu") {
          match TokenRepository::uid(&bearer[7..]) {
            Ok(uid) => {
              println!("Hi jwt middleware uid {uid:}");
            }
            Err(err) => {
              println!("jwt middleware error {err:?}");
            }
          }
        }
        false
      }
      None => false,
    };
    let future = self.inner.call(request);
    Box::pin({
      async move {
        let response: Response = future.await?;
        Ok(response)
      }
    })
  }
}
