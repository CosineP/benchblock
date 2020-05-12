use benchblock::TO_READ;
use std::env;
use std::fs::File;
use std::io::{Result, Read};
use std::thread;

fn read_it() -> Result<()> {
    // to avoid O(n) CPU read_to_string due to UTF-8
    let mut file = File::open(TO_READ)?;
    let mut contents = vec![];
    file.read_to_end(&mut contents)?;
    // adding this to avoid getting it all optimized out
    println!("{}", contents.len());
    Ok(())
}

// cargo run <n: threads = fetches>
fn main() -> Result<()> {
    let mut args = env::args();
    args.next(); // self
    let n: usize = args.next().expect("cargo run <n>").parse().expect("n is number");
    let mut joins = vec![];
    for _ in 0..n {
        joins.push(thread::spawn(read_it));
    }
    for join in joins {
        join.join().expect("thread panicked")?;
    }
    Ok(())
}

