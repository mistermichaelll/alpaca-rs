use crate::Alpaca;
use crate::core::client::BASE_URL_PAPER_API;
use crate::trading::trading_core::*;
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Order<T> {
    symbol: String,
    time_in_force: TimeInForce,
    #[serde(rename = "type")]
    order_type: T,
    qty: Quantity,
    #[serde(with = "rust_decimal::serde::str_option")]
    limit_price: Option<rust_decimal::Decimal>,
    position_intent: PositionIntent,
}

pub type StockOrderType = Order<StockOrder>;
pub type CryptoOrderType = Order<CryptoOrder>;

pub struct OrderBuilder<T> {
    symbol: Option<String>,
    time_in_force: Option<TimeInForce>,
    order_type: Option<T>,
    qty: Option<Quantity>,
    limit_price: Option<Option<Decimal>>,
    position_intent: Option<PositionIntent>,
}

impl<T> OrderBuilder<T> {
    pub fn new() -> Self {
        Self {
            symbol: None,
            time_in_force: None,
            order_type: None,
            qty: None,
            limit_price: None,
            position_intent: None,
        }
    }

    pub fn symbol(mut self, symbol: &str) -> Self {
        self.symbol = Some(symbol.to_string());
        self
    }

    pub fn time_in_force(mut self, tif: TimeInForce) -> Self {
        self.time_in_force = Some(tif);
        self
    }

    pub fn order_type(mut self, ot: T) -> Self {
        self.order_type = Some(ot);
        self
    }

    pub fn quantity<Q: Into<Quantity>>(mut self, qty: Q) -> Self {
        self.qty = Some(qty.into());
        self
    }

    pub fn limit_price(mut self, price: Option<Decimal>) -> Self {
        self.limit_price = Some(price);
        self
    }

    pub fn position_intent(mut self, intent: PositionIntent) -> Self {
        self.position_intent = Some(intent);
        self
    }

    pub fn build(self) -> Result<Order<T>, String> {
        Ok(Order {
            symbol: self.symbol.ok_or("Symbol is required")?,
            time_in_force: self.time_in_force.ok_or("Time in force is required")?,
            order_type: self.order_type.ok_or("Order type is required")?,
            qty: self.qty.ok_or("Quantity is required")?,
            limit_price: self.limit_price.unwrap_or(None),
            position_intent: self.position_intent.ok_or("Position intent is required")?,
        })
    }

    pub fn builder() -> OrderBuilder<T> {
        OrderBuilder::new()
    }
}

impl<T> Order<T> {
    pub fn builder() -> OrderBuilder<T> {
        OrderBuilder::new()
    }
}

impl Alpaca {
    //
    // Sends an order to the trading API.
    //
    pub async fn send_order<T: Serialize>(
        &self,
        o: Order<T>,
    ) -> Result<ExecutionReport, Box<dyn Error>> {
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
