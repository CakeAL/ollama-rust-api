use std::pin::Pin;

use serde::de::DeserializeOwned;
use tokio_stream::{Stream, StreamExt};

use crate::{
    Ollama,
    model::{
        chat::{ChatRequestParameters, ChatResponse},
        generate::{GenerateRequestParameters, GenerateResponse},
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
}
