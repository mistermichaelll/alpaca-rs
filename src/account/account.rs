use crate::core::client::Alpaca;
use crate::core::client::BASE_URL_PAPER_API;
use jiff::civil::Date;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error};

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Account {
    id: String,
    admin_configurations: HashMap<String, String>,
    user_configurations: Option<HashMap<String, String>>,
    account_number: String,
    status: String,
    crypto_status: String,
    options_approved_level: i32,
    options_trading_level: i32,
    currency: String,
    buying_power: String,
    regt_buying_power: String,
    effective_buying_power: String,
    non_marginable_buying_power: String,
    options_buying_power: String,
    cash: String,
    accrued_fees: String,
    portfolio_value: String,
    trading_blocked: bool,
    transfers_blocked: bool,
    account_blocked: bool,
    created_at: jiff::Timestamp,
    trade_suspended_by_user: bool,
    multiplier: String,
    shorting_enabled: bool,
    equity: String,
    last_equity: String,
    long_market_value: String,
    short_market_value: String,
    position_market_value: String,
    initial_margin: String,
    maintenance_margin: String,
    last_maintenance_margin: String,
    sma: String,
    balance_asof: Date,
    crypto_tier: i32,
    intraday_adjustments: String,
    pending_reg_taf_fees: String,
}

impl Alpaca {
    ///
    /// Return your Alpaca account details.
    ///
    pub async fn get_account(&self) -> Result<Account, Box<dyn Error>> {
        let res = self
            .request(BASE_URL_PAPER_API, reqwest::Method::GET, "account")
            .send()
            .await?;

        let body: Account = res.json().await?;

        Ok(body)
    }
}
