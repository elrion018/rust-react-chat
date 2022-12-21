use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum Input{
    #[serde(rename = "join")]
    Join(JoinInput),
    #[serde(rename = "send")]
    Send(SendInput),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JoinInput {
    pub name: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendInput {
    pub text: String,
    pub created_at: String,
}