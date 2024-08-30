diesel::table! {
  #[sql_name = "binance_spot_scalping"]
  scalping (id) {
    id -> Varchar,
    symbol -> Varchar,
    capital -> Double,
    price -> Double,
    take_price -> Double,
    stop_price -> Double,
    take_order_id -> BigInt,
    stop_order_id -> BigInt,
    profit -> Double,
    timestamp -> BigInt,
    status -> Integer,
    version -> BigInt,
    remark -> Varchar,
    expired_at -> Timestamptz,
    created_at -> Timestamptz,
    updated_at -> Timestamptz,
  }
}