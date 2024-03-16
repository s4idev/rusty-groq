// Imports necessary for the benchmark and async handling
use criterion::{criterion_group, criterion_main, Criterion};
use std::time::Instant;
use tokio::runtime::Runtime;

use groq::*;

fn benchmark_groq_client(c: &mut Criterion) {
    let mut group = c.benchmark_group("GroqClient");

    group.bench_function("send_request", |b| {
        // Create a single runtime for all benchmarks to minimize overhead
        let rt = Runtime::new().expect("Failed to create a Tokio runtime");

        b.iter(|| {
            // Block on the async code using the runtime
            rt.block_on(async {
                let base_url = "https://api.groq.com/openai/v1".to_string();
                let api_key = std::env::var("GROQ_API_KEY")
                    .expect("GROQ_API_KEY must be set for benchmarking");
                let groq_client = GroqClient::new(base_url, api_key);

                let message = "Explain the importance of low latency LLMs";
                let model = "mixtral-8x7b-32768";

                let start = Instant::now();
                // Handle errors more gracefully
                if let Err(e) = groq_client.send_request(message, model).await {
                    eprintln!("Failed to send request: {}", e);
                }
                let duration = start.elapsed();
                println!("Time taken: {:?}", duration);
            })
        });
    });

    group.finish();
}

// Criterion's main function to run the benchmarks
criterion_group!(benches, benchmark_groq_client);
criterion_main!(benches);
