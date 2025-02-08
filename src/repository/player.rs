use crate::web::Error;

use crate::model::{self, Player};

pub async fn alter_last_login_and_login_count(
    db: &sqlx::PgPool,
    player_id: i32,
) -> sqlx::Result<()> {
    sqlx::query(r"UPDATE player SET last_login=NOW(), login_count=login_count+1 WHERE id=$1")
        .bind(player_id)
        .execute(db)
        .await
        .map(|_| ())
}

pub async fn get_player_by_email(
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
            if crate::util::password::is_valid_password(pass, &player.password) {
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

pub async fn disable_player_protection(db: &sqlx::PgPool, player_id: i32) -> sqlx::Result<()> {
    sqlx::query(
        r"UPDATE player SET protection=0 WHERE id=$1",
    )
    .bind(player_id)
    .execute(db)
    .await
    .map(|_| ())
}

pub async fn set_religion(db: &sqlx::PgPool, player_id: i32, religion: model::player::PlayerReligion) -> sqlx::Result<()> {
    sqlx::query(
        r"UPDATE player SET religion=$1 WHERE id=$2",
    )
    .bind(religion)
    .bind(player_id)
    .execute(db)
    .await
    .map(|_| ())
}