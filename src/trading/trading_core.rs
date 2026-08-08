use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum PositionIntent {
    BuyToOpen,
    BuyToClose,
    SellToOpen,
    SellToClose,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum StockOrder {
    #[serde(rename = "market")]
    Market,
    #[serde(rename = "limit")]
    Limit,
    #[serde(rename = "stop")]
    Stop,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum CryptoOrder {
    #[serde(rename = "market")]
    Market,
    #[serde(rename = "limit")]
    Limit,
    #[serde(rename = "stop_limit")]
    StopLimit,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TimeInForce {
    #[serde(rename = "d")]
    Day,
    #[serde(rename = "gtc")]
    GoodTilCancelled,
    #[serde(rename = "opg")]
    AtTheOpening,
    #[serde(rename = "cls")]
    AtTheClose,
    #[serde(rename = "ioc")]
    ImmediateOrCancel,
    #[serde(rename = "fok")]
    FillOrKill,
}

impl TimeInForce {
    pub fn as_str(&self) -> &'static str {
        match self {
            TimeInForce::Day => "day",
            TimeInForce::GoodTilCancelled => "gtc",
            TimeInForce::AtTheOpening => "opg",
            TimeInForce::AtTheClose => "cls",
            TimeInForce::ImmediateOrCancel => "ioc",
            TimeInForce::FillOrKill => "fok",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Leg {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Legs {
    legs: Vec<Leg>,
}
