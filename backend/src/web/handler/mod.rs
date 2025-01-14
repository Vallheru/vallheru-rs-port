pub mod prelude;

pub mod home;
pub mod player;

pub use crate::web::Error;

pub async fn not_found_fallback() -> Error {
    Error::NotFound
}

pub async fn method_not_allowed_fallback() -> Error {
    Error::Forbidden
}
