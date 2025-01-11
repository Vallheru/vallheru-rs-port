use super::{ApiMethod, ApiRequest};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginResponse {
    pub token: String,
    pub id: i32,
    pub username: String,
    pub login_count: i32,
}

impl ApiRequest for LoginRequest {
    fn endpoint(&self) -> String {
        String::from("/api/login")
    }
    fn method(&self) -> ApiMethod {
        ApiMethod::Post
    }
}
