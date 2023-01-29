
/**
 * Uuid 타입이 Serialize, Deserialize를 구현하지 않아서
 * serde의 with를 사용하여 구현한다.
 */
use uuid::Uuid;
use serde::{Deserialize, Deserializer, Serializer};

pub fn serialize<S>(uuid: &Uuid, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&uuid.to_string())
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Uuid, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Uuid::parse_str(&s).map_err(serde::de::Error::custom)
}