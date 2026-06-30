use tokio_postgres::{Client, NoTls, Error};
use anyhow::{Result, Context};
use crate::models::Trade;

pub async fn connect_db() -> Result<Client> {
    let (client, connection) = tokio_postgres::connect("postgres://postgres:postgres@localhost:4545/market", NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(client)
}

pub async fn insert_db(client: &mut Client, trades: &mut Vec<Trade>) -> Result<()> {
    if trades.is_empty() {
        return Ok(());
    }

    let stmt = client.prepare(
        "INSERT INTO trades (exchange, symbol, event_time, price, quantity, side)
        VALUES ($1, $2, $3, $4, $5, $6)",
    ).await?;
    println!("called");

    let transaction = client.transaction().await?;

    println!("insert initiated");
    for trade in trades {
        println!("{:?}", trade);
        transaction.execute(
            &stmt,
            &[&trade.exchange, &trade.symbol, &trade.event_time, 
                &trade.price, &trade.quantity, &trade.side]
        ).await?;
    }

    transaction.commit().await?;

    Ok(())
}
