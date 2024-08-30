diesel::table! {
  #[sql_name = "binance_margin_cross_positions"]
  scalping (id) {
    id -> Varchar,
    symbol -> Varchar,
    side -> Integer,
    leverage -> Integer,
    capital -> Double,
    notional -> Double,
    entry_price -> Double,
    entry_quantity -> Double,
    entry_amount -> Double,
    timestamp -> BigInt,
    status -> Integer,
    version -> BigInt,
    created_at -> Timestamptz,
    updated_at -> Timestamptz,
  }
}