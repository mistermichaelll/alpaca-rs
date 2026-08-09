use alpaca_rs::trading::orders::Order;
use alpaca_rs::{Alpaca, CryptoOrder, PositionIntent, TimeInForce};
use dotenvy;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;

    let alpaca = Alpaca::http_client(
        std::env::var("ALPACA_PAPER_API_KEY")?,
        std::env::var("ALPACA_PAPER_SECRET_KEY")?,
    );

    let order = Order::builder()
        .symbol("ETH/USD")
        .time_in_force(TimeInForce::GoodTilCancelled)
        .order_type(CryptoOrder::Market)
        .quantity(3)
        .position_intent(PositionIntent::BuyToOpen)
        .build()
        .expect("Failed to build the order.");

    let r = alpaca.send_order(order).await?;

    println!("{}", serde_json::to_string(&r)?);

    Ok(())
}
