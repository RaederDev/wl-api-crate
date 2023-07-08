pub mod date_format {
    use chrono::{DateTime, FixedOffset};
    use serde::{self, Deserialize, Deserializer};

    const FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.f%z";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<FixedOffset>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        DateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

pub mod optional_date_format {
    use chrono::{DateTime, FixedOffset};
    use serde::{self, Deserialize, Deserializer};

    const FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.f%z";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<FixedOffset>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt_str: Option<String> = Option::deserialize(deserializer)?;
        match opt_str {
            Some(s) => DateTime::parse_from_str(&s, FORMAT)
                .map(Some)
                .map_err(serde::de::Error::custom),
            None => Ok(None),
        }
    }
}
