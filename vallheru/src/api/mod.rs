pub mod login;
pub use login::LoginRequest;
pub use login::LoginResponse;

pub mod player;
pub use player::PlayerRequest;
pub use player::PlayerResponse;

pub mod token;
pub use token::IsTokenValidRequest;
pub use token::IsTokenValidResponse;

pub mod error;
pub use error::ErrorResponse;
pub use error::ErrorResponseKind;
use reqwest::Url;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub static NoToken: Option<&'static str> = None;

pub enum ApiMethod {
    Get,
    Post,
}

pub trait ApiRequest {
    fn endpoint(&self) -> String;
    fn method(&self) -> ApiMethod;
}

pub trait Stringify<'a> {
    fn from_str(path_str: &str) -> Self;
    fn to_str(&self) -> &'a str;
}

// this error should not be returned from the API endpoint. This error is returned when the api is called
#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error("an api http call failed")]
    Reqwest(#[from] reqwest::Error),

    #[error("invalid url passed to the api request")]
    UrlParser(#[from] url::ParseError),

    #[error("invalid response received from the API request")]
    InvalidResponse(#[from] serde_json::Error),

    #[error("invalid request parameter received for extractor")]
    InvalidExtractorParameter(String),

    #[error("handled error received from api")]
    Api(ErrorResponse),
}

impl From<ErrorResponse> for ApiError {
    fn from(value: ErrorResponse) -> Self {
        Self::Api(value)
    }
}

pub type Result<T> = std::result::Result<T, ApiError>;

pub fn handle_api_error(
    err: &ApiError,
    error_response_handler: impl FnOnce(&ErrorResponse),
    unexpected_error_handler: impl FnOnce(&ApiError),
) {
    match err {
        ApiError::Api(error_response) => error_response_handler(error_response),
        _ => unexpected_error_handler(err),
    }
}

fn default_client() -> reqwest::Client {
    reqwest::Client::new()
}

pub async fn api_request<Q, R>(
    client: Option<&reqwest::Client>,
    base_url: &str,
    req: &Q,
    token: Option<impl Into<String>>,
) -> Result<R>
where
    Q: ApiRequest + Serialize,
    R: DeserializeOwned,
{
    let client = match client {
        Some(client) => client,
        None => &default_client(),
    };

    let endpoint = Url::parse(base_url)?.join(&req.endpoint())?;

    let mut request = match req.method() {
        ApiMethod::Get => client.get(endpoint),
        ApiMethod::Post => client.post(endpoint).json(req),
    };

    if let Some(token) = token {
        request = request.header("Authorization", format!("Token {}", token.into()));
    }

    let response = request.send().await?;

    let text = response.text().await?;

    let res: serde_json::Result<R> = serde_json::from_str(&text);
    match res {
        Ok(res) => Ok(res),
        Err(err) => {
            // Handled error returned?
            let err_res: serde_json::Result<ErrorResponse> = serde_json::from_str(&text);
            match err_res {
                Ok(error_response) => Err(error_response)?,
                Err(_) => Err(err)?,
            }
        }
    }
}
