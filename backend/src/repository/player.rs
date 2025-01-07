use crate::web::Error;

use vallheru::model::Player;

pub async fn alter_last_login(db: &sqlx::PgPool, player_id: i32) -> sqlx::Result<()> {
    sqlx::query(r"UPDATE player SET last_login=NOW() WHERE id=$1")
        .bind(player_id)
        .execute(db)
        .await
        .map(|_| ())
}

pub async fn get_user_by_email_and_password(
    db: &sqlx::PgPool,
    email: &str,
    pass: &str,
) -> crate::web::Result<Player> {
    let player = sqlx::query_as::<_, Player>(r"SELECT * FROM player WHERE email=$1")
        .bind(email)
        .fetch_one(db)
        .await;

    match player {
        Ok(player) => {
            if vallheru::password_utils::is_valid_password(pass, &player.password) {
                Ok(player)
            } else {
                Err(Error::Unauthorized(Some(String::from(
                    "invalid password or user not found",
                ))))
            }
        }
        Err(e) => match e {
            sqlx::Error::RowNotFound => Err(Error::Unauthorized(Some(String::from(
                "invalid password or user not found",
            )))),
            _ => Err(e.into()),
        },
    }
}

pub async fn get_player_by_token(db: &sqlx::PgPool, token: &str) -> Option<Player> {
    sqlx::query_as::<_, Player>(
        r"SELECT * 
            FROM player AS p 
            LEFT JOIN token AS t ON p.id=t.player_id 
            WHERE t.token=$1 AND valid_until>NOW()+'5 minutes'::INTERVAL",
    )
    .bind(token)
    .fetch_one(db)
    .await
    .map(Some)
    .unwrap_or(None)
}
