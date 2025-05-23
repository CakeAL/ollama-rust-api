use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::parameter::Parameter;

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GenerateRequestParameters {
    pub model: String,
    pub prompt: Option<String>,
    pub options: Option<Parameter>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GenerateResponse {
    pub model: String,
    pub created_at: DateTime<Utc>,
    pub response: String,
    pub done: bool,

    pub total_duration: Option<i64>, // 时间为纳秒
    pub load_duration: Option<i64>,
    pub prompt_eval_count: Option<i64>,
    pub prompt_eval_duration: Option<i64>,
    pub eval_count: Option<i64>,
    pub eval_duration: Option<i64>,
    pub context: Option<Vec<i64>>,
}
