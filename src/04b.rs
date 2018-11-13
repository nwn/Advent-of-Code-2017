use std::io;
use std::io::prelude::*;
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let num_valid = lines.filter(|line| {
        let mut words: HashSet<String> = HashSet::new();

        // Try inserting all nonempty words into a set
        let line = line.iter().next().unwrap();
        line.split_whitespace().map(|word| {
            if word.is_empty() {
                false
            } else {
                // Sort word to collapse anagrams
                let mut sorted: Vec<char> = word.chars().collect();
                sorted.sort();
                let sorted = String::from_iter(sorted.iter());

                words.insert(sorted)
            }
        }).all(|b| b)
    }).count();
    println!("{}", num_valid);
}
