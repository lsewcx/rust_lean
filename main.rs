use std::io::{self, BufRead};


fn main() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines();
    let w: i64 = iter.next().unwrap().unwrap().trim().parse().unwrap();
    let h: i64 = iter.next().unwrap().unwrap().trim().parse().unwrap();
    println!("{}", w * h);
}
