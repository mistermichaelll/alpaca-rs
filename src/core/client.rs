use reqwest::Url;

pub const BASE_URL_PAPER_API: &str = "https://paper-api.alpaca.markets/v2/";
pub const BASE_URL_CRYPTO_SANDBOX: &str = "https://data.sandbox.alpaca.markets/v1beta3/crypto/us/"; // default to us for dev
pub const BASE_URL_CRYPTO: &str = "https://data.alpaca.markets/v1beta3/crypto/us/"; // default to us for dev

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

    pub fn request(
        &self,
        base_url: &str,
        method: reqwest::Method,
        path: &str,
    ) -> reqwest::RequestBuilder {
        let base = Url::parse(base_url).expect("Failed to parse URL.");

        let url = base.join(path).expect("Couldn't join URL.");

        self.client
            .request(method, url)
            .header("APCA-API-KEY-ID", &self.api_token)
            .header("APCA-API-SECRET-KEY", &self.secret_token)
    }
}
