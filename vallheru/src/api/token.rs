use super::{ApiMethod, ApiRequest};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct IsTokenValidRequest {
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
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
