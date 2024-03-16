Groq Rust Library
=================

A Rust library for interacting with the [Groq API](https://console.groq.com/docs/quickstart). This library provides a convenient Rust interface for using the LLM model from the Groq API. The model is configurable and not limited to mixtral-8x7B.

Note that the majority of this codebase is provided by Groq and Mistral with a little human intervention.

Usage
-----

To use the library in your project, add the following to your `Cargo.toml`:
```toml
[dependencies]
groq-rust = "0.1.0"
```
Then, in your code:
```rust
use groq_rust::{GroqClient, GroqError};

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

Alright, cobbers, here's the go. We’re setting off on a ripper of an adventure where we’re not just using Groq; we’re letting Groq run the whole shebang. It’s like asking the kangaroos to manage the zoo – bold, a bit cheeky, but downright brilliant. We’re all in on Groq, making it not just part of the team but the whole team. Why? Because if Groq’s as quick as a possum up a gum tree, imagine what happens when it starts calling the shots. It’s like creating a boomerang that throws itself – a bit of a head-scratcher but epic once you see it in action. So, let’s make Groq create everything for Groq, turning this into a full-blown Groq-a-thon. It’s our chance to kick back, maybe crack a cold one, and watch Groq do its thing, pondering life’s great mysteries, like why we park on driveways and drive on parkways, or in our case, why 'budgie smugglers' is a perfectly acceptable term for swimwear.
