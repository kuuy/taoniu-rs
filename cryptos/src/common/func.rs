use std::future::Future;
use std::pin::Pin;

use crate::common::Ctx;

pub type EventFn = Box<dyn Fn(Ctx, String) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error>>>>>>;