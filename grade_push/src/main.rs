use clap::Parser;
use std::fs;

#[derive(Parser)]
struct Args {
    /// Path to results.json
    path: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let data = fs::read_to_string(&args.path)?;
    println!("Pushing grade results from {}", args.path);
    let client = reqwest::Client::new();
    let resp = client
        .post("http://lrs.example.com/statement")
        .body(data)
        .send()
        .await?;
    println!("LRS response: {}", resp.status());
    Ok(())
}
