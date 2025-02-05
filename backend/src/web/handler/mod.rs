pub mod prelude;

pub mod new_home;
pub mod home;
pub mod player;
pub mod player_statistics;

pub use crate::web::Error;

pub async fn not_found_fallback() -> Error {
    Error::NotFound
}

pub async fn method_not_allowed_fallback() -> Error {
    Error::Forbidden
}
