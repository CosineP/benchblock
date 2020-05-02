use benchblock::TO_READ;
use std::env;
use tokio::fs::read_to_string;
use tokio::io::Result;

async fn read_it() -> Result<()> {
    read_to_string(TO_READ).await?;
    Ok(())
}

// cargo run <n: fetches>
#[tokio::main]
async fn main() -> Result<()> {
    let mut args = env::args();
    args.next(); // self
    let n = args.next().expect("cargo run <n>").parse().expect("n is number");
    let mut futures = vec![];
    for _ in 0..n {
        futures.push(read_it());
    }
    for fut in futures {
        fut.await?;
    }
    Ok(())
}


