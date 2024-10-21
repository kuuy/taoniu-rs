use std::collections::HashMap;
use std::sync::Arc;

use async_nats::Subscriber;
use futures_util::{stream, StreamExt};
use serde::Deserialize;
use tokio::sync::Mutex;
use tokio::task::JoinSet;
use tokio_tungstenite::{tungstenite::Message, connect_async};
use clap::Parser;

use crate::common::*;
use crate::streams::api::requests::workers::binance::spot::SpotWorker as SpotRequest;
use crate::streams::api::responses::workers::binance::spot::SpotWorker as SpotResponse;

#[derive(Parser)]
pub struct ApiCommand {}

impl Default for ApiCommand {
  fn default() -> Self {
    Self::new()
  }
}

impl ApiCommand {
  pub fn new() -> Self {
    Self {
      ..Default::default()
    }
  }

  pub async fn run(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    let endpoint = Env::var("BINANCE_SPOT_API_STREAMS_ENDPOINT".to_string());
    println!("endpoint {endpoint:}");

    let (stream, _) = connect_async(&endpoint).await.expect("Failed to connect");
    let (writer, reader) = stream.split();
    let writer = Arc::new(Mutex::new(writer));
    let reader = Arc::new(Mutex::new(reader));
    println!("stream connected");

    let mut requests = HashMap::<&str, StreamFn>::new();
    SpotRequest::new(ctx.clone()).subscribe(&mut requests).await?;
    let client = ctx.nats.clone();

    let mut subscribers: Vec<Subscriber> = Vec::new();
    for event in requests.keys() {
      subscribers.push(client.subscribe(&event[..]).await.unwrap());
    }

    let mut messages = stream::select_all(subscribers);

    let mut responses = HashMap::<&str, EventFn>::new();
    SpotResponse::new(ctx.clone()).subscribe(&mut responses).await?;

    let mut workers = JoinSet::new();
    workers.spawn(Box::pin({
      let ctx = ctx.clone();
      async move {
        while let Some(message) = messages.next().await {
          if let Some(method) = requests.get(&message.subject[..]) {
            let payload = std::str::from_utf8(&message.payload).unwrap();
            let _ = method(ctx.clone(), writer.clone(), payload.into()).await;
          }
        }
      }
    }));
    workers.spawn(Box::pin({
      let ctx = ctx.clone();
      let mut reader = reader.lock_owned().await;
      async move {
        while let Some(message) = reader.next().await {
          match message.unwrap() {
            Message::Text(content) => {
              let payload = std::str::from_utf8(&content.as_bytes()).unwrap();
              println!("response {payload:}");
            }
            Message::Close(_) => break,
            _ => continue,
          }
        }
      }
    }));
    let _ = workers.join_next().await;

    Ok(())
  }
}
