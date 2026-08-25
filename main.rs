use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let nums: Vec<i64> = line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    // Find and print the maximum.
    let _ = nums;
    let mut result=std::i64::MIN;
    for i in nums {
        if i > result {
            result = i;
        }
    }
    println!("{}", result);
}
