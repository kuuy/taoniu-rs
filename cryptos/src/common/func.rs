use std::sync::Arc;
use std::future::Future;
use std::pin::Pin;

use futures_util::stream::SplitSink;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};

use crate::common::Ctx;

pub type EventFn = Box<dyn Fn(Ctx, String) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error>>> + Send + 'static >> + Send + Sync + 'static>;
pub type RequestFn = Box<dyn Fn(Ctx, Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>, String) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error>>> + Send + 'static >> + Send + Sync + 'static>;
pub type ResponseFn = Box<dyn Fn(Ctx, Vec<String>, String) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error>>> + Send + 'static >> + Send + Sync + 'static>;