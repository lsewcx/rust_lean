use std::io::{self, BufRead};

fn square(n: i32) -> i32 {
    // Return n * n.
    n * n
}

fn main() {
    let stdin = io::stdin();
    let n: i32 = stdin.lock().lines().next().unwrap().unwrap().trim().parse().unwrap();
    println!("{}", square(n));
}
