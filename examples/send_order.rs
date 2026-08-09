use alpaca_rs::trading::orders;
use alpaca_rs::{Alpaca, CryptoOrder, PositionIntent, TimeInForce};
use dotenvy;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;

    let alpaca = Alpaca::http_client(
        std::env::var("ALPACA_PAPER_API_KEY")?,
        std::env::var("ALPACA_PAPER_SECRET_KEY")?,
    );

    let order = orders::Order::new(
        String::from("BTC/USD"),
        TimeInForce::GoodTilCancelled,
        CryptoOrder::Market,
        0.5,
        None,
        PositionIntent::BuyToOpen,
    );

    let r = alpaca.send_order(order).await?;

    println!("{}", serde_json::to_string(&r)?);

    Ok(())
}
