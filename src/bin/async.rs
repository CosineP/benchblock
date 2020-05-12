use benchblock::TO_READ;
use std::env;
use tokio::fs::File;
use tokio::io::{Result, AsyncReadExt};

async fn read_it() -> Result<()> {
    // to avoid O(n) CPU read_to_string due to UTF-8
    let mut file = File::open(TO_READ).await?;
    let mut contents = vec![];
    file.read_to_end(&mut contents).await?;
    // adding this to be fair to block
    println!("{}", contents.len());
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


