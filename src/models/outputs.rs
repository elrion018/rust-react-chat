use serde::{Deserialize, Serialize};
use crate::models::user::User;
use crate::models::message::Message;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct JoinedOutput {
    pub user: User,
}


impl JoinedOutput {
    pub fn new(user: User) -> JoinedOutput {
        JoinedOutput { user }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct LeftOutput {
    user: User,
}

impl LeftOutput {
    pub fn new(user: User) -> LeftOutput {
        LeftOutput { user }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct SentOutput {
    user: User,
    message: Message,
}

impl SentOutput {
    pub fn new(user: User, message: Message) -> SentOutput {
        SentOutput { user, message }
    }
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct ErrorOutput {
    user: User,
}

impl ErrorOutput {
    pub fn new(user: User) -> ErrorOutput {
        ErrorOutput { user }
    }
}
