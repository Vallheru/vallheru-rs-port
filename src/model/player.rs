use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::str::FromStr;

#[derive(Default, sqlx::Type, Serialize, Deserialize, Clone, Debug)]
#[sqlx(type_name = "PLAYER_RANK", rename_all = "lowercase")]
pub enum PlayerRank {
    Admin,
    Staff,
    #[default]
    Member,
}

#[derive(Default, sqlx::Type, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[sqlx(type_name = "PLAYER_GENDER", rename_all = "lowercase")]
pub enum PlayerGender {
    #[sqlx(rename = "")]
    #[default]
    Unknown,
    Male,
    Female,
}

#[derive(Default, sqlx::Type, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[sqlx(type_name = "PLAYER_RELIGION", rename_all = "lowercase")]
pub enum PlayerReligion {
    #[sqlx(rename = "")]
    #[default]
    Atheist,
    Illuminati,
    Karserth,
    Anariel,
    Heluvald,
    Tartus,
    Oregarl,
    Daeraell,
    #[sqlx(rename = "teathe-di")]
    TeatheDi,
    Thindil,
}

impl FromStr for PlayerReligion {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" | "atheist" => Ok(Self::Atheist),
            "illuminati" => Ok(Self::Illuminati),
            "karserth" => Ok(Self::Karserth),
            "anariel" => Ok(Self::Anariel),
            "heluvald" => Ok(Self::Heluvald),
            "tartus" => Ok(Self::Tartus),
            "oregarl" => Ok(Self::Oregarl),
            "daeraell" => Ok(Self::Daeraell),
            "teathe-di" | "teathedi" => Ok(Self::TeatheDi),
            "thindil" => Ok(Self::Thindil),
            _ => Err(()),
        }
    }
}

//   <ul>
//     <li>- <a href="/game/player-statistics/select-religion?select=illuminati">Illuminati</a></li>
//     <li>- <a href="/game/player-statistics/select-religion?select=karserth">Karserth</a></li>
//     <li>- <a href="/game/player-statistics/select-religion?select=anariel">Anariel</a></li>
//     <li>- <a href="/game/player-statistics/select-religion?select=heluvald">Heluvald</a></li>
//     <li>- <a href="/game/player-statistics/select-religion?select=tartus">Tartus</a></li>
//     <li>- <a href="/game/player-statistics/select-religion?select=oregarl">Oregarl</a></li>
//     <li>- <a href="/game/player-statistics/select-religion?select=daeraell">Daeraell</a></li>
//     <li>- <a href="/game/player-statistics/select-religion?select=teathe-di">TeatheDi</a></li>
//     <li>- <a href="/game/player-statistics/select-religion?select=thindil">Thindil</a></li>
//     <li>- <a href="/game/player-statistics">Remain an Atheist</a></li>
#[derive(Default, sqlx::Type, Serialize, Deserialize, Clone, Debug)]
#[sqlx(type_name = "PLAYER_RACE", rename_all = "lowercase")]
pub enum PlayerRace {
    #[sqlx(rename = "")]
    #[default]
    Unknown,
    Human,
    Elf,
    Dwarf,
    Hobbit,
    Reptilian,
    Gnome,
}

#[derive(Default, sqlx::Type, Serialize, Deserialize, Clone, Debug)]
#[sqlx(type_name = "PLAYER_CLASS", rename_all = "lowercase")]
pub enum PlayerClass {
    #[sqlx(rename = "")]
    #[default]
    Unknown,
    Warrior,
    Mage,
    Craftsman,
    Barbarian,
    Thief,
}

#[derive(Default, FromRow, Serialize, Deserialize, Clone, Debug)]
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
    pub ip: Option<String>,
    pub protection: i32,
    pub gender: PlayerGender,

    pub last_killed: Option<i32>,
    pub last_killed_by: Option<i32>,

    pub fights_won: i32,
    pub fights_lost: i32,

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

    pub ap: i32,
    pub race: PlayerRace,
    pub profession: PlayerClass,
    pub religion: PlayerReligion,
    pub agility: BigDecimal,
    pub strength: BigDecimal,
    pub intelligence: BigDecimal,
    pub wisdom: BigDecimal,
    pub speed: BigDecimal,
    pub stamina: BigDecimal,
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
