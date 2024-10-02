use clap::{Parser, Args, Subcommand};

use crate::common::*;
use crate::repositories::binance::ApiError;
use crate::repositories::binance::futures::orders::*;

#[derive(Parser)]
pub struct OrdersCommand {
  #[command(subcommand)]
  commands: Commands,
}

impl Default for OrdersCommand {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(Subcommand)]
enum Commands {
  /// orders submit
  Submit(SubmitArgs),
  /// orders sync
  Sync(SyncArgs),
}

#[derive(Args)]
struct SubmitArgs {
  /// symbol
  symbol: String,
  /// position side
  position_side: String,
  /// side
  side: String,
  /// price
  price: f64,
  /// quantity
  quantity: f64,
}

#[derive(Args)]
struct SyncArgs {
  /// symbol
  symbol: String,
  /// limit
  limit: i64,
}

impl OrdersCommand {
  pub fn new() -> Self {
    Self {
      ..Default::default()
    }
  }

  async fn submit(
    &self,
    ctx: Ctx,
    symbol: String,
    position_side: String,
    side: String,
    price: f64,
    quantity: f64,
  ) -> Result<(), Box<dyn std::error::Error>> {
    println!("orders submit");
    match OrdersRepository::submit(ctx, &symbol, &position_side, &side, price, quantity).await {
      Ok(order_id) => {
        println!("orders submit success {order_id:}");
      },
      Err(err) => {
        if err.is::<ApiError>() {
          println!("error {:?}", err);
        } else {
          println!("error {:?}", err);
        }
      },
    }

    Ok(())
  }

  async fn sync(
    &self,
    ctx: Ctx,
    symbol: String,
    limit: i64,
  ) -> Result<(), Box<dyn std::error::Error>> {
    println!("orders sync");
    let values = OrdersRepository::sync(ctx.clone(), &symbol, 0, limit).await;
    println!("orders sync {:?}", values);
    Ok(())
  }

  pub async fn run(&self, ctx: Ctx) -> Result<(), Box<dyn std::error::Error>> {
    match &self.commands {
      Commands::Submit(args) => self.submit(
        ctx.clone(),
        args.symbol.clone(),
        args.position_side.clone(),
        args.side.clone(),
        args.price.clone(),
        args.quantity.clone(),
      ).await,
      Commands::Sync(args) => self.sync(ctx.clone(), args.symbol.clone(), args.limit.clone()).await,
    }
  }
}
