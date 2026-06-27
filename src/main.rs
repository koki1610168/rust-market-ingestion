mod models;
mod binance;

use anyhow::Result;
use tokio::sync::mpsc;

use crate::models::Trade;
use crate::binance::run_binance_trade;



#[tokio::main]
async fn main() -> Result<()> {
    let (tx, mut rx) = mpsc::channel(10_000);

    tokio::spawn(async move {
        if let Err(e) = run_binance_trade("BTCUSDT", tx).await {
            eprintln!("Error running binance trade: {}", e);
        }
    });

    while let Some(i) = rx.recv().await {
        println!("{:?}", i);
    }
    Ok(())
}







