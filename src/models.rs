use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug)]
pub struct Trade {
    pub exchange: String,
    pub symbol: String,
    pub event_time: DateTime<Utc>,
    pub price: Decimal,
    pub quantity: Decimal,
    pub side: Option<String>,
}
