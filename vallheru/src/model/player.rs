use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(sqlx::Type, Serialize, Deserialize, Clone, Debug)]
#[sqlx(type_name = "PLAYER_RANK", rename_all = "lowercase")]
pub enum PlayerRank {
    Admin,
    Member,
}

#[derive(FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct Player {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
    // #[serde(with = "crate::date::date_serializer")] // declaring
    pub created_at: crate::DateTime,
    // #[serde(with = "crate::date::option_date_serializer")] // declaring
    pub last_login: Option<crate::DateTime>,
    pub login_count: i32,
    pub rank: PlayerRank,
    pub last_page: String,

    pub level: i32,
    pub exp: i32,
    pub hp: i32,
    pub max_hp: i32,
    pub sp: i32,
    pub energy: i32,
    pub max_energy: i32,
    pub inc_energy: i32,

    pub gold: i32,
    pub bank: i32,
    pub mithrill: i32,
    pub vallars: i32,
}

// #[cfg(test)]
// mod test {
//     use serde_json::json;

//     use super::Player;

//     #[test]
//     fn serialization_with_none_last_login() {
//         let p = Player {
//             id: 10,
//             username: String::from("admin"),
//             password: String::from("pass"),
//             email: String::from("email@op.pl"),
//             created_at: crate::date::from_string("2025-01-06 20:40:19.441241").unwrap(),
//             last_login: None,
//         };
//         let p_json = json!(p);
//         let p_json = p_json.to_string();

//         let new_p: Player = serde_json::from_str(&p_json).unwrap();
//         let new_p_json = json!(new_p);
//         let new_p_json = new_p_json.to_string();

//         assert_eq!(p_json, new_p_json);
//     }

//     #[test]
//     fn serialization_full() {
//         let p = Player {
//             id: 10,
//             username: String::from("admin"),
//             password: String::from("pass"),
//             email: String::from("email@op.pl"),
//             created_at: crate::date::from_string("2025-01-06 20:40:19.441241").unwrap(),
//             last_login: Some(crate::date::from_string("2025-01-06 22:40:19.441241").unwrap()),
//         };
//         let p_json = json!(p);
//         let p_json = p_json.to_string();

//         let new_p: Player = serde_json::from_str(&p_json).unwrap();
//         let new_p_json = json!(new_p);
//         let new_p_json = new_p_json.to_string();

//         assert!(new_p.last_login.is_some());
//         assert_eq!(p_json, new_p_json);
//     }
// }
