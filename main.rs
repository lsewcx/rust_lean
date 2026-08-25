use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let mut seen: HashSet<&str> = HashSet::new();
    for w in line.split_whitespace() {
        seen.insert(w);
    }
    // Print the size of seen.
    let _ = seen;
    let mut result = HashSet::new();
    for w in line.split_whitespace() {
        result.insert(w);
    }
    println!("{}", result.len());
}
