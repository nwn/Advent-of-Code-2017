use std::io;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut left_refs: HashSet<String> = HashSet::new();
    let mut right_refs: HashSet<String> = HashSet::new();

    lines.for_each(|line| {
        let line = line.iter().next().unwrap();
        let mut words = line.split_whitespace();

        // Read parent name
        left_refs.insert(String::from(words.next().unwrap()));

        // Skip weight
        words.next();

        // Skip arrow if present
        if words.next().is_some() {
            // Read children
            for word in words {
                let word = word.split_terminator(',').next().unwrap();
                right_refs.insert(String::from(word));
            }
        }
    });

    let root = left_refs.difference(&right_refs).next().unwrap();
    println!("{}", root);
}
