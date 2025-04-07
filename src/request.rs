use std::pin::Pin;

use tokio_stream::{Stream, StreamExt};

use crate::{
    Ollama,
    model::generate::{GenerateRequestParameters, GenerateResponse},
};

pub(crate) type OllamaResponseStream =
    Pin<Box<dyn Stream<Item = Result<GenerateResponse, ()>> + Send>>;

pub(crate) async fn generate(
    ollama: &Ollama,
    para: &GenerateRequestParameters,
) -> Result<OllamaResponseStream, reqwest::Error> {
    let url = format!("{}api/generate", ollama.host());
    let body = serde_json::json!(para);
    let resp = ollama
        .client
        .post(&url)
        .body(body.to_string())
        .send()
        .await?;
    let stream = Box::new(resp.bytes_stream().map(|res| match res {
        Ok(bytes) => {
            let res = serde_json::from_slice::<GenerateResponse>(&bytes);
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

#[cfg(test)]
mod tests {
    use crate::{Ollama, model::generate::GenerateRequestParameters, request::generate};
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
}
