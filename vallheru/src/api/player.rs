use super::{ApiMethod, ApiRequest, Stringify};
use serde::{Deserialize, Serialize};

#[derive(Serialize, PartialEq)]
pub enum PlayerIdentifier {
    AuthToken,
}

impl<'a> Stringify<'a> for PlayerIdentifier {
    fn from_str(path_str: &str) -> Self {
        match path_str {
            "AuthToken" => Self::AuthToken,
            _ => Self::AuthToken, // may be changed in the future, for now we do not have any other type
        }
    }
    fn to_str(&self) -> &'a str {
        match self {
            Self::AuthToken => "AuthToken",
        }
    }
}

#[derive(Serialize)]
pub struct PlayerRequest {
    pub identifier: PlayerIdentifier,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerResponse {
    pub player: crate::model::Player,
}

impl ApiRequest for PlayerRequest {
    fn endpoint(&self) -> String {
        format!("/api/game/player/{}", self.identifier.to_str())
    }

    fn method(&self) -> ApiMethod {
        ApiMethod::Get
    }
}
