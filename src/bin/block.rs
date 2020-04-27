use benchblock::FETCH;
use reqwest::blocking::get;
use std::env;
use std::thread;

fn fetch_it() -> Result<(), reqwest::Error> {
    let res = get(FETCH)?;
    res.bytes()?;
    Ok(())
}

// cargo run <n: threads = fetches>
fn main() -> Result<(), reqwest::Error> {
    let mut args = env::args();
    args.next(); // self
    let n = args.next().expect("cargo run <n>").parse().expect("n is number");
    let mut joins = vec![];
    for _ in 0..n {
        joins.push(thread::spawn(fetch_it));
    }
    for join in joins {
        join.join().expect("thread panicked")?;
    }
    Ok(())
}

