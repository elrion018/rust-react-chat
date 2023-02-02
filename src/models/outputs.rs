use crate::models::message::Message;
use crate::models::user::User;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(tag = "type", content = "data")]
pub enum Output {
    #[serde(rename = "joined")]
    Joined(JoinedOutput),
    #[serde(rename = "left")]
    Left(LeftOutput),
    #[serde(rename = "sent")]
    Sent(SentOutput),
    #[serde(rename = "error")]
    Error(ErrorOutput),
    #[serde(rename = "alive")]
    Alive,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct JoinedOutput {
    pub user: User,
}

impl JoinedOutput {
    pub fn new(user: User) -> JoinedOutput {
        JoinedOutput { user }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct LeftOutput {
    user: User,
}

impl LeftOutput {
    pub fn new(user: User) -> LeftOutput {
        LeftOutput { user }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SentOutput {
    user: User,
    message: Message,
}

impl SentOutput {
    pub fn new(user: User, message: Message) -> SentOutput {
        SentOutput { user, message }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(tag = "code")]
pub enum ErrorOutput {
    #[serde(rename = "invalid-name")]
    InvalidName,
    #[serde(rename = "invalid-message")]
    InvalidMessage,
    #[serde(rename = "failed-to-join")]
    FailedToJoin,
    #[serde(rename = "failed-to-send")]
    FailedToSend,
}

#[derive(Debug, Clone)]
pub struct OutputParcel {
    pub client_id: Uuid,
    pub output: Output,
}
