use crate::Alpaca;
use crate::core::client::BASE_URL_PAPER_API;
use crate::trading::trading_core::*;
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Order {
    symbol: String,
    time_in_force: TimeInForce,
    #[serde(rename = "type")]
    order_type: String,
    qty: u32,
    #[serde(with = "rust_decimal::serde::str_option")]
    limit_price: Option<rust_decimal::Decimal>,
    position_intent: PositionIntent,
}

impl Order {
    pub fn new(
        symbol: String,
        time_in_force: TimeInForce,
        order_type: String,
        quantity: u32,
        limit_price: Option<Decimal>,
        position_intent: PositionIntent,
    ) -> Order {
        Self {
            symbol: symbol,
            time_in_force: time_in_force,
            order_type: order_type,
            qty: quantity,
            limit_price: limit_price,
            position_intent: position_intent,
        }
    }
}

impl Alpaca {
    //
    // Sends an order to the trading API.
    //
    pub async fn send_order(&self, o: Order) -> Result<ExecutionReport, Box<dyn Error>> {
        let res = self
            .request(BASE_URL_PAPER_API, reqwest::Method::POST, "orders")
            .header("accept", "application/json")
            .json(&o)
            .send()
            .await?;

        let status = &res.status();
        if !status.is_success() {
            let error_text = res.text().await?;
            return Err(format!("Alpaca API Error ({}): {}", status, error_text).into());
        }

        let body: ExecutionReport = res.json().await?;

        Ok(body)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExecutionReport {
    id: String,
    client_order_id: String,
    created_at: jiff::Timestamp,
    updated_at: jiff::Timestamp,
    submitted_at: jiff::Timestamp,
    filled_at: Option<jiff::Timestamp>,
    expired_at: Option<jiff::Timestamp>,
    canceled_at: Option<jiff::Timestamp>,
    failed_at: Option<jiff::Timestamp>,
    replaced_at: Option<jiff::Timestamp>,
    replaced_by: Option<String>,
    replaces: Option<String>,
    asset_id: String,
    symbol: String,
    asset_class: String,
    notional: Option<String>,
    qty: String,
    filled_qty: String,
    filled_avg_price: Option<String>,
    order_class: String,
    order_type: String,
    #[serde(rename = "type")]
    _type: String,
    side: String,
    position_intent: String,
    time_in_force: String,
    limit_price: Option<String>,
    stop_price: Option<String>,
    status: String,
    extended_hours: bool,
    legs: Option<Legs>,
    trail_percent: Option<String>,
    trail_price: Option<String>,
    hwm: Option<String>,
    subtag: Option<String>,
    source: Option<String>,
    expires_at: jiff::Timestamp,
}
