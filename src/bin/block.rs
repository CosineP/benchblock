use benchblock::FETCH;
use reqwest::blocking::get;
use std::env;
use std::thread;

// at 1000 we get out of files
// at 500 we get device busy
const THREADS: usize = 200;

fn fetch_it() -> Result<(), reqwest::Error> {
    get(FETCH)?.bytes()?;
    Ok(())
}

// cargo run <n: threads = fetches>
fn main() -> Result<(), reqwest::Error> {
    let mut args = env::args();
    args.next(); // self
    let n: usize = args.next().expect("cargo run <n>").parse().expect("n is number");
    let mut i = 0;
    while i < n {
        let mut joins = vec![];
        for _ in 0..THREADS {
            if !(i < n) {
                break;
            }
            joins.push(thread::spawn(fetch_it));
            i += 1;
        }
        for join in joins {
            join.join().expect("thread panicked")?;
        }
    }
    Ok(())
}

