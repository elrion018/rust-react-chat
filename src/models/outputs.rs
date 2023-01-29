use serde::{Deserialize, Serialize};
use crate::models::user::User;
use crate::models::message::Message;

#[derive(Debug, Serialize, Deserialize)]
struct UserJoinedOutput {
    user: User,
}


impl UserJoinedOutput {
    pub fn new(user: User) -> UserJoinedOutput {
        UserJoinedOutput { user }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct UserLeftOutput {
    user: User,
}

impl UserLeftOutput {
    pub fn new(user: User) -> UserLeftOutput {
        UserLeftOutput { user }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct MessageSentOutput {
    user: User,
    message: Message,
}

impl MessageSentOutput {
    pub fn new(user: User, message: Message) -> MessageSentOutput {
        MessageSentOutput { user, message }
    }
}


#[derive(Debug, Serialize, Deserialize)]
struct ErrorOutput {
    user: User,
}

impl ErrorOutput {
    pub fn new(user: User) -> ErrorOutput {
        ErrorOutput { user }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Output {
    #[serde(rename = "user-joined")]
    UserJoined(UserJoinedOutput),
    #[serde(rename = "user-left")]
    UserLeft(UserLeftOutput),
    #[serde(rename = "message-sent")]
    MessageSent(MessageSentOutput),
    #[serde(rename = "error")]
    Error(ErrorOutput),
}