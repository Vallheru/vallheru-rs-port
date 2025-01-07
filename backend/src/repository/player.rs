use crate::web::Error;

pub async fn get_user_by_email_and_password(
    db: &sqlx::PgPool,
    email: &str,
    pass: &str,
) -> crate::web::Result<vallheru::model::Player> {
    let player =
        sqlx::query_as::<_, vallheru::model::Player>(r"SELECT * FROM player WHERE email=$1")
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

