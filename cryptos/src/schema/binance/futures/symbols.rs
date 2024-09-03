diesel::table! {
  #[sql_name = "binance_futures_symbols"]
  symbols (id) {
    id -> Varchar,
    symbol -> Varchar,
    base_asset -> Varchar,
    quote_asset -> Varchar,
    filters -> Jsonb,
    depth -> Jsonb,
    status -> Integer,
    created_at -> Timestamptz,
    updated_at -> Timestamptz,
  }
}