pub use crate::web::{AppState, Result};
pub use axum::{
    extract::Extension,
    extract::{Path, State},
    response::Html,
    Form, Json,
};
pub use minijinja::context;
