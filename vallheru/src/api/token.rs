use super::{ApiMethod, ApiRequest};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct IsTokenValidRequest {
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct IsTokenValidResponse {
    pub is_valid: bool,
    pub reason: String,
}

impl ApiRequest for IsTokenValidRequest {
    fn endpoint(&self) -> String {
        String::from("/api/is-token-valid")
    }
    fn method(&self) -> ApiMethod {
        ApiMethod::Post
    }
}

impl IsTokenValidResponse {
    pub fn default_invalid_empty_token() -> Self {
        Self {
            is_valid: false,
            reason: String::from("Token is empty"),
        }
    }

    pub fn default_invalid_internal_error() -> Self {
        Self {
            is_valid: false,
            reason: String::from("internal server error"),
        }
    }
}
