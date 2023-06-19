use serde::{self, Deserialize, Deserializer, Serializer};
use sqlx::types::chrono::NaiveDateTime;

const FORMAT: &str = "%Y-%m-%d %H:%M:%S%.f";

pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
{
  let s = format!("{}", date.format(FORMAT));
  serializer.serialize_str(&s)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
  where
    D: Deserializer<'de>,
{
  let s = String::deserialize(deserializer)?;
  NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
}
