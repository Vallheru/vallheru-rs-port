use std::sync::Arc;

use vallheru::{
    api::{player::PlayerIdentifier, PlayerResponse, Stringify},
    model::Player,
};

use crate::web::handler::prelude::*;

pub async fn get_player(
    Extension(this_player): Extension<Arc<Player>>,
    Path(identifier): Path<String>,
) -> Result<Json<PlayerResponse>> {
    let identifier = PlayerIdentifier::from_str(&identifier);

    match identifier {
        // Info about player with authentication token must be provided via authentication middleware.
        // We should expect it exists
        PlayerIdentifier::AuthToken => Ok(Json(PlayerResponse {
            player: Arc::unwrap_or_clone(this_player),
        })),
    }
}
