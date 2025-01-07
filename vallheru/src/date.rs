pub type DateTime = chrono::DateTime<chrono::Utc>;

type Result<T> = std::result::Result<T, chrono::ParseError>;

pub fn from_string(date: &str) -> Result<DateTime> {
    Ok(chrono::NaiveDateTime::parse_from_str(date, "%Y-%m-%d %H:%M:%S%.f")?.and_utc())
}

pub mod date_serializer {
    use super::DateTime;
    use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S: Serializer>(dt: &DateTime, serializer: S) -> Result<S::Ok, S::Error> {
        dt.to_rfc3339().serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<DateTime, D::Error> {
        let time: String = Deserialize::deserialize(deserializer)?;

        chrono::DateTime::parse_from_rfc3339(&time)
            .map_err(|_| Error::custom("invalid date"))
            .map(|res| res.to_utc())
    }
}

pub mod option_date_serializer {
    use super::DateTime;
    use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S: Serializer>(
        dt: &Option<DateTime>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match dt {
            None => "".serialize(serializer),
            Some(dt) => dt.to_rfc3339().serialize(serializer),
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<DateTime>, D::Error> {
        let time: String = Deserialize::deserialize(deserializer)?;
        if time.is_empty() {
            return Ok(None);
        }

        chrono::DateTime::parse_from_rfc3339(&time)
            .map_err(|_| Error::custom("invalid date"))
            .map(|res| Some(res.to_utc()))
    }
}
