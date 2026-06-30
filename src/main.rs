mod models;
mod binance;
mod db;

use anyhow::Result;
use tokio::sync::mpsc;

use crate::models::Trade;
use crate::binance::run_binance_trade;
use crate::db::{insert_db, connect_db};



#[tokio::main]
async fn main() -> Result<()> {
    let mut client = connect_db().await?;
    let (tx, mut rx) = mpsc::channel(10_000);


    tokio::spawn(async move {
        if let Err(e) = run_binance_trade("BTCUSDT", tx).await {
            eprintln!("Error running binance trade: {}", e);
        }
    });

    let mut buffer = Vec::with_capacity(10_000);
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));

    loop {
        tokio::select! {
            Some(trade) = rx.recv() => {
                buffer.push(trade);

                if buffer.len() >= 100 {
                    insert_db(&mut client, &mut buffer).await?;
                    buffer.clear();
                }
            }
            _ = interval.tick() => {
                    if !buffer.is_empty() {
                        insert_db(&mut client, &mut buffer).await?;
                        buffer.clear();
                    }
                }
        }
    }
}







