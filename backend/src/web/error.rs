use crate::web::middleware::AuthError;
use axum::{http::StatusCode, response::IntoResponse};
use serde_json::json;
use vallheru::api::{self, ErrorResponseKind};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("authentication required")]
    Unauthorized(Option<String>),

    #[error("user may not perform that action")]
    Forbidden,

    #[error("an authentication failed")]
    Auth(#[from] AuthError),

    #[error("request path not found")]
    NotFound,

    #[error("an internal server error occurred")]
    AnyHow(#[from] anyhow::Error),

    #[error("an database error occurred")]
    Sqlx(#[from] sqlx::Error),

    #[error("an internal server error occurred")]
    InternalServer(String),

    #[error("a handled error from the api")]
    Api((StatusCode, String)),
}

impl Error {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::Api((code, _)) => code.clone(),
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::Auth(_) => StatusCode::UNAUTHORIZED,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Self::InternalServer(_) | Self::AnyHow(_) | Self::Sqlx(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let error = match self {
            Self::Forbidden => api::ErrorResponse {
                ty: ErrorResponseKind::RequestForbidden,
                details: self.to_string(),
            },
            Self::Unauthorized(ref details) => api::ErrorResponse {
                ty: ErrorResponseKind::Unauthorized,
                details: details.clone().unwrap_or(self.to_string()),
            },
            Self::Auth(ref e) => api::ErrorResponse {
                ty: ErrorResponseKind::AuthError,
                details: e.to_string(),
            },
            Self::NotFound => api::ErrorResponse {
                ty: ErrorResponseKind::NotFound,
                details: self.to_string(),
            },
            Self::Sqlx(ref e) => {
                println!("SQLx error: {:?}", e);

                api::ErrorResponse {
                    ty: ErrorResponseKind::InternalServerError,
                    details: self.to_string(),
                }
            }
            Self::AnyHow(ref e) => {
                println!("Generic error: {:?}", e);

                api::ErrorResponse {
                    ty: ErrorResponseKind::InternalServerError,
                    details: self.to_string(),
                }
            }
            Self::InternalServer(ref details) => {
                let details = if details.is_empty() {
                    self.to_string()
                } else {
                    details.clone()
                };

                println!("Internal error: {:?}", details);

                api::ErrorResponse {
                    ty: ErrorResponseKind::InternalServerError,
                    details,
                }
            }
            Self::Api((_, ref details)) => api::ErrorResponse {
                ty: ErrorResponseKind::APIError,
                details: details.clone(),
            },
        };

        (self.status_code(), json!(error).to_string()).into_response()
    }
}
