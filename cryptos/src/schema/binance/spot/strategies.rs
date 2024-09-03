diesel::table! {
  #[sql_name = "binance_spot_strategies"]
  strategies (id) {
    id -> Varchar,
    symbol -> Varchar,
    indicator -> Varchar,
    interval -> Varchar,
    price -> Double,
    signal -> Integer,
    timestamp -> BigInt,
    created_at -> Timestamptz,
    updated_at -> Timestamptz,
  }
}