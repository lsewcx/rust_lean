use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    println!("{}", line.to_uppercase());
}
