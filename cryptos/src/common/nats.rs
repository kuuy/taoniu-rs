use async_nats::{Client, ConnectError};

use crate::Env;

pub struct Nats {}

impl Nats {
  pub async fn new() -> Result<Client, ConnectError> {
    let dsn = Env::var("NATS_DSN");
    let token = Env::var("NATS_TOKEN");
    let conn = async_nats::connect_with_options(
      dsn,
      async_nats::ConnectOptions::with_token(token)
    ).await?;
    Ok(conn)
  }
}
