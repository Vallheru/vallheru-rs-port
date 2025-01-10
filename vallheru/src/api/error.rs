use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ErrorResponse {
    #[serde(rename = "type")]
    pub ty: String,
    pub details: String,
}
