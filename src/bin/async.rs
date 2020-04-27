use benchblock::FETCH;
use reqwest::get;
use std::env;

async fn fetch_it() -> Result<(), reqwest::Error> {
    // don't use client, HTTP keep-alive is not fair to blocking version
    get(FETCH).await?.bytes().await?;
    Ok(())
}

// cargo run <n: fetches>
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


