diesel::table! {
  #[sql_name = "binance_futures_positions"]
  positions (id) {
    id -> Varchar,
    symbol -> Varchar,
    side -> Integer,
    leverage -> Integer,
    capital -> Double,
    notional -> Double,
    entry_price -> Double,
    entry_quantity -> Double,
    timestamp -> BigInt,
    status -> Integer,
    version -> BigInt,
    created_at -> Timestamptz,
    updated_at -> Timestamptz,
  }
}