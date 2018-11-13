use std::io;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let mut banks: Vec<u32> = line.split_whitespace()
        .map(|str| str.parse().expect("Failed to read integer"))
        .collect();
    let num_banks = banks.len();

    let mut previous_states: HashSet<Vec<u32>> = HashSet::new();
    while !previous_states.contains(&banks) {
        previous_states.insert(banks.clone());

        // Find bank to redistribute
        let max_index = banks.iter()
            .enumerate()
            .rev()
            .max_by_key(|&(_, val)| val)
            .unwrap().0;

        let redist_amount = banks[max_index];
        banks[max_index] = 0;
        for i in 1..num_banks + 1 {
            let addend = redist_amount / num_banks as u32 +
                         (i <= redist_amount as usize % num_banks) as u32;
            banks[(max_index + i) % num_banks] += addend;
        }
    }

    println!("{}", previous_states.len());
}
