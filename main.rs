use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let nums: Vec<i64> = line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    // Filter to evens, square, sum, and print.
    let _ = nums;
    let answer: i64 = nums.iter()
    .filter(|n| *n % 2 == 0)
    .map(|n| n * n)
    .sum();
    println!("{}", answer)
}



