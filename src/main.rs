use postgres::{Client, NoTls, Error};
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};


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

fn main() -> Result<(), Error> {
    println!("Hello, world!");

    let mut client = Client::connect("postgres://postgres:postgres@localhost:4545/market", NoTls)?;

    client.execute("INSERT INTO trades (exchange, symbol, event_time, price, quantity, side)
        VALUES ($1, $2, $3, $4, $5, $6)", 
            &[
                &String::from("hyperliquid"), 
                &String::from("HYPEUSDT"), 
                &chrono::offset::Utc::now(), 
                &Decimal::new(635, 1), 
                &Decimal::new(1, 1), 
                &String::from("Buy"),
        ],
    )?;

    for row in client.query("SELECT id, exchange, symbol, event_time, price, quantity, side FROM trades", &[])? {
        let trade = Trade {
            id: row.get("id"),
            exchange: row.get("exchange"),
            symbol: row.get("symbol"),
            event_time: row.get("event_time"),
            price: row.get("price"),
            quantity: row.get("quantity"),
            side: row.get("side")
        };
        println!("{:?}", trade);

    }

    Ok(())
}
