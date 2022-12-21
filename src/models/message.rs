
use uuid::Uuid;
use crate::models::user::User;

pub struct Message {
    pub id: Uuid,
    pub user: User,
    pub text: String,   
}