use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let n: i64 = stdin.lock().lines().next().unwrap().unwrap().trim().parse().unwrap();
    // Loop and sum, then print.
    let _ = n;
    let mut num:i64 = 0;
    for i in 1..=n{
        num +=i;
    }
    println!("{}", num);
}
