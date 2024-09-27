diesel::table! {
  #[sql_name = "binance_spot_plans"]
  plans (id) {
    id -> Varchar,
    symbol -> Varchar,
    interval -> Varchar,
    side -> Integer,
    price -> Double,
    quantity -> Double,
    amount -> Double,
    timestamp -> BigInt,
    status -> Integer,
    remark -> Varchar,
    created_at -> Timestamptz,
    updated_at -> Timestamptz,
  }
}