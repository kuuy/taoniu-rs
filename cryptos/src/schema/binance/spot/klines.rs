diesel::table! {
  #[sql_name = "binance_spot_klines"]
  klines (id) {
    id -> Varchar,
    symbol -> Varchar,
    interval -> Varchar,
    open -> Double,
    close -> Double,
    high -> Double,
    low -> Double,
    volume -> Double,
    quota -> Double,
    timestamp -> BigInt,
    created_at -> Timestamptz,
    updated_at -> Timestamptz,
  }
}