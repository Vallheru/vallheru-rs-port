use crate::web::middleware::AuthError;
use axum::{http::StatusCode, response::IntoResponse};
use serde_json::json;

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
}

impl Error {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::Auth(_) => StatusCode::UNAUTHORIZED,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Self::InternalServer(_) | Self::AnyHow(_) | Self::Sqlx(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }

    pub fn ty(&self) -> String {
        String::from(match self {
            Self::Forbidden => "forbidden",
            Self::Auth(_) => "auth error",
            Self::NotFound => "not found",
            Self::Unauthorized(_) => "unauthorized",
            Self::InternalServer(_) | Self::AnyHow(_) | Self::Sqlx(_) => "internal server error",
        })
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let error = match self {
            Self::Forbidden => vallheru::api::ErrorResponse {
                ty: self.ty(),
                details: self.to_string(),
            },
            Self::Unauthorized(ref details) => vallheru::api::ErrorResponse {
                ty: self.ty(),
                details: details.clone().unwrap_or(self.to_string()),
            },
            Self::Auth(ref e) => vallheru::api::ErrorResponse {
                ty: self.ty(),
                details: e.to_string(),
            },
            Self::NotFound => vallheru::api::ErrorResponse {
                ty: self.ty(),
                details: self.to_string(),
            },
            Self::Sqlx(ref e) => {
                println!("SQLx error: {:?}", e);

                vallheru::api::ErrorResponse {
                    ty: self.ty(),
                    details: self.to_string(),
                }
            }
            Self::AnyHow(ref e) => {
                println!("Generic error: {:?}", e);

                vallheru::api::ErrorResponse {
                    ty: self.ty(),
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

                vallheru::api::ErrorResponse {
                    ty: self.ty(),
                    details,
                }
            }
        };

        (self.status_code(), json!(error).to_string()).into_response()
    }
}
