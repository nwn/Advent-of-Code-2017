use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_line(&mut input).expect("Failed to read input");
    input.pop(); // Remove trailing newline

    let lengths = input.split(',').map(|len| len.parse::<usize>().unwrap());

    const MARKS: usize = 256;
    let mut marks: Vec<u32> = (0..MARKS as u32).collect();
    let mut skip = 0;
    let mut pos = 0;
    for len in lengths {
        for i in 0..(len / 2) {
            marks[..].swap((pos + i) % MARKS, (pos + len - i - 1) % MARKS);
        }

        pos += len + skip;
        skip += 1;
    }
    println!("{}", marks[0] * marks[1]);
}
