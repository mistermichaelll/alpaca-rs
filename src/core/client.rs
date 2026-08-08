pub const BASE_URL_PAPER_MARKETS: &str = "https://paper-api.alpaca.markets/v2/";

#[derive(Clone, Debug)]
pub struct Alpaca {
    pub client: reqwest::Client,
    api_token: String,
    secret_token: String,
}

impl Alpaca {
    pub fn http_client(api_token: impl Into<String>, secret_token: impl Into<String>) -> Alpaca {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to build HTTP client.");

        Self {
            client,
            api_token: api_token.into(),
            secret_token: secret_token.into(),
        }
    }

    pub fn request(&self, method: reqwest::Method, path: &str) -> reqwest::RequestBuilder {
        let base = reqwest::Url::parse(BASE_URL_PAPER_MARKETS).expect("Failed to parse URL.");

        let url = base.join(path).expect("Couldn't parse URL???");

        self.client
            .request(method, url)
            .header("APCA-API-KEY-ID", &self.api_token)
            .header("APCA-API-SECRET-KEY", &self.secret_token)
    }
}
