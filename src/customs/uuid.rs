use serde::{Deserialize};
use serde::ser::Serializer;
use serde::de::Deserializer;
use uuid::Uuid;

pub fn serialize<S>(id: &Uuid, s: S) -> Result<S::Ok, S::Error>
    where S: Serializer
{
    s.serialize_str(&id.to_string())
}

pub fn deserialize<'de, D>(d: D) -> Result<Uuid, D::Error>
    where D: Deserializer<'de>
{
    let s = String::deserialize(d)?;
    Uuid::parse_str(&s).map_err(serde::de::Error::custom)
}