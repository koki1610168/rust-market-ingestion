use postgres::{Client, NoTls, Error};
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};
use futures_util::StreamExt;
use anyhow::Result;
use serde_json;
use serde::Deserialize;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use rustls::crypto::ring;
use std::str::FromStr;

use tokio::sync::mpsc::Sender;
use crate::models::Trade;


#[derive(Debug, Deserialize)]
struct BinanceAggrTrade {
    #[serde(rename = "e")]
    event_type: String,

    #[serde(rename = "E")]
    event_time: i64,

    #[serde(rename = "s")]
    symbol: String,

    #[serde(rename = "a")]
    aggr_trade_id: u64,

    #[serde(rename = "p")]
    price: String,

    #[serde(rename = "q")]
    qty: String,

    // quantity without RPI orders
    #[serde(rename = "nq")]
    normal_qty: String,

    #[serde(rename = "f")]
    first_trade_id: u64,

    #[serde(rename = "l")]
    last_trade_id: u64,

    #[serde(rename = "T")]
    trade_time: u64,

    #[serde(rename = "m")]
    is_mm: bool,
}

impl BinanceAggrTrade {
    fn into_trade(self) -> Result<Trade> {
        let event_time = DateTime::from_timestamp_millis(self.event_time)
                            .expect("invalid Binance timestamp");
        let side = if self.is_mm {
            "Sell"
        } else {
            "Buy"
        };
        let price = Decimal::from_str(self.price.as_str())?;
        let quantity = Decimal::from_str(self.qty.as_str())?;

        let trade: Trade = Trade {
            exchange: "binance".to_string(),
            symbol: self.symbol,
            event_time: event_time,
            price: price,
            quantity: quantity,
            side: Some(side.to_string()),
        };

        Ok(trade)
    }
}


pub async fn run_binance_trade(symbol: &str, tx: Sender<Trade>) -> Result<()> {
    ring::default_provider()
        .install_default()
        .expect("failed to install rustls crypto provider");

    let url = format!("wss://fstream.binance.com/market/ws/{}@aggTrade", symbol.to_lowercase());

    // WebSocket connection
    let (websocket, _) = connect_async(url).await.expect("Failed to connect");

    println!("WebSocket connected");

    let (_, mut reader) = websocket.split();

    while let Some(msg) = reader.next().await {
        let msg = msg?;

        if !msg.is_text() {
            continue;
        }

        let text = msg.to_text()?;
        let binanceAggrTrade: BinanceAggrTrade = serde_json::from_str(text)?;
        let trade = binanceAggrTrade.into_trade()?;

        tx.send(trade).await?;
    }

    Ok(())
}
