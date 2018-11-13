use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let sum = lines.map(|line| {
        let line = line.unwrap();
        let values: Vec<u32> = line.split_whitespace()
            .map(|str| str.parse().expect("Failed to read integer"))
            .collect();
        if values.is_empty() {
            return 0;
        }

        let min = values.iter().min().unwrap();
        let max = values.iter().max().unwrap();
        return max - min;
    }).sum::<u32>();
    println!("{}", sum);
}
