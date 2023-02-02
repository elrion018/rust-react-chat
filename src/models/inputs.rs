use crate::models::user::User;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/**
 * Input is the type that is used to represent the input from the client.
 * It have three variants, Join and Send.
 */
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(tag = "type", content = "data")]
pub enum Input {
    #[serde(rename = "join")]
    Join(JoinInput),
    #[serde(rename = "send")]
    Send(SendInput),
    #[serde(rename = "leave")]
    Leave(LeaveInput),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JoinInput {
    pub user: User,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SendInput {
    pub text: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LeaveInput {
    pub user: User,
    pub created_at: String,
}

#[derive(Debug, Clone)]
pub struct InputParcel {
    pub client_id: Uuid,
    pub input: Input,
}
