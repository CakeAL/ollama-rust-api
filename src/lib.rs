pub mod model;
pub(crate) mod request;

use model::{
    chat::{ChatRequestParameters, ChatResponse},
    embedding::EmbedResponse,
    generate::{GenerateRequestParameters, GenerateResponse},
    models::ModelList,
};
use request::OllamaResponseStream;
use reqwest::{Client, Url};

pub struct Ollama {
    host: Url,
    client: Client,
}

impl Ollama {
    pub fn new(host: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            host: Url::parse(host)?,
            client: Client::new(),
        })
    }

    pub fn change_host(&mut self, host: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.host = Url::parse(host)?;
        Ok(())
    }

    pub fn host(&self) -> &str {
        self.host.as_str()
    }

    pub async fn check_online(&self) -> bool {
        if let Ok(resp) = self.client.get(self.host.clone()).send().await {
            if let Ok(text) = resp.text().await {
                return text.eq_ignore_ascii_case("Ollama is running");
            }
        }
        false
    }

    pub async fn generate(
        &self,
        para: &GenerateRequestParameters,
    ) -> Result<OllamaResponseStream<GenerateResponse>, reqwest::Error> {
        request::generate(self, para).await
    }

    pub async fn chat(
        &self,
        para: &ChatRequestParameters,
    ) -> Result<OllamaResponseStream<ChatResponse>, reqwest::Error> {
        request::chat(self, para).await
    }

    pub async fn tags(&self) -> Result<ModelList, reqwest::Error> {
        request::tags(self).await
    }

    pub async fn embedding(
        &self,
        para: &model::embedding::EmbedRequestParameters,
    ) -> Result<EmbedResponse, reqwest::Error> {
        request::embed(self, para).await
    }
}

impl Default for Ollama {
    fn default() -> Self {
        Self {
            host: Url::parse("http://localhost:11434").unwrap(),
            client: Client::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_check_online() {
        let ollama = Ollama::default();
        assert_eq!(true, ollama.check_online().await);
    }
}
