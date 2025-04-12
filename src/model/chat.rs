use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::parameter::Parameter;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageRole {
    System,
    User,
    Assistant,
    Tool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Message {
    pub role: MessageRole,
    pub content: String,
    pub images: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ChatRequestParameters {
    pub model: String,
    pub messages: Vec<Message>,
    pub options: Option<Parameter>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ChatResponse {
    pub model: String,
    pub created_at: DateTime<Utc>,
    pub message: Message,
    pub done: bool,

    pub total_duration: Option<i64>, // 时间为纳秒
    pub load_duration: Option<i64>,
    pub prompt_eval_count: Option<i64>,
    pub prompt_eval_duration: Option<i64>,
    pub eval_count: Option<i64>,
    pub eval_duration: Option<i64>,
    pub context: Option<Vec<i64>>,
}
