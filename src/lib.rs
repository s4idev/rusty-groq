use thiserror::Error;
use reqwest::Client;
use serde_json::json;

#[derive(Error, Debug)]
pub enum GroqError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("API error: {0}")]
    Api(String),
}


pub struct GroqClient {
    base_url: String,
    api_key: String,
    client: Client,
}

impl GroqClient {
    pub fn new(base_url: String, api_key: String) -> Self {
        GroqClient {
            base_url,
            api_key,
            client: Client::new(),
        }
    }

    pub async fn send_request(&self, message: &str, model: &str) -> Result<String, GroqError> {
        let url = format!("{}/chat/completions", self.base_url);
        let json_data = json!({
            "messages": [{"role": "user", "content": message}],
            "model": model,
        });
    
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&json_data)
            .send()
            .await?
            .text()
            .await?;
    
        Ok(response)
    }
}
