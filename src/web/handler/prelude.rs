pub use crate::web::{AppState, Result};
pub use axum::{
    extract::{Extension, Query},
    extract::{Path, State},
    response::Html,
    Form, Json,
};
pub use log::error;
pub use minijinja::context;
pub use serde::{Deserialize, Serialize};
