use crate::web::Result;
use vallheru::model::Token;

pub async fn get_active_token_for_player(db: &sqlx::PgPool, player_id: i32) -> Option<String> {
    let token = sqlx::query_as::<_, Token>(
        r"SELECT * FROM token WHERE player_id=$1 AND valid_until>NOW()+'5 minutes'::INTERVAL",
    )
    .bind(player_id)
    .fetch_one(db)
    .await;

    match token {
        Ok(token) => Some(token.token),
        _ => None,
    }
}

pub async fn create_token_for_player(db: &sqlx::PgPool, player_id: i32) -> Option<String> {
    let token = Token::new(player_id);

    sqlx::query(
        r"INSERT INTO token(player_id, token, created_at, valid_until) VALUES($1, $2, $3, $4)",
    )
    .bind(player_id)
    .bind(&token.token)
    .bind(token.created_at)
    .bind(token.valid_until)
    .execute(db)
    .await
    .map(|_| Some(token.token))
    .unwrap_or(None)
}

pub async fn extend_token_for_player(db: &sqlx::PgPool, player_id: i32, token: &str) -> Result<()> {
    sqlx::query(
        r"UPDATE token SET valid_until=NOW()+'12 hours'::INTERVAL WHERE player_id=$1 AND token=$2",
    )
    .bind(player_id)
    .bind(token)
    .execute(db)
    .await
    .map(|_| ())
    .map_err(|e| e.into())
}
