use uuid::Uuid;
use serde::{Deserialize, Serialize};
// use crate::customs::uuid_serde;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct User {
    // #[serde(with = "uuid_serde")]
    pub id: Uuid,
    pub name: String,
}

impl User {
    fn new(id: Uuid, name: &str) -> User {
        User { id, name: name.to_string() }
    }
}



