use clap::{Parser, Subcommand};

mod config;
mod context;
mod common;
mod commands;

use config::binance::spot::config as Config;
use commands::*;
use common::*;

fn main() {
  Env::load();

  let vars: Vec<String> = Env::vars("ASYNQ_BINANCE_SPOT_QUEUE".to_string());
  for var in &vars {
    println!("redis queue {}", var);
  }
  println!("redis address {}", Env::var(format!("REDIS_{:02}_ADDRESS", 1)));
  println!("redis db {}", Env::int(format!("REDIS_{:02}_DB", 1)));
  println!("dydx position id {}", Env::int64("DYDX_POSITION_ID".to_string()));
  //println!("current dir {}", env::args().nth(0).unwrap());
  //println!("current dir {}", env::current_dir().unwrap().join(".env").display());
  //if err := godotenv.Load(path.Join(filepath.Dir(os.Args[0]), ".env")); err != nil {
  //  let env_path = env::current_dir().unwrap().join(".env");
  //  dotenv().ok();
  //}
  //dotenv::from_path(my_path.as_path())

  println!("{}", format!("REDIS_{:02}_ADDRESS", 1));
  //println!("{}", env::var("REDIS_01_ADDRESS").unwrap());
  Cli::parse();
}

#[derive(Parser)]
struct Cli {
  #[command(subcommand)]
  commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
  Binance(BinanceCommands),
}
