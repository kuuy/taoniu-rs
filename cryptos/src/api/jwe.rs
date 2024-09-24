use std::task::{Context, Poll};

use axum::{
  http::Request,
};
use tower::{Layer, Service};

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

impl<S, Body, Response> Service<Request<Body>> for JweMiddleware<S>
where
  S: Service<Request<Body>, Response = Response>,
{
  type Response = S::Response;
  type Error = S::Error;
  type Future = S::Future;

  #[inline]
  fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
    self.inner.poll_ready(cx)
  }

  #[inline]
  fn call(&mut self, request: Request<Body>) -> Self::Future {
    println!("Hi jwe middleware");
    self.inner.call(request)
  }
}
