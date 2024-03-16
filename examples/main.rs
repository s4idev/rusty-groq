use groq::{GroqClient, GroqError};

#[tokio::main]
async fn main() -> Result<(), GroqError> {
    let base_url = "https://api.groq.com/openai/v1".to_string();
    let api_key = std::env::var("GROQ_API_KEY").expect("GROQ_API_KEY must be set");
    let groq_client = GroqClient::new(base_url, api_key);

    let message = "Explain the importance of low latency LLMs";
    let model = "mixtral-8x7b-32768";

    let response = groq_client.send_request(message, model).await?;

    println!("Response: {}", response);

    Ok(())
}