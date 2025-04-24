use std::pin::Pin;

use serde::de::DeserializeOwned;
use tokio_stream::{Stream, StreamExt};

use crate::{
    Ollama,
    model::{
        chat::{ChatRequestParameters, ChatResponse},
        embedding::{EmbedRequestParameters, EmbedResponse},
        generate::{GenerateRequestParameters, GenerateResponse},
        models::ModelList,
    },
};

pub(crate) type OllamaResponseStream<T> = Pin<Box<dyn Stream<Item = Result<T, ()>> + Send>>;

async fn send_request<T: DeserializeOwned>(
    client: &reqwest::Client,
    url: &str,
    body: &str,
) -> Result<OllamaResponseStream<T>, reqwest::Error> {
    let resp = client.post(url).body(body.to_string()).send().await?;
    let stream = Box::new(resp.bytes_stream().map(|res| match res {
        Ok(bytes) => {
            let res = serde_json::from_slice::<T>(&bytes);
            match res {
                Ok(res) => Ok(res),
                Err(e) => {
                    eprintln!("Failed to deserialize response: {}", e);
                    Err(())
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to read response: {}", e);
            Err(())
        }
    }));

    Ok(Pin::from(stream))
}

pub(crate) async fn generate(
    ollama: &Ollama,
    para: &GenerateRequestParameters,
) -> Result<OllamaResponseStream<GenerateResponse>, reqwest::Error> {
    let url = format!("{}api/generate", ollama.host());
    let body = serde_json::json!(para);
    send_request(&ollama.client, &url, &body.to_string()).await
}

pub(crate) async fn chat(
    ollama: &Ollama,
    para: &ChatRequestParameters,
) -> Result<OllamaResponseStream<ChatResponse>, reqwest::Error> {
    let url = format!("{}api/chat", ollama.host());
    let body = serde_json::json!(para);
    send_request(&ollama.client, &url, &body.to_string()).await
}

pub(crate) async fn tags(ollama: &Ollama) -> Result<ModelList, reqwest::Error> {
    let url = format!("{}api/tags", ollama.host());
    let resp = ollama.client.get(&url).send().await?;
    let body = resp.bytes().await?;
    let model_list = serde_json::from_slice(&body).unwrap_or(ModelList { models: Vec::new() });
    Ok(model_list)
}

pub(crate) async fn embed(
    ollama: &Ollama,
    para: &EmbedRequestParameters,
) -> Result<EmbedResponse, reqwest::Error> {
    let url = format!("{}api/embed", ollama.host());
    let body = serde_json::json!(para);
    let resp = ollama
        .client
        .post(&url)
        .body(body.to_string())
        .send()
        .await?;
    Ok(serde_json::from_slice(&resp.bytes().await?).unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        Ollama,
        model::{
            chat::{Message, MessageRole},
            generate::GenerateRequestParameters,
        },
    };
    use tokio_stream::StreamExt;

    #[tokio::test]
    async fn test_generate() {
        let ollama = Ollama::default();
        let para = GenerateRequestParameters {
            model: "qwen2.5:3b".to_string(),
            prompt: Some("你好。".to_string()),
            ..Default::default()
        };
        let mut stream = generate(&ollama, &para).await.unwrap();
        while let Some(Ok(res)) = stream.next().await {
            print!("{}", res.response);
            if res.done {
                println!("");
                dbg!(res);
            }
        }
    }

    #[tokio::test]
    async fn test_chat() {
        let ollama = Ollama::default();
        let para = ChatRequestParameters {
            model: "qwen2.5:3b".to_string(),
            messages: vec![Message {
                role: MessageRole::User,
                content: "你好。".to_string(),
                images: None,
            }],
            ..Default::default()
        };
        let mut stream = chat(&ollama, &para).await.unwrap();
        while let Some(Ok(res)) = stream.next().await {
            print!("{}", res.message.content);
            if res.done {
                println!("");
                dbg!(res);
            }
        }
    }

    #[tokio::test]
    async fn test_tags() {
        let ollama = Ollama::default();
        let model_list = tags(&ollama).await.unwrap();
        dbg!(model_list);
    }

    #[tokio::test]
    async fn test_embed() {
        let ollama = Ollama::default();
        let para = EmbedRequestParameters {
            model: "nomic-embed-text:137m-v1.5-fp16".into(),
            input: vec![
                "Why is the sky blue?".to_string(),
                "Why is the grass green?".to_string(),
            ],
        };
        let embedding = embed(&ollama, &para).await.unwrap();
        dbg!(embedding);
    }
}
