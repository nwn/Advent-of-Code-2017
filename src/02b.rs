use std::io;
use std::io::prelude::*;
use std::cmp::PartialOrd;

fn minmax<T: PartialOrd>(a: T, b: T) -> (T,T) {
    if a <= b {
        (a,b)
    } else {
        (b,a)
    }
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let sum = lines.map(|line| {
        let line = line.unwrap();
        let values: Vec<u32> = line.split_whitespace()
            .map(|str| str.parse().expect("Failed to read integer"))
            .collect();

        for i in 0..values.len() {
            for j in 0..i {
                let (b,a) = minmax(values[i], values[j]);
                if (b != 0) && (a % b == 0) {
                    return a / b;
                }
            }
        }
        panic!("Failed to find divisible pair");
    }).sum::<u32>();
    println!("{}", sum);
}
