use benchblock::TO_READ;
use std::env;
use std::fs::read_to_string;
use std::io::Result;
use std::thread;

fn read_it() -> Result<()> {
    read_to_string(TO_READ)?;
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

