use postgres::{Client, NoTls, Error};
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};
use futures_util::StreamExt;
use anyhow::Result;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use rustls::crypto::ring;

/*
#[derive(Debug)]
struct Trade {
    id: i64,
    exchange: String,
    symbol: String,
    event_time: DateTime<Utc>,
    price: Decimal,
    quantity: Decimal,
    side: Option<String>,
}
*/


#[tokio::main]
async fn main() -> Result<()> {
    ring::default_provider()
        .install_default()
        .expect("failed to install rustls crypto provider");

    let url = "wss://fstream.binance.com/market/ws/btcusdt@aggTrade";

    // WebSocket connection
    let (websocket, _) = connect_async(url).await.expect("Failed to connect");

    println!("WebSocket connected");

    let (_, mut reader) = websocket.split();

    while let Some(msg) = reader.next().await {
        match msg {
            Ok(msg) => {
                if let Ok(text) = msg.to_text() {
                    println!("{}", text);
                }
            }
            Err(e) => {
                eprintln!("Error occured trade WebSocket: {}", e);
                break;
            }
        }
    }



    Ok(())
}
