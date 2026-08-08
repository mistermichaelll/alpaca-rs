use alpaca_rs::Alpaca;
use alpaca_rs::{CryptoCurrency, Currency, TimeFrame};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;

    let alpaca = Alpaca::http_client(
        std::env::var("ALPACA_API_KEY")?,
        std::env::var("ALPACA_SECRET_KEY")?,
    );

    let body = alpaca
        .get_historical_bars(
            TimeFrame::Hour,
            5,
            vec![(CryptoCurrency::BTC, Currency::USD)],
        )
        .await?;

    println!("{}", serde_json::to_string(&body)?);
    Ok(())
}
