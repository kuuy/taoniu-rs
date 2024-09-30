diesel::table! {
  #[sql_name = "binance_futures_scalping_plans"]
  plans (plan_id) {
    plan_id -> Varchar,
    status -> Integer,
  }
}