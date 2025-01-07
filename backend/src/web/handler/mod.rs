pub mod home;

pub use crate::web::Error;

pub async fn not_found_fallback() -> Error {
    Error::NotFound
}
