
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::models::user::User;
use crate::customs::uuid_serde;
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Message {
    #[serde(with = "uuid_serde")]
    pub id: Uuid,
    pub user: User,
    pub text: String,
    pub created_at: DateTime<Utc>,
}

impl Message {
    fn new(id: Uuid, user: User, text: &str, created_at: DateTime<Utc>) -> Message {
        Message { id, user, text: text.to_string(), created_at }
    }
}