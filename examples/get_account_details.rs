use alpaca_rs::Alpaca;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;

    let alpaca = Alpaca::http_client(
        std::env::var("ALPACA_PAPER_API_KEY")?,
        std::env::var("ALPACA_PAPER_SECRET_KEY")?,
    );

    let account_details = alpaca.get_account().await?;

    println!("{}", serde_json::to_string(&account_details)?);
    Ok(())
}
