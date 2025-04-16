use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 这是模型信息
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ModelList {
    pub models: Vec<Model>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Model {
    pub name: String,
    pub model: Option<String>, // 这个大概是没用的
    pub modified_at: DateTime<Utc>,
    pub size: i64,
    pub digest: String,
    pub details: ModelDetails,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ModelDetails {
    pub format: String,
    pub family: String,
    pub families: Vec<String>,
    pub parameter_size: String,
    pub quantization_level: String,
}
