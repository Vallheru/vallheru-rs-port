use std::sync::Arc;

use vallheru::model::Player;

use crate::web::handler::prelude::*;

pub async fn get_player(
    Extension(this_player): Extension<Arc<Player>>,
    Path(identifier): Path<String>,
) -> String {
    "test".into()
}

// pub async fn login()