use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct User {
    pub id: Uuid,
    pub name: String,
}

impl User {
    fn new(id: Uuid, name: &str) -> User {
        User {
            id,
            name: name.to_string(),
        }
    }
}
