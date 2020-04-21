use reqwest::get;
use std::env;

const FETCH: &'static str = "https://steven-universe.fandom.com/wiki/Steven_Universe:_The_Movie/Transcript";

async fn fetch_it() -> Result<(), reqwest::Error> {
    // don't use client, HTTP keep-alive pooling is not fair to blocking version
    get(FETCH).await?.text().await?;
    Ok(())
}

// cargo run <n: threads = fetches>
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let mut args = env::args();
    args.next(); // self
    let n = args.next().expect("cargo run <n>").parse().expect("n is number");
    let mut futures = vec![];
    for _ in 0..n {
        futures.push(fetch_it());
    }
    for fut in futures {
        fut.await?;
    }
    Ok(())
}


