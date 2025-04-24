use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct EmbedRequestParameters {
    pub model: String,
    pub input: Vec<String>,
    // pub options: Option<Parameter>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct EmbedResponse {
    pub model: String,
    pub embeddings: Vec<Vec<f32>>,
    pub total_duration: i64,
    pub load_duration: i64,
    pub prompt_eval_count: i64,
}
