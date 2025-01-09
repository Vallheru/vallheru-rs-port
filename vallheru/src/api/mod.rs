pub mod player;
pub use player::LoginRequest;
pub use player::LoginResponse;

pub mod error;
pub use error::ErrorResponse;
use reqwest::Url;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub enum ApiMethod {
    Get,
    Post,
}

trait ApiRequest {
    fn endpoint(&self) -> &'static str;
    fn method(&self) -> ApiMethod;
}

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error("an api http call failed")]
    Reqwest(#[from] reqwest::Error),

    #[error("invalid url passed to the api request")]
    UrlParser(#[from] url::ParseError),

    #[error("invalid response received from the API request")]
    InvalidResponse(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, ApiError>;

fn default_client() -> reqwest::Client {
    reqwest::Client::new()
}

pub async fn api_request<Q, R>(
    client: Option<reqwest::Client>,
    base_url: &str,
    req: &Q,
) -> Result<R>
where
    Q: ApiRequest + Serialize,
    R: DeserializeOwned,
{
    let client = client.unwrap_or(default_client());
    let endpoint = Url::parse(base_url)?.join(req.endpoint())?;

    let response = match req.method() {
        ApiMethod::Get => client.get(endpoint),
        ApiMethod::Post => client.post(endpoint).json(req),
    }
    .send()
    .await?;

    let text = response.text().await?;

    let res: R = serde_json::from_str(&text)?;

    Ok(res)
}
