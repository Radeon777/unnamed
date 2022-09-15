use std::fmt;

use serde::{
    de::{self, Visitor},
    Deserialize, Serialize, Serializer,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Snowflake(u64);

impl Snowflake {
    pub fn timestamp(&self) -> u64 {
        (self.0 >> 22) + 1420070400000
    }
    pub fn internal_worker_identifier(&self) -> u8 {
        ((self.0 & 0x3E0000) >> 17) as u8
    }
    pub fn internal_process_identifier(&self) -> u8 {
        ((self.0 & 0x1F000) >> 12) as u8
    }
    pub fn increment(&self) -> u16 {
        (self.0 & 0xFFF) as u16
    }
}
impl Serialize for Snowflake {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for Snowflake {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct SnowflakeVisitior;
        impl<'a> Visitor<'a> for SnowflakeVisitior {
            type Value = Snowflake;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "snowflake must be encoded with string")
            }
            fn visit_str<E>(self, string: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Snowflake(
                    string.parse().map_err(|_| E::custom("invalid number"))?,
                ))
            }
        }
        deserializer.deserialize_str(SnowflakeVisitior)
    }
}
