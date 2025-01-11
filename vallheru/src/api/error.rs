use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub enum ErrorResponseKind {
    APIError,
    RequestForbidden,
    AuthError,
    NotFound,
    Unauthorized,
    InternalServerError,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ErrorResponse {
    #[serde(rename = "type")]
    pub ty: ErrorResponseKind,
    pub details: String,
}
