
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::models::user::User;
#[derive(Debug)]
pub struct Message {
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