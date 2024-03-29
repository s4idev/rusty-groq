Groq Rust Library
=================

A Rust library for interacting with the [Groq API](https://console.groq.com/docs/quickstart). This library provides a convenient Rust interface for using the LLM model from the Groq API. The model is configurable and not limited to mixtral-8x7B.

Note that the majority of this codebase is provided by Groq and Mistral with a little human intervention.

Usage
-----

To use the library in your project, add the following to your `Cargo.toml`:
```toml
[dependencies]
rusty-groq = "0.1.0"
```
Then, in your code:
```rust
use groq::{GroqClient, GroqError};

#[tokio::main]
async fn main() -> Result<(), GroqError> {
    let base_url = "https://api.groq.com/openai/v1".to_string();
    let api_key = std::env::var("GROQ_API_KEY").expect("GROQ_API_KEY must be set");
    let groq_client = GroqClient::new(base_url, api_key);

    let message = "Explain the importance of low latency LLMs";
    let model = "mixtral-8x7b-32768"; // replace with your desired model

    let response = groq_client.send_request(message, model).await?;

    println!("Response: {}", response);

    Ok(())
}
```
Note that you need to set the `GROQ_API_KEY` environment variable to your Groq API key.

Developing
---------

To run the library's tests, use `cargo test`.

To run the library's benchmarks, use `cargo bench`.

License
-------

Licensed under the MIT license.

Contribution
------------

Alright, here's the plan: we're putting Groq in charge, and it's going to create everything for Groq under the MIT license. That means contributors can use and adapt the work freely. It's like inventing a boomerang that throws itself – sounds crazy, but it's going to be epic! We'll kick back, enjoy the show, and ponder life's mysteries, like why a bathroom's called "the loo" when there's no loo in sight. Our goal: 100% Groq-made, and contributors should use the MIT license. Let's do this!