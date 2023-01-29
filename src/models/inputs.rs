use serde::{Deserialize, Serialize};


/**
 * Input is the type that is used to represent the input from the client.
 * It have three variants, Join and Send.
 */
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "data")]
pub enum Input{
    #[serde(rename = "join")]
    Join(JoinInput),
    #[serde(rename = "send")]
    Send(SendInput),
    #[serde(rename = "leave")]
    Leave(LeaveInput),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct JoinInput {
    pub name: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SendInput {
    pub text: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LeaveInput {
    pub name: String,
    pub created_at: String,
}


