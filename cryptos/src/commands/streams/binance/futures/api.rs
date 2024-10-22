use std::collections::HashMap;
use std::sync::Arc;

use async_nats::Subscriber;
use futures_util::{stream, StreamExt};
use redis::AsyncCommands;
use tokio::sync::Mutex;
use tokio::task::JoinSet;
use tokio_tungstenite::{tungstenite::Message, connect_async};
use clap::Parser;

use crate::common::*;
use crate::config::binance::futures::config as Config;
use crate::streams::api::requests::workers::binance::futures::FuturesWorker as FuturesRequest;
use crate::streams::api::responses::workers::binance::futures::FuturesWorker as FuturesResponse;
use crate::streams::api::ApiResponse;

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
    let endpoint = Env::var("BINANCE_FUTURES_API_STREAMS_ENDPOINT".to_string());
    println!("endpoint {endpoint:}");

    let (stream, _) = connect_async(&endpoint).await.expect("Failed to connect");
    let (writer, reader) = stream.split();
    let writer = Arc::new(Mutex::new(writer));
    let reader = Arc::new(Mutex::new(reader));
    println!("stream connected");

    let mut requests = HashMap::<&str, RequestFn>::new();
    FuturesRequest::new(ctx.clone()).subscribe(&mut requests).await?;
    let client = ctx.nats.clone();

    let mut subscribers: Vec<Subscriber> = Vec::new();
    for event in requests.keys() {
      subscribers.push(client.subscribe(&event[..]).await.unwrap());
    }

    let mut messages = stream::select_all(subscribers);

    let mut responses = HashMap::<&str, ResponseFn>::new();
    FuturesResponse::new(ctx.clone()).subscribe(&mut responses).await?;

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
      let mut rdb = ctx.rdb.lock().await.clone();
      async move {
        while let Some(message) = reader.next().await {
          match message.unwrap() {
            Message::Text(payload) => {
              let payload = std::str::from_utf8(&payload.as_bytes()).unwrap();
              match serde_json::from_str::<ApiResponse>(payload.as_ref()) {
                Ok(response) => {
                  if response.status == 200 {
                    let request: String = match rdb.hget(Config::REDIS_KEY_STREAMS_API, &response.id[..]).await {
                      Ok(Some(request)) => request,
                      Ok(None) => {
                        println!("request {} not exists", response.id);
                        "".to_string()
                      }
                      Err(err) => {
                        println!("error: {}", err);
                        "".to_string()
                      }
                    };
                    if request != "" {
                      let mut values: Vec<String> = request.split(",").map(|s|s.into()).collect();
                      if let Some(method) = responses.get(&values[0][..]) {
                        values.remove(0);
                        let _ = method(ctx.clone(), values, payload.into()).await;
                      }
                    }
                    println!("request {request}");
                  }
                  let _: bool = rdb.hdel(Config::REDIS_KEY_STREAMS_API, &response.id[..]).await.unwrap();
                  println!("response {} {}", response.id, response.status);
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
    let _ = workers.join_next().await;

    Ok(())
  }
}
