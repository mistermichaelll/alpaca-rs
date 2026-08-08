use crate::Alpaca;
use crate::core::client::BASE_URL_CRYPTO;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum CryptoCurrency {
    // there are a lot more than this, but i'm not typing them all out rn.
    // there's an argument to be made to just accept a string input rather than enumerating these,
    // but unlike the stocks there's only a limited number of cryptocurrencies on the site.
    BTC,
    ETH,
    SHIB,
    SOL,
    USDC,
    USDT,
}

impl CryptoCurrency {
    fn as_str(&self) -> &'static str {
        match self {
            CryptoCurrency::BTC => "BTC",
            CryptoCurrency::ETH => "ETH",
            CryptoCurrency::SHIB => "SHIB",
            CryptoCurrency::SOL => "SOL",
            CryptoCurrency::USDC => "USDC",
            CryptoCurrency::USDT => "USDT",
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Currency {
    USD,
    USDC,
    USDT,
    BTC,
}

impl Currency {
    fn as_str(&self) -> &'static str {
        match self {
            Currency::USD => "USD",
            Currency::USDC => "USDC",
            Currency::USDT => "USDT",
            Currency::BTC => "BTC",
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum TimeFrame {
    Min,
    Hour,
    Day,
    Week,
    Month,
}

impl TimeFrame {
    fn as_str(&self) -> &'static str {
        match self {
            TimeFrame::Min => "Min",
            TimeFrame::Hour => "Hour",
            TimeFrame::Day => "Day",
            TimeFrame::Week => "Week",
            TimeFrame::Month => "Month",
        }
    }
}

fn set_timeframe(n: i32, timeframe: TimeFrame) -> String {
    return format!("{}{}", n, timeframe.as_str());
}

fn set_currency_pair(crypto: CryptoCurrency, currency: Currency) -> String {
    return format!("{}/{}", crypto.as_str(), currency.as_str());
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
struct Bar {
    c: f64,
    h: f64,
    l: f64,
    n: i32,
    o: f64,
    t: jiff::Timestamp,
    v: f64,
    vw: f64,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct HistoricalBars {
    bars: HashMap<String, Vec<Bar>>,
    next_page_token: Option<String>,
}

impl Alpaca {
    pub async fn get_historical_bars(
        &self,
        time_grain: TimeFrame,
        time_length: i32,
        currency_pairs: Vec<(CryptoCurrency, Currency)>,
    ) -> Result<HistoricalBars, Box<dyn Error>> {
        let query_timeframe = set_timeframe(time_length, time_grain);

        let symbols: String = currency_pairs
            .iter()
            .map(|x| set_currency_pair(x.0, x.1))
            .collect::<Vec<String>>()
            .join(",");

        let query = vec![("symbols", symbols), ("timeframe", query_timeframe)];

        let res = self
            .request(BASE_URL_CRYPTO, Method::GET, "bars")
            .query(&query)
            .send()
            .await?;

        let body = res.json().await?;
        Ok(body)
    }
}
