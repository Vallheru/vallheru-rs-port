use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
    #[serde(rename = "type")]
    pub ty: String,
    pub details: String,
}
