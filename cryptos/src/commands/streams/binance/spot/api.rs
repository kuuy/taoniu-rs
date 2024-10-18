use std::time::Duration;
use std::sync::Arc;
use futures_util::stream::StreamExt;

use chrono::prelude::Utc;
use rust_decimal::prelude::*;
use redis::AsyncCommands;
use serde::{Deserialize, Deserializer};
use tokio_tungstenite::{tungstenite::Message, connect_async};
use clap::Parser;

use crate::common::*;
use crate::config::binance::spot::config as Config;
use crate::repositories::binance::spot::scalping::*;

#[derive(Parser)]
pub struct ApiCommand {}

impl Default for ApiCommand {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(Deserialize)]
struct ApiRequest {
  id: String,
  method: String,
  params: ApiParams,
}

#[derive(Deserialize)]
struct ApiParams {}

#[derive(Deserialize)]
struct ApiResponse {
  id: String,
  status: i32,
  result: ApiResult,
}

#[derive(Deserialize)]
struct ApiResult {}

impl ApiCommand {
  pub fn new() -> Self {
    Self {
      ..Default::default()
    }
  }

  async fn response(ctx: Ctx, response: ApiResponse) -> Result<(), Box<dyn std::error::Error>> {
    println!("process response {} {}", response.id, response.status);
    Ok(())
  }

  async fn request(ctx: Ctx, request: ApiRequest) -> Result<(), Box<dyn std::error::Error>> {
    println!("process request {} {}", request.id, request.method);
    Ok(())
  }

  pub async fn run(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    let endpoint = Env::var("BINANCE_SPOT_API_STREAMS_ENDPOINT".to_string());
    println!("endpoint {endpoint:}");

    let (stream, _) = connect_async(&endpoint).await.expect("Failed to connect");
    let (mut write, read) = stream.split();
    let read = Arc::new(tokio::sync::Mutex::new(read));
    println!("stream connected");
    let handle = tokio::spawn(Box::pin({
      let ctx = ctx.clone();
      let mut read = read.lock_owned().await;
      async move {
        while let Some(message) = read.next().await {
          match message.unwrap() {
            Message::Text(content) => {
              match serde_json::from_str::<ApiResponse>(&content) {
                Ok(response) => {
                  let _ = Self::response(ctx.clone(), response).await;
                }
                Err(err) => println!("error: {}", err)
              }
            }
            Message::Close(_) => break,
            _ => continue,
          }
        }
      }
    }));
    handle.await.expect("The read task failed.");

    Ok(())
  }
}
