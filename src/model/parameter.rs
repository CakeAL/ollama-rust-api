use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Parameter {
    mirostat: Option<i32>,
    mirostat_eta: Option<f32>,
    mirostat_tau: Option<f32>,
    num_ctx: Option<i32>,
    repeat_last_n: Option<i32>,
    repeat_penalty: Option<f32>,
    temperature: Option<f32>,
    seed: Option<i32>,
    stop: Option<String>,
    num_predict: Option<i32>,
    top_k: Option<i32>,
    top_p: Option<f32>,
    min_p: Option<f32>,
}