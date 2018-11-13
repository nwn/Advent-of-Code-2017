use std::io;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let num_valid = lines.filter(|line| {
        let mut words: HashSet<&str> = HashSet::new();

        // Try inserting all nonempty words into a set
        let line = line.iter().next().unwrap();
        line.split_whitespace().map(|word| {
            word.is_empty() || words.insert(word)
        }).all(|b| b)
    }).count();
    println!("{}", num_valid);
}
