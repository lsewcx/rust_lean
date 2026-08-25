use std::io::{self, BufRead};

fn count(s: &str) -> usize {

    return s.len();
}

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    println!("{}", count(&line));
}
